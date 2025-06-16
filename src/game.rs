use crate::animation::*;
use crate::balls_and_bullets::*;
use crate::barrels::*;
use crate::collisions::*;
use crate::consts::*;
use crate::countdown_and_portal::*;
use crate::meanies::*;
use crate::paddle::*;
use crate::shop::*;
use crate::*;
use bevy::audio::*;
use bevy::prelude::*;
use rand::distr::weighted::WeightedIndex;
use std::time::Duration;

#[derive(Component)]
/// Tag component to mark entities as part of the game scene
/// Used for despawning all game entities when transitioning states
pub struct GameTag;

#[derive(Component)]
/// Component for brick entities
pub struct Brick {
    /// Brick variant/type (affects appearance and behavior)
    pub variant: usize,
}

#[derive(Component)]
/// Component for messages displayed in the info area
pub struct InfoAreaMsg {
    /// Timer for how long the message should be displayed
    pub timer: Timer,
}

#[derive(Resource)]
/// Main game resource that stores all game state and assets
pub struct Game {
    /// Grid of brick entities (None for empty spaces)
    pub grid: Vec<Option<Entity>>,
    /// Number of lives remaining
    pub nlives: u32,
    /// Number of balls currently in play
    pub nballs: u32,
    /// Number of enemy meanies currently in play
    pub nmeanies: u32,
    /// Current level index
    pub current_level: usize,
    /// Number of bricks remaining to be destroyed
    pub bricks_left: u32,
    /// Whether the exit portal is currently open
    pub portal_open: bool,
    /// Seconds remaining on the level timer
    pub seconds_left: f32,
    /// Average time between frame updates, used for stability
    pub avg_delta: f32,

    // Asset handles
    /// Menu background image
    pub h_menu_bg: Handle<Image>,
    /// Paddle variant images
    pub h_paddles: Vec<Handle<Image>>,
    /// Brick variant images
    pub h_bricks: Vec<Handle<Image>>,
    /// Ball image
    pub h_ball: Handle<Image>,
    /// Bullet image
    pub h_bullet: Handle<Image>,
    /// Level background images
    pub h_backgrounds: Vec<Handle<Image>>,
    /// Animation frames for brick destruction
    pub h_brick_frames: Vec<Vec<Handle<Image>>>,
    /// Animation frames for barrel power-ups
    pub h_barrel_frames: Vec<Vec<Handle<Image>>>,
    /// Animation frames for portal
    pub h_portal_frames: Vec<Handle<Image>>,
    /// Right side portal image
    pub h_portal_right: Handle<Image>,
    /// Top-left portal image
    pub h_portal_top_left: Handle<Image>,
    /// Top-right portal image
    pub h_portal_top_right: Handle<Image>,
    /// Animation frames for ball impacts
    pub h_ball_impact_frames: Vec<Handle<Image>>,
    /// Animation frames for bullet impacts
    pub h_bullet_impact_frames: Vec<Handle<Image>>,
    /// Animation frames for enemy meanies
    pub h_meanies_frames: Vec<Vec<Handle<Image>>>,
    // pub h_meanies_shadows: Vec<Vec<Handle<Image>>>,
    /// Shadow images for paddle variants
    pub h_paddle_shadows: Vec<Handle<Image>>,
    /// Shadow image for bricks
    pub h_brick_shadow: Handle<Image>,
    /// Shadow image for ball
    pub h_ball_shadow: Handle<Image>,
    /// Shadow image for barrels
    pub h_barrel_shadow: Handle<Image>,
    /// Game logo image
    pub h_logo: Handle<Image>,
    /// Probability distribution for barrel types
    pub barrel_dist: WeightedIndex<u32>,

    // Audio assets
    /// Main theme music
    pub music_main_theme: Handle<AudioSource>,
    /// Arkanoid theme music
    pub music_arkanoid: Handle<AudioSource>,
    /// Wall hit sound effect
    pub sound_hit_wall: Handle<AudioSource>,
    /// Brick hit sound effect
    pub sound_hit_brick: Handle<AudioSource>,
    /// Bullet firing sound effect
    pub sound_fire_bullet: Handle<AudioSource>,
    /// Paddle hit sound effect
    pub sound_paddle: Handle<AudioSource>,
    /// Magnet paddle catch sound effect
    pub sound_magnet: Handle<AudioSource>,
    /// Bullet hit sound effects (randomly selected)
    pub sound_bullet_hit: Vec<Handle<AudioSource>>,
    /// Chime sound effect (used in level 3)
    pub sound_chime: Handle<AudioSource>,
    /// Level start sound effect
    pub sound_start: Handle<AudioSource>,
    /// Portal sound effect
    pub sound_portal: Handle<AudioSource>,
    /// Barrel power-up sound effects
    pub sound_barrels: Vec<Handle<AudioSource>>,

    // Player progress
    /// List of levels the player has unlocked
    pub levels_unlocked: Vec<String>,
    /// List of secrets the player has discovered
    pub secrets_unlocked: Vec<String>,
    /// List of secrets generated in the current game
    pub secrets_generated: Vec<String>,
    /// Player's username
    pub username: String,
}

/// Registers all game systems with the Bevy app
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_enter)
        .add_systems(OnExit(GameState::Game), despawn_all::<GameTag>)
        .add_systems(
            Update,
            (
                paddle_update,
                ball_update,
                bullets_update,
                barrels_update,
                portal_update,
                meanies_update,
                animate,
                countdown_update,
                lives_update,
            )
                .run_if(in_state(GameState::Game)),
        );
}

/// Initialization when entering a new level (i.e. enters Game state).
/// Creates the paddle, ball, bricks, portal, and UI elements
pub fn game_enter(mut commands: Commands, mut game: ResMut<Game>) {
    // Create Paddle
    let paddle_variant = 0;
    let paddle = commands
        .spawn((
            Sprite::from_image(game.h_paddles[0].clone()),
            Transform::from_xyz(GAMEAREA_CENTER_X, PADDLE_Y, LAYER_PADDLE),
            Paddle {
                variant: paddle_variant,
                gun_timer: Timer::new(Duration::from_millis(GUN_TIMER_MS), TimerMode::Once),
            },
            GameTag,
        ))
        .id();
    let paddle_shadow = commands
        .spawn((
            Sprite::from_image(game.h_paddle_shadows[0].clone()),
            Transform::from_xyz(SHADOW_DX, SHADOW_DY, LAYER_SHADOWS - LAYER_PADDLE),
            PaddleShadow {},
        ))
        .id();
    commands.entity(paddle).add_child(paddle_shadow);

    // Create Initial Ball
    game.nballs = 0;
    spawn_new_ball(&mut commands, paddle_variant, GAMEAREA_CENTER_X, &mut game);

    // Reset Meanies
    game.nmeanies = 0;

    // Countdown
    game.seconds_left = LEVEL_TIMERS[game.current_level];
    let mut msg: String = String::from("Beat Timer\nto Discover\nSecret");
    let mut color: Color = Color::WHITE;
    if level_is_unlocked(game.current_level, &game) {
        // Portal closing countdown if level is unlocked
        msg = String::from("\nExit!");
        game.seconds_left = UNLOCKED_PORTAL_TIMER;
        color = Color::Srgba(Srgba::new(0.1, 1.0, 0.1, 1.0));
    } else if secret_is_discovered(game.current_level, &game) {
        // Harder countdown if secret is unlocked but level is not
        msg = String::from("Beat Time\nto Unlock\nLevel");
        game.seconds_left = LEVEL_TIMERS[game.current_level] - 30.;
        color = Color::Srgba(Srgba::new(1.0, 0.1, 0.1, 1.0));
    }
    commands.spawn((
        Transform::from_xyz(INFOAREA_CENTER_X, INFOAREA_HEADER_Y, LAYER_BANNER),
        TextLayout::new_with_justify(JustifyText::Center),
        Text2d::new(msg),
        GameTag,
    ));
    commands.spawn((
        Countdown {
            timer: Timer::from_seconds(game.seconds_left, TimerMode::Once),
        },
        Text2d::new(format!("{:.0}", game.seconds_left).as_str()),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        Transform::from_xyz(INFOAREA_CENTER_X, INFOAREA_TIMER_Y, LAYER_BANNER),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(color),
        GameTag,
    ));

    // Info Bar at the bottom of the infoarea
    commands.spawn((
        Transform::from_xyz(INFOAREA_CENTER_X, INFOAREA_MSG_Y, LAYER_BANNER),
        TextLayout::new_with_justify(JustifyText::Center),
        Text2d::new(""),
        InfoAreaMsg {
            timer: Timer::from_seconds(INFOAREA_TIMER, TimerMode::Once),
        },
        GameTag,
    ));

    // Create Portal
    game.portal_open = false;
    let portal_state: PortalState = if level_is_unlocked(game.current_level, &game) {
        commands.spawn((
            AudioPlayer::new(game.sound_portal.clone()),
            PlaybackSettings::DESPAWN,
        ));
        PortalState::Opening
    } else {
        PortalState::Closed
    };
    commands.spawn((
        Sprite::from_image(game.h_portal_frames[0].clone()),
        Transform::from_xyz(GAMEAREA_MAXX - 55., GAMEAREA_MINY + 35., LAYER_PORTAL_BG),
        Animation {
            timer: Timer::from_seconds(1.0 / PORTAL_FRAMERATE, TimerMode::Repeating),
            frozen: matches!(portal_state, PortalState::Closed),
            variant: AnimationType::Freeze,
            frames: game.h_portal_frames.clone(),
            current_frame: 0,
            reverse: false,
            velocity: Vec2::ZERO,
        },
        Portal { portal_state },
        GameTag,
    ));
    commands.spawn((
        Sprite::from_image(game.h_portal_right.clone()),
        Transform::from_xyz(GAMEAREA_MAXX - 10., GAMEAREA_MINY + 35., LAYER_PORTAL_FG),
        GameTag,
    ));

    // Meanies portals
    commands.spawn((
        Sprite::from_image(game.h_portal_top_left.clone()),
        Transform::from_xyz(MEANIES_PORTAL_X[0], GAMEAREA_MAXY - 20., LAYER_PORTAL_FG),
        GameTag,
    ));
    commands.spawn((
        Sprite::from_image(game.h_portal_top_right.clone()),
        Transform::from_xyz(MEANIES_PORTAL_X[1], GAMEAREA_MAXY - 20., LAYER_PORTAL_FG),
        GameTag,
    ));

    // Logo
    commands.spawn((
        Sprite::from_image(game.h_logo.clone()),
        Transform::from_xyz(INFOAREA_CENTER_X, INFOAREA_LOGO_Y, LAYER_BANNER),
        GameTag,
    ));

    // Lives
    if game.current_level == 0 {
        game.nlives = NLIVES;
    }
    commands.spawn((
        Text2d::new("Lives: -"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Transform::from_xyz(INFOAREA_CENTER_X, INFOAREA_LIVES_Y, LAYER_BANNER),
        TextLayout::new_with_justify(JustifyText::Center),
        LivesDisplay {
            nlives_displayed: 0,
        },
        GameTag,
    ));

    // Background image
    commands.spawn((
        Sprite::from_image(game.h_backgrounds[game.current_level].clone()),
        Transform::from_xyz(GAMEAREA_CENTER_X, GAMEAREA_CENTER_Y, LAYER_BG),
        GameTag,
    ));

    // Populate Bricks
    game.bricks_left = 0;
    let level = LEVELS[game.current_level];
    for (r, &row) in level.iter().enumerate() {
        for (c, &brick) in row.iter().enumerate() {
            let grid_idx: usize = r * GRID_COLS + c;

            // Skip empty slots
            if brick == b' ' {
                game.grid[grid_idx] = None;
                continue;
            }

            // convert hex character to integer
            let variant = if brick <= b'9' {
                brick - b'0'
            } else {
                brick - b'a' + 10
            } as usize;

            // Spawn brick
            let (x, y) = game_rc_to_xy(r, c);
            let brick_entity_id = commands
                .spawn((
                    Sprite::from_image(game.h_bricks[variant].clone()),
                    Transform::from_xyz(x, y, LAYER_BRICKS),
                    Brick { variant },
                    GameTag,
                ))
                .id();

            // All bricks except 14 have a shadow
            if variant != 14 || secret_is_unlocked(Secret::XRay as usize, &game) {
                let shadow_id = commands
                    .spawn((
                        Sprite::from_image(game.h_brick_shadow.clone()),
                        Transform::from_xyz(SHADOW_DX, SHADOW_DY, LAYER_SHADOWS - LAYER_BRICKS),
                    ))
                    .id();
                commands.entity(brick_entity_id).add_child(shadow_id);
            }

            // Add brick to grid
            game.grid[grid_idx] = Some(brick_entity_id);
            if variant != 13 {
                game.bricks_left += 1;
            }
        }
    }

    if game.current_level == 3 {
        // Annoying chime sound in level 3
        commands.spawn((
            AudioPlayer::new(game.sound_chime.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(5.0)),
            GameTag,
        ));
    } else {
        // Regular start level tune for the rest
        commands.spawn((
            AudioPlayer::new(game.sound_start.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
