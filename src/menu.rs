use crate::consts::*;
use crate::game::*;
use crate::save::*;
use crate::shop::*;

use bevy::app::AppExit;
use bevy::prelude::*;

use super::{GameState, despawn_all};

enum MenuState {
    Menu,
    Credits,
    Reset,
}

#[derive(Component)]
struct Menu(MenuState);

#[derive(Component)]
struct MenuTag;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), menu_enter)
        .add_systems(OnExit(GameState::Menu), despawn_all::<MenuTag>)
        .add_systems(Update, menu_update.run_if(in_state(GameState::Menu)));
}

fn menu_enter(mut commands: Commands, game: Res<Game>) {
    // Background
    commands.spawn((Sprite::from_image(game.h_menu_bg.clone()), MenuTag));

    // Menu Text Placeholder
    commands.spawn((
        Text2d::new(get_menu_text(&game)),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Left),
        Menu(MenuState::Menu),
        MenuTag,
    ));

    // Music
    if !secret_is_unlocked(Secret::Arkanoid as usize, &game) {
        commands.spawn((
            AudioPlayer::new(game.music_main_theme.clone()),
            PlaybackSettings::LOOP,
            MenuTag,
        ));
    } else {
        commands.spawn((
            AudioPlayer::new(game.music_arkanoid.clone()),
            PlaybackSettings::LOOP,
            MenuTag,
        ));
    }
}

fn get_menu_text(game: &Game) -> String {
    if secret_is_unlocked(Secret::Credits as usize, game) {
        "1 - PLAY\n\n2 - SHOP\n\n3 - RESET PROGRESS\n\n4 - CREDITS\n\nESC - Exit".to_string()
    } else {
        "1 - PLAY\n\n2 - SHOP\n\n3 - RESET PROGRESS\n\nESC - Exit".to_string()
    }
}

fn menu_update(
    mut game_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    menu_comp: Single<(&mut Text2d, &mut Menu), With<MenuTag>>,
    bg_comp: Single<(&Sprite, &mut Visibility), With<MenuTag>>,
    mut exit: EventWriter<AppExit>,
    mut game: ResMut<Game>,
) {
    let (mut text, mut menu) = menu_comp.into_inner();
    let (_, mut visibility) = bg_comp.into_inner();

    match menu.0 {
        MenuState::Menu => {
            if input.just_pressed(KeyCode::Digit1) {
                // Start new game
                game.current_level = 0;
                game_state.set(GameState::Transition);
            } else if input.just_pressed(KeyCode::Digit2) {
                // Enter Shop
                game_state.set(GameState::Shop);
            } else if input.just_pressed(KeyCode::Digit3) {
                // Delete all progress
                menu.0 = MenuState::Reset;
                *text = Text2d::new(
                    "Delete all game progress \n\
		     and secrets? (Y/N)\n\n\
		     THE GAME WILL CLOSE"
                        .to_string(),
                );
            } else if input.just_pressed(KeyCode::Digit4)
                && secret_is_unlocked(Secret::Credits as usize, &game)
            {
                // Credits screen
                *visibility = Visibility::Hidden;
                menu.0 = MenuState::Credits;
                if game.levels_unlocked.iter().any(|s| s.is_empty()) {
                    *text = Text2d::new(format!(
                        "{CREDITS}\n\nPermanently unlock all portals to claim a phonetool icon.\n\n\
			 Press ESC to go BACK"
                    ));
                } else {
                    *text = Text2d::new(format!(
                        "{CREDITS}\n\nClaim a phonetool icon with these codes:\n\
			 {}\n{}\n\n\
			 Press ESC to go BACK",
                        game.levels_unlocked[0..4].join(" "),
                        game.levels_unlocked[4..NLEVELS].join(" ")
                    ));
                }
            } else if input.just_pressed(KeyCode::Escape) {
                exit.write(AppExit::Success);
            }
        }
        MenuState::Reset => {
            // Reset confirmation
            if input.just_pressed(KeyCode::KeyN) || input.just_pressed(KeyCode::Escape) {
                *text = Text2d::new(get_menu_text(&game));
                menu.0 = MenuState::Menu;
            } else if input.just_pressed(KeyCode::KeyY) {
                game_reset();
                *text = Text2d::new(get_menu_text(&game));
                exit.write(AppExit::Success);
            }
        }
        MenuState::Credits => {
            // Credits Screen
            if input.just_pressed(KeyCode::Escape) {
                *visibility = Visibility::Visible;
                *text = Text2d::new(get_menu_text(&game));
                menu.0 = MenuState::Menu;
            }
        }
    }
}
