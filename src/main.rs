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
use bevy::prelude::*;
use bevy::window::*;

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
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(2. * SCREEN_WIDTH, 2. * SCREEN_HEIGHT)
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
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
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
