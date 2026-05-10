mod animation;
mod balls_and_bullets;
mod barrels;
mod collisions;
mod consts;
mod countdown_and_portal;
mod game;
mod meanies;
mod menu;
mod paddle;
mod save;
mod shop;
mod splash;
mod transition;

use crate::consts::*;
use crate::shop::*;
use bevy::prelude::*;
use bevy::window::*;
use std::env;

#[derive(States, Default, PartialEq, Eq, Copy, Hash, Clone, Debug)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Shop,
    Game,
    Transition,
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for -c flag and handle the sequence of strings
    if let Some(c_index) = args.iter().position(|arg| arg == "-c") {
        // Get all arguments after -c
        let codes: Vec<&String> = args.iter().skip(c_index + 1).collect();

        for string in codes {
            println!("{}", decode(string).unwrap());
        }
        return; // Exit after handling the command
    }

    // Normal game execution if no -c flag
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        (2. * SCREEN_WIDTH) as u32,
                        (2. * SCREEN_HEIGHT) as u32,
                    )
                    .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .init_state::<GameState>()
        .add_systems(Startup, startup)
        .add_plugins(splash::splash_plugin)
        .add_plugins(game::game_plugin)
        .add_plugins(transition::transition_plugin)
        .add_plugins(menu::menu_plugin)
        .add_plugins(shop::shop_plugin)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMin {
                min_width: SCREEN_WIDTH,
                min_height: SCREEN_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// This despawns all entities with the <T> tag. Useful when changing the game state.
fn despawn_all<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
