use super::GameState;
use crate::consts::*;
use crate::game::*;
use crate::shop::*;
use rand::seq::*;
use crate::*;

use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Transition {
    pub timer: Timer,
    pub idx: usize,
    pub texts: Vec<String>,
    pub colors: Vec<Color>,
    pub end_game: bool,
}

pub fn transition_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Transition), transition_enter)
        .add_systems(OnExit(GameState::Transition), despawn_all::<Transition>)
        .add_systems(
            Update,
            transition_update.run_if(in_state(GameState::Transition)),
        );
}

// When entering this function, game.current_level is set to the
// level that we are entering, not the one we are leaving
pub fn transition_enter(mut commands: Commands, mut game: ResMut<Game>) {
    let mut texts: Vec<String> = Vec::new();
    let mut colors: Vec<Color> = Vec::new();
    let mut end_game = false;

    // Check if game over
    if game.nlives == 0 {
        commands.spawn((
            Text2d::new("Game Over"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 20.,
                ..default()
            },
            Transform::from_xyz(0., 0., LAYER_BANNER),
            TextLayout::new_with_justify(JustifyText::Center),
            Transition {
                idx: 0,
                timer: Timer::new(Duration::from_secs(TRANSITION_BANNER_SECS), TimerMode::Once),
                texts: vec!["Game Over".to_string()],
                colors: vec![Color::WHITE],
                end_game: true,
            },
        ));

        // Reset for new game
        game.nlives = NLIVES;
        return;
    }

    // Unblock secrets from recently completed level
    if game.nlives > 0 && game.current_level > 0 && game.seconds_left > 0. {
        let prev_level = game.current_level - 1;
        if game.secrets_generated[prev_level].is_empty() {
            shop_code_generate_new(&mut game, 'X', prev_level as u8);
            texts.push("You Discovered a Secret\n(check the Shop)".to_string());
            colors.push(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0)));
        } else if game.levels_unlocked[prev_level].is_empty() {
            shop_code_generate_new(&mut game, 'L', prev_level as u8);
            texts.push("You permanently unlocked the\nportal in this level...".to_string());
            colors.push(Color::Srgba(Srgba::new(1.0, 1.0, 1.0, 1.0)));
        };
    }

    // See if next level is blocked by a secret
    if game.current_level == 3 && !secret_is_unlocked(Secret::Hell as usize, &game) {
        texts.push(
            "The gates of Hell are sealed.\n\
	     Only those who possess the ancient knowledge may enter."
                .to_string(),
        );
        colors.push(Color::WHITE);
        end_game = true;
    } else if game.current_level == NLEVELS && !secret_is_unlocked(Secret::Credits as usize, &game)
    {
        texts.push(
            "You completed the Game but tales of those \n\
	     who crafted this journey remain concealed.\n\
	     Only the most dedicated players shall \n\
	     witness these sacred scrolls."
                .to_string(),
        );
        colors.push(Color::WHITE);
        end_game = true;
    } else if game.current_level == NLEVELS && !game.secrets_unlocked[7].is_empty() {
        texts.push(
            "You completed the game! Congratulations!\n\
	     Make sure to visit the CREDITs screen from the menu!\n"
                .to_string(),
        );
        colors.push(Color::WHITE);
        end_game = true;
    } else {
        // Next level banner
	let mut text : String = LEVEL_TITLES[game.current_level].to_string();
	text.push_str("\n\n\n");
	text.push_str(HINTS.choose(&mut rand::rng()).unwrap());
        texts.push(format!(
            "Level #{}\n{}",
            game.current_level + 1,
            text,
        ));
        colors.push(Color::Srgba(LEVEL_COLORS[game.current_level]));
    }

    commands.spawn((
        Text2d::new(""),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 20.,
            ..default()
        },
        Transform::from_xyz(0., 0., LAYER_BANNER),
        TextLayout::new_with_justify(JustifyText::Center),
        Transition {
            idx: 0,
            timer: Timer::new(Duration::from_secs(TRANSITION_BANNER_SECS), TimerMode::Once),
            texts,
            colors,
            end_game,
        },
    ));
}

pub fn transition_update(
    transition_comp: Single<(&mut Text2d, &mut TextColor, &mut Transition)>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (mut text, mut color, mut transition) = transition_comp.into_inner();

    transition.timer.tick(time.delta());
    if transition.timer.just_finished() || transition.idx == 0 {
        if transition.idx < transition.texts.len() {
            text.clear();
            text.push_str(transition.texts[transition.idx].as_str());
            *color = TextColor(transition.colors[transition.idx]);
            transition.idx += 1;
            transition.timer.reset();
        } else if transition.end_game {
            game_state.set(GameState::Menu);
        } else {
            game_state.set(GameState::Game);
        }
    }
}
