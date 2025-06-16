use crate::consts::*;
use crate::game::*;
use crate::save::*;

use super::{GameState, despawn_all};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use ciphers::{Cipher, Vigenere};
use rand::distr::weighted::WeightedIndex;

#[derive(Component)]
struct ShopTag;

#[derive(Component)]
struct ShopCodeText;

#[derive(Component)]
struct ShopSecretsText;

#[derive(Resource)]
pub struct Shop {
    input: String,
}

pub fn secret_is_discovered(secret: usize, game: &Game) -> bool {
    !game.secrets_generated[secret].is_empty()
}

pub fn secret_is_unlocked(secret: usize, game: &Game) -> bool {
    !game.secrets_unlocked[secret].is_empty() && !game.secrets_generated[secret].is_empty()
}

pub fn level_is_unlocked(level: usize, game: &Game) -> bool {
    !game.levels_unlocked[level].is_empty()
}

pub fn shop_plugin(app: &mut App) {
    app.add_systems(Startup, shop_startup)
        .add_systems(OnEnter(GameState::Shop), shop_enter)
        .add_systems(OnExit(GameState::Shop), despawn_all::<ShopTag>)
        .add_systems(
            Update,
            (shop_keyboard_input, shop_update).run_if(in_state(GameState::Shop)),
        );
}

fn shop_startup(mut commands: Commands) {
    let shop = Shop {
        input: String::new(),
    };
    commands.insert_resource(shop);
    // for l in ["A", "B", "C", "D", "E", "F", "G", "H"] {
    //     let test_decoded : String = String::from("FELIPEX") + l;
    //     let test_encoded = encode(&test_decoded);
    //     println!(
    //         "CODE {} -> {} -> {}",
    //         test_decoded,
    //         test_encoded,
    //         decode(&test_encoded).unwrap()
    //     );
    // }
}

fn shop_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text2d::new("Enter Codes to Unlock Secrets"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_xyz(0., 255., 1.),
        ShopTag,
    ));
    commands.spawn((
        Text2d::new("_".repeat(CODE_LEN).to_string().as_str()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_xyz(0., 230., 1.),
        ShopCodeText,
        ShopTag,
    ));
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::Srgba(bevy::color::Srgba::WHITE)),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0., 0., 1.),
        ShopSecretsText,
        ShopTag,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("images/background_shop.png")),
        ShopTag,
    ));
}

fn shop_keyboard_input(
    mut game_state: ResMut<NextState<GameState>>,
    mut shop: ResMut<Shop>,
    mut event: EventReader<KeyboardInput>,
    text: Single<&mut Text2d, With<ShopCodeText>>,
) {
    let mut modified: bool = false;
    for ev in event.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Escape => {
                shop.input.clear();
                game_state.set(GameState::Menu);
                break;
            }
            Key::Backspace => {
                shop.input.pop();
                modified = true;
            }
            Key::Character(input) => {
                for c in input.chars() {
                    if c.is_ascii_alphabetic() && shop.input.len() < CODE_LEN {
                        shop.input.push(c);
                        modified = true;
                    }
                }
            }
            _ => {}
        }
    }
    if modified {
        shop.input = shop.input.to_uppercase();
        text.into_inner().0 = format!("{:_<CODE_LEN$}", shop.input);
    }
}

fn shop_update(
    mut shop: ResMut<Shop>,
    mut game: ResMut<Game>,
    mut code_text: Single<&mut Text2d, With<ShopCodeText>>,
    mut secrets_text: Single<&mut Text2d, (With<ShopSecretsText>, Without<ShopCodeText>)>,
) {
    // Add code keyed in by user
    if shop.input.len() == CODE_LEN {
        code_text.clear();
        match shop_code_add(&mut game, &shop.input) {
            Ok(_) => {
                code_text.push_str("_".repeat(CODE_LEN).to_string().as_str());
                secrets_text.clear();
                game_save(&game);
                shop_process_secrets(&mut game);
            }
            Err(e) => {
                code_text.push_str(e.as_str());
            }
        }
        shop.input.clear();
    }

    // Update screen message
    if secrets_text.is_empty() {
        let mut unlocked_txt: String = String::new();

        for (idx, secret) in SECRETS.iter().enumerate() {
            if secret_is_unlocked(idx, &game) {
                unlocked_txt.push_str(SECRETS[idx]);
                unlocked_txt.push_str(" (unlocked!)\n");
            } else if secret_is_discovered(idx, &game) {
                unlocked_txt.push_str(secret);
                unlocked_txt.push_str(" (locked)\n");
            }
        }
        if unlocked_txt.is_empty() {
            unlocked_txt.push_str("(None)\n");
        }

        let mut generated_txt: String = String::new();
        let separator: [&str; 4] = ["    ", "    ", "    ", "\n"];
        for (idx, code) in game.secrets_generated.iter().enumerate() {
            if !code.is_empty() {
                generated_txt.push_str(code);
                generated_txt.push_str(separator[idx % separator.len()]);
            }
        }
        if generated_txt.is_empty() {
            generated_txt.push_str("(None)\n");
        }

        secrets_text.push_str("SECRETS DISCOVERED:");
        secrets_text.push_str("\n(Use codes from other players to unlock them)\n\n");
        secrets_text.push_str(&unlocked_txt);
        secrets_text.push_str("\nSHARE THESE WITH OTHER PLAYERS:\n\n");
        generated_txt = generated_txt.trim_end().to_string();
        secrets_text.push_str(&generated_txt);
        secrets_text.push_str("\n(Press ESC to return to menu)");
    }
}

pub fn shop_code_add(game: &mut Game, code: &str) -> Result<usize, String> {
    let secret = decode(code).unwrap_or_default();
    if code.len() == CODE_LEN
        && secret.len() == CODE_LEN - 1
        && secret.as_bytes()[USERNAME_LEN + 1] - b'A' < NLEVELS as u8
    {
        let idx: usize = (secret.as_bytes()[USERNAME_LEN + 1] - b'A') as usize;
        if secret[..game.username.len()] == game.username {
            match secret.as_bytes()[USERNAME_LEN] {
                b'X' => {
                    game.secrets_generated[idx] = code.to_string();
                    return Ok(idx);
                }
                b'L' => {
                    game.levels_unlocked[idx] = code.to_string();
                    return Ok(idx);
                }
                _ => (),
            }
        } else if secret.as_bytes()[USERNAME_LEN] == b'X' {
            if !secret_is_discovered(idx, game) {
                return Err("Secret not discovered yet".to_string());
            } else if secret_is_unlocked(idx, game) {
                return Err("Duplicated".to_string());
            }
            game.secrets_unlocked[idx] = code.to_string();
            return Ok(idx);
        }
    }
    Err("Incorrect".to_string())
}

pub fn shop_process_secrets(game: &mut Game) {
    let mut barrel_weights = BARREL_WEIGHTS.to_vec();
    if secret_is_unlocked(Secret::Gun as usize, game) {
        barrel_weights[1] = 2;
    }
    if secret_is_unlocked(Secret::Magnet as usize, game) {
        barrel_weights[3] = 4;
    }
    if secret_is_unlocked(Secret::Multiball as usize, game) {
        barrel_weights[4] = 4;
    }
    if secret_is_unlocked(Secret::Gun as usize, game)
        && secret_is_unlocked(Secret::Magnet as usize, game)
        && secret_is_unlocked(Secret::Multiball as usize, game)
    {
        barrel_weights[9] = 2; // Extra Time Barrel
    }
    game.barrel_dist = WeightedIndex::new(barrel_weights).unwrap();

    if secret_is_unlocked(Secret::XRay as usize, game) {
        game.h_bricks[14] = game.h_bricks[10].clone();
    }
}

pub fn shop_code_generate_new(game: &mut Game, t: char, idx: u8) {
    let secret_idx_letter = (idx + b'A') as char;
    let decoded: String = format!("{}{}{}", game.username, t, secret_idx_letter);
    let encoded: String = encode(&decoded);
    let _ = shop_code_add(game, &encoded);
    game_save(game);
}

fn calculate_crc(input: &str) -> u8 {
    let mut crc: u8 = 0;
    for c in input.chars() {
        crc = crc.wrapping_add(c as u8);
        crc <<= 1;
    }
    // Convert CRC to uppercase letter
    b'A' + (crc % 26)
}

fn encode(s: &str) -> String {
    let crc: u8 = calculate_crc(s);
    // Compute Key for encryption
    let key_rotation: usize = (crc as usize) % CIPHER_KEY.len();
    let mut key = String::with_capacity(CIPHER_KEY.len());
    key.push_str(&CIPHER_KEY[key_rotation..]); // First part: from rotation to end
    key.push_str(&CIPHER_KEY[..key_rotation]); // Second part: from start to rotation
    // Encode
    let vigenere = Vigenere::new(&key);
    let mut encoded: String = vigenere.encipher(s).unwrap();
    // Add crc to the end of the encoded string
    encoded.push(crc as char);
    encoded
}

fn decode(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err("Attempting to decode empty string".to_string());
    }
    // Extract crc from the end
    let expected_crc: u8 = s.as_bytes()[s.len() - 1];
    let s: &str = &s[..s.len() - 1];
    // Compute key for decryption
    let key_rotation: usize = (expected_crc as usize) % CIPHER_KEY.len();
    let mut key = String::with_capacity(CIPHER_KEY.len());
    key.push_str(&CIPHER_KEY[key_rotation..]); // First part: from rotation to end
    key.push_str(&CIPHER_KEY[..key_rotation]); // Second part: from start to rotation
    // Decode
    let vigenere = Vigenere::new(&key);
    let decoded: String = vigenere.decipher(s).unwrap();
    // Check crc
    if calculate_crc(&decoded) == expected_crc {
        Ok(decoded)
    } else {
        Err("CRC Failed".to_string())
    }
}
