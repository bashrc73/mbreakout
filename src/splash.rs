use super::{GameState, despawn_all};
use crate::consts::*;
use crate::game::*;
use crate::save::*;
use bevy::prelude::*;
use rand::distr::weighted::WeightedIndex;
use std::thread;
use users::{get_current_uid, get_user_by_uid};

const SPLASH_TIMER: f32 = 5.;

#[derive(Component)]
struct SplashTag;

#[derive(Component)]
struct Countdown {
    timer: Timer,
}

pub fn splash_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Splash),
        (splash_setup, load_game_resources),
    )
    .add_systems(Update, splash_timer.run_if(in_state(GameState::Splash)))
    .add_systems(OnExit(GameState::Splash), despawn_all::<SplashTag>);
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Countdown {
            timer: Timer::from_seconds(SPLASH_TIMER, TimerMode::Once),
        },
        SplashTag,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("images/logo.png")),
        SplashTag,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("images/logo_breakout.png")),
        Transform::from_xyz(0., 215., 1.),
        SplashTag,
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("images/logo_edition.png")),
        Transform::from_xyz(0., -215., 1.),
        SplashTag,
    ));
}

fn splash_timer(
    countdown_comp: Single<&mut Countdown>,
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let mut countdown = countdown_comp.into_inner();
    if countdown.timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}

pub fn load_game_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let mut username = String::from(user.name().to_string_lossy());
    username.truncate(USERNAME_LEN);
    username = format!("{:X<USERNAME_LEN$}", username.to_uppercase());

    let mut game = Game {
        grid: vec![None; GRID_ROWS * GRID_COLS],
        nlives: NLIVES,
        nballs: 0,
        nmeanies: 0,
        current_level: 0,
        bricks_left: 0,
        portal_open: false,
        seconds_left: 0.,
	avg_delta: 1./60., // 60 FPS, will converge quickly to actual FPS
        h_menu_bg: asset_server.load("images/background_menu.png"),
        h_paddles: vec![
            asset_server.load("images/bat00.png"),
            asset_server.load("images/bat33.png"), // Extended
            asset_server.load("images/bat23.png"), // Fire
            asset_server.load("images/bat43.png"), // Shrink
            asset_server.load("images/bat13.png"), // Magnet
        ],
        h_bricks: Vec::new(),
        h_ball: asset_server.load("images/ball0.png"),
        h_bullet: asset_server.load("images/bullet0.png"),
        h_backgrounds: Vec::new(),
        h_brick_frames: Vec::new(),
        h_barrel_frames: Vec::new(),
        h_portal_frames: Vec::new(),
        h_portal_right: asset_server.load("images/portal_right_frame.png"),
        h_portal_top_left: asset_server.load("images/portal_top_left_frame.png"),
        h_portal_top_right: asset_server.load("images/portal_top_right_frame.png"),
        h_ball_impact_frames: Vec::new(),
        h_bullet_impact_frames: Vec::new(),
        h_meanies_frames: Vec::new(),
        // h_meanies_shadows: Vec::new(),
        h_paddle_shadows: vec![
            asset_server.load("images/bats00.png"),
            asset_server.load("images/bats33.png"), // Extended
            asset_server.load("images/bats23.png"), // Fire
            asset_server.load("images/bats43.png"), // Shrink
            asset_server.load("images/bats13.png"), // Magnet
        ],
        h_brick_shadow: asset_server.load("images/bricks.png"),
        h_ball_shadow: asset_server.load("images/balls.png"),
        h_barrel_shadow: asset_server.load("images/barrels.png"),
        h_logo: asset_server.load("images/logo_small.png"),
        barrel_dist: WeightedIndex::new(BARREL_WEIGHTS).unwrap(),

        music_main_theme: asset_server.load("music/title_theme.ogg"),
        music_arkanoid: asset_server.load("music/arkanoid.ogg"),
        sound_hit_wall: asset_server.load("sounds/hit_wall0.ogg"),
        sound_hit_brick: asset_server.load("sounds/hit_brick0.ogg"),
        sound_fire_bullet: asset_server.load("sounds/laser0.ogg"),
        sound_paddle: asset_server.load("sounds/hit_fast0.ogg"),
        sound_magnet: asset_server.load("sounds/ball_stick0.ogg"),
        sound_bullet_hit: vec![
            asset_server.load("sounds/bullet_hit0.ogg"),
            asset_server.load("sounds/bullet_hit1.ogg"),
            asset_server.load("sounds/bullet_hit2.ogg"),
            asset_server.load("sounds/bullet_hit3.ogg"),
        ],
        sound_chime: asset_server.load("sounds/chime.ogg"),
        sound_start: asset_server.load("sounds/arkanoid_start.ogg"),
        sound_portal: asset_server.load("sounds/portal_exit0.ogg"),
        sound_barrels: vec![
            asset_server.load("sounds/bat_extend0.ogg"),
            asset_server.load("sounds/bat_gun0.ogg"),
            asset_server.load("sounds/bat_small0.ogg"),
            asset_server.load("sounds/magnet0.ogg"),
            asset_server.load("sounds/multiball0.ogg"),
            asset_server.load("sounds/speed_up0.ogg"),
            asset_server.load("sounds/powerup0.ogg"), // SLOW
            asset_server.load("sounds/powerup0.ogg"), // PORTAL
            asset_server.load("sounds/extra_life0.ogg"),
            asset_server.load("sounds/powerup0.ogg"), // EXTRA TIME
        ],
        levels_unlocked: vec![String::new(); NLEVELS],
        secrets_unlocked: vec![String::new(); NLEVELS],
        secrets_generated: vec![String::new(); NLEVELS],
        username,
    };

    // This is a hack to give time to asset_server to load stuff
    // before loading more stuff and running out of file handlers
    thread::sleep(std::time::Duration::from_millis(2000));

    for l in 0..NLEVELS {
        let filepath = format!("images/arena{l:x}.png");
        game.h_backgrounds.push(asset_server.load(filepath));
    }

    for b in 0..BRICK_TYPES {
        let filepath = format!("images/brick{b:x}.png");
        game.h_bricks.push(asset_server.load(filepath));
    }

    // Regular bricks collision animations
    for b in 0..13 {
        game.h_brick_frames.push(Vec::new());
        for f in 0..BRICK_FRAMES {
            let filepath = format!("images/impact{b:x}{f:x}.png");
            game.h_brick_frames[b].push(asset_server.load(filepath));
        }
    }

    // Indestructible bricks collision animations
    game.h_brick_frames.push(Vec::new());
    game.h_brick_frames.push(Vec::new());
    for f in 0..IBRICK_FRAMES {
        let filepath = format!("images/impactd{f:x}.png");
        game.h_brick_frames[12].push(asset_server.load(filepath.clone()));
        game.h_brick_frames[13].push(asset_server.load(filepath));
    }

    // Invisible brick collision animations
    game.h_brick_frames.push(Vec::new());
    for f in 0..BRICK_FRAMES {
        let filepath = format!("images/impactc{f:x}.png");
        game.h_brick_frames[14].push(asset_server.load(filepath));
    }

    for b in 0..BARREL_TYPES {
        game.h_barrel_frames.push(Vec::new());
        for f in 0..BARREL_FRAMES {
            let filepath = format!("images/barrel{b:x}{f:x}.png");
            game.h_barrel_frames[b].push(asset_server.load(filepath));
        }
    }

    for f in 0..PORTAL_FRAMES {
        let filepath = format!("images/portal_exit{f:x}.png");
        game.h_portal_frames.push(asset_server.load(filepath));
    }

    for f in 0..BALL_IMPACT_FRAMES {
        let filepath = format!("images/impactc{f:x}.png");
        game.h_ball_impact_frames.push(asset_server.load(filepath));
    }

    for f in 0..BULLET_IMPACT_FRAMES {
        let filepath = format!("images/impactf{f:x}.png");
        game.h_bullet_impact_frames
            .push(asset_server.load(filepath));
    }

    for m in 0..MEANIES_TYPES {
        game.h_meanies_frames.push(Vec::new());
        // game.h_meanies_shadows.push(Vec::new());
        for f in 0..MEANIES_NFRAMES[m] {
            let filepath = format!("images/meanie{m:x}{f:x}.png");
            game.h_meanies_frames[m].push(asset_server.load(filepath));
            // let filepath = format!("images/meanies{m:x}{f:x}.png");
            // game.h_meanies_shadows[m].push(asset_server.load(filepath));
        }
    }

    game_load(&mut game);

    commands.insert_resource(game);
}
