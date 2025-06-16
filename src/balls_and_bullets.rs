use crate::animation::*;
use crate::collisions::*;
use crate::consts::*;
use crate::game::*;
use crate::paddle::*;
use crate::*;

use bevy::audio::*;
use bevy::prelude::*;
use rand::seq::*;

#[derive(Component, Clone)]
/// Component for ball entities with physics properties
pub struct Ball {
    /// Total number of impacts (used for speed increases)
    pub impacts: u32,
    /// Impacts since last paddle contact (used for direction changes)
    pub impacts_since_paddle: u32,
    /// Current speed of the ball in pixels per second
    pub speed: f32,
    /// Normalized direction vector
    pub direction: Vec2,
    /// Whether the ball is caught by a magnetic paddle
    pub caught: bool,
    /// Whether the ball is currently in collision with something
    /// Used to prevent multiple collisions in a single frame
    pub in_collision: bool,
}

#[derive(Component)]
/// Component for bullets shot by the gun paddle
pub struct Bullet;

#[derive(Component)]
/// Component for displaying the number of lives
pub struct LivesDisplay {
    /// The number of lives currently shown on screen
    pub nlives_displayed: u32,
}

/// System that handles ball physics and collisions
/// Manages ball movement, wall/brick/paddle collisions, and speed adjustments
pub fn ball_update(
    mut commands: Commands,
    mut bricks: Query<&mut Brick>,
    mut balls: Query<(Entity, &mut Transform, &mut Ball), Without<Paddle>>,
    paddle_comp: Single<(&Transform, &Paddle), Without<Ball>>,
    msg_comp: Single<(&mut Text2d, &mut InfoAreaMsg)>,
    t_step: Res<Time>,
    mut game: ResMut<Game>,
) {
    let (p_tr, paddle) = paddle_comp.into_inner();
    let p_size = PADDLE_SIZES[paddle.variant];
    let (mut text, mut text_timer) = msg_comp.into_inner();

    // Instead of using dt directly, we use an average to reduce jitter cause
    // by a problem with the way bevy calculates frame time
    game.avg_delta = 0.8 * game.avg_delta + 0.2 * t_step.delta().as_secs_f32();
    let dt = game.avg_delta;

    for (ball_entity, mut ball_tr, mut ball) in &mut balls {
        // Skip balls caught by the paddle
        if ball.caught {
            continue;
        }

        // Move ball pixel by pixel
        for _ in 0..(ball.speed * dt).ceil() as i32 {
            ball_tr.translation.x += ball.direction.x;
            ball_tr.translation.y += ball.direction.y;

            // Check collision with walls
            let mut wall_collisions: Vec<Vec2> = Vec::new();
            if ball_tr.translation.x - BALL_RADIUS < BALLAREA_MINX {
                ball.direction.x = ball.direction.x.abs();
                wall_collisions.push(Vec2::new(
                    ball_tr.translation.x - BALL_RADIUS,
                    ball_tr.translation.y,
                ));
            }
            if ball_tr.translation.x + BALL_RADIUS > BALLAREA_MAXX {
                ball.direction.x = -ball.direction.x.abs();
                wall_collisions.push(Vec2::new(
                    ball_tr.translation.x + BALL_RADIUS,
                    ball_tr.translation.y,
                ));
            }
            if ball_tr.translation.y + BALL_RADIUS > BALLAREA_MAXY {
                ball.direction.y = -ball.direction.y.abs();
                wall_collisions.push(Vec2::new(
                    ball_tr.translation.x,
                    ball_tr.translation.y + BALL_RADIUS,
                ));
            }

            ball.impacts += wall_collisions.len() as u32;
            ball.impacts_since_paddle += wall_collisions.len() as u32;

            // Collision with bottom wall
            if ball_tr.translation.y + BALL_RADIUS < GAMEAREA_MINY {
                // ball.direction.y = ball.direction.y.abs();
                // despawn ball
                commands.entity(ball_entity).despawn();
                game.nballs -= 1;
                break;
            }

            // Wall impact animation
            for pos in wall_collisions {
                commands.spawn((
                    Sprite::from_image(game.h_ball_impact_frames[0].clone()),
                    Transform::from_xyz(pos.x, pos.y, LAYER_EXPLOSIONS),
                    Animation {
                        timer: Timer::from_seconds(1.0 / BRICK_FRAMERATE, TimerMode::Repeating),
                        frozen: false,
                        variant: AnimationType::Despawn,
                        frames: game.h_ball_impact_frames.clone(),
                        current_frame: 0,
                        reverse: false,
                        velocity: Vec2::ZERO,
                    },
                    GameTag,
                ));
                commands.spawn((
                    AudioPlayer::new(game.sound_hit_wall.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }

            // Check collision with paddle
            if let Some(collides) = collision(
                Body::Round(ball_tr.translation.truncate(), BALL_RADIUS),
                p_tr.translation.truncate(),
                p_size,
            ) {
                // ball.in_collision is a flag to avoid continuous collisions with the paddle
                if !ball.in_collision {
                    ball.impacts_since_paddle = 0;
                    match collides {
                        Collision::Top => {
                            if ball.direction.y < 0. {
                                let slope: f32 = (PADDLE_MAX_ANGLE - PADDLE_MIN_ANGLE) / p_size.x;
                                let p_maxx = p_tr.translation.x + p_size.x / 2.0;
                                let p_minx = p_maxx - p_size.x;
                                let dx = (p_maxx - ball_tr.translation.x).clamp(0., p_size.x);
                                let angle = PADDLE_MIN_ANGLE + slope * dx;
                                ball.direction = Vec2::from_angle(angle);
                                if paddle.variant == PADDLE_MAGNET
                                    && ball_tr.translation.x >= p_minx + PADDLE_MAGNET_BORDER
                                    && ball_tr.translation.x <= p_maxx - PADDLE_MAGNET_BORDER
                                {
                                    ball.caught = true;
                                    ball_tr.translation.y =
                                        p_tr.translation.y + p_size.y / 2.0 + BALL_RADIUS;
                                    commands.spawn((
                                        AudioPlayer::new(game.sound_magnet.clone()),
                                        PlaybackSettings::DESPAWN,
                                    ));
                                    break;
                                }
                            }
                        }
                        Collision::Left => ball.direction.x = -ball.direction.x.abs(),
                        Collision::Right => ball.direction.x = ball.direction.x.abs(),
                        Collision::Bottom => ball.direction.y = -ball.direction.y.abs(),
                    }
                    commands.spawn((
                        AudioPlayer::new(game.sound_paddle.clone()),
                        PlaybackSettings::DESPAWN,
                    ));
                    ball.in_collision = true;
                }
            } else {
                ball.in_collision = false;
            }

            // Check collisions with bricks
            let collisions = collision_with_bricks(
                &mut commands,
                Body::Round(ball_tr.translation.truncate(), BALL_RADIUS),
                ball.direction,
                &mut bricks,
                &mut game,
            );

            if !collisions.is_empty() {
                commands.spawn((
                    AudioPlayer::new(game.sound_hit_brick.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }

            ball.impacts += collisions.len() as u32;
            ball.impacts_since_paddle += collisions.len() as u32;

            for c in collisions {
                match c {
                    Collision::Left => ball.direction.x = -ball.direction.x.abs(),
                    Collision::Right => ball.direction.x = ball.direction.x.abs(),
                    Collision::Bottom => ball.direction.y = -ball.direction.y.abs(),
                    Collision::Top => ball.direction.y = ball.direction.y.abs(),
                }
            }
        }

        // Update ball speed
        if ball.impacts > BALL_SPEEDUP_IMPACTS {
            ball.impacts = 0;
            ball.speed = (ball.speed * BALL_SPEED_DELTA).min(BALL_MAX_SPEED);
            *text = Text2d::new(format!(
                "Speed {}%",
                (ball.speed / BALL_INITIAL_SPEED * 100.).round()
            ));
            text_timer.timer.reset();
        }

        // Nudge ball
        if ball.impacts_since_paddle > BALL_NUDGE_IMPACTS {
            ball.direction = nudge_ball(ball.direction, MULTIBALL_ANGLE_RANGE);
            ball.impacts_since_paddle = 0;
            *text = Text2d::new(format!(
                "Nudging Ball {}%",
                (ball.speed / BALL_INITIAL_SPEED * 100.).round()
            ));
            text_timer.timer.reset();
        }
    }
}

pub fn nudge_ball(direction: Vec2, max_rotation: f32) -> Vec2 {
    let rotation = Vec2::from_angle(rand::random_range(-max_rotation..max_rotation));
    let mut new_direction = direction.rotate(rotation);
    if new_direction.x.abs() < 0.05 || new_direction.y.abs() < 0.05 {
        let rotation = Vec2::from_angle(0.1);
        new_direction = new_direction.rotate(rotation);
    }
    new_direction
}

/// Creates a new ball attached to the paddle
pub fn spawn_new_ball(
    commands: &mut Commands,
    paddle_variant: usize,
    paddle_x: f32,
    game: &mut Game,
) {
    let ball_y = PADDLE_Y + PADDLE_SIZES[paddle_variant].y / 2.0 + BALL_RADIUS;
    let ball_x = paddle_x + 5.;
    let ball = commands
        .spawn((
            Sprite::from_image(game.h_ball.clone()),
            Transform::from_xyz(ball_x, ball_y, LAYER_BALL),
            Ball {
                impacts: 0,
                impacts_since_paddle: 0,
                speed: BALL_INITIAL_SPEED,
                direction: Vec2::from_angle(BALL_INITIAL_ANGLE),
                caught: true,
                in_collision: false,
            },
            GameTag,
        ))
        .id();
    let ball_shadow = commands
        .spawn((
            Sprite::from_image(game.h_ball_shadow.clone()),
            Transform::from_xyz(SHADOW_DX, SHADOW_DY, LAYER_SHADOWS - LAYER_BALL),
        ))
        .id();
    commands.entity(ball).add_child(ball_shadow);
    game.nballs += 1;
}

/// System that handles bullet movement and collision with bricks
pub fn bullets_update(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    mut bricks: Query<&mut Brick>,
    mut game: ResMut<Game>,
) {
    let dt = game.avg_delta;

    for (bullet_entity, mut bullet_tr) in &mut bullets {
        // Move bullets up
        bullet_tr.translation.y += BULLET_SPEED * dt;

        // Explode bullet if hits brick or top wall
        let bullet_top = bullet_tr.translation.y + BULLET_SIZE.y / 2.0;
        if bullet_top > BALLAREA_MAXY
            || !collision_with_bricks(
                &mut commands,
                Body::Rectangular(bullet_tr.translation.truncate(), BULLET_SIZE),
                Vec2::new(0., 1.),
                &mut bricks,
                &mut game,
            )
            .is_empty()
        {
            commands.spawn((
                Sprite::from_image(game.h_ball_impact_frames[0].clone()),
                Transform::from_xyz(bullet_tr.translation.x, bullet_top, LAYER_EXPLOSIONS),
                Animation {
                    timer: Timer::from_seconds(1.0 / BRICK_FRAMERATE, TimerMode::Repeating),
                    frozen: false,
                    variant: AnimationType::Despawn,
                    frames: game.h_bullet_impact_frames.clone(),
                    current_frame: 0,
                    reverse: false,
                    velocity: Vec2::ZERO,
                },
                GameTag,
            ));
            commands.spawn((
                AudioPlayer::new(
                    game.sound_bullet_hit
                        .choose(&mut rand::rng())
                        .unwrap()
                        .clone(),
                ),
                PlaybackSettings::DESPAWN,
            ));
            commands.entity(bullet_entity).despawn();
        }
    }
}

/// System that handles player lives and game over conditions
/// Manages life loss when balls are lost and spawns new balls
pub fn lives_update(
    mut commands: Commands,
    paddle_comp: Single<(&mut Transform, &mut Sprite, &mut Paddle), Without<PaddleShadow>>,
    shadow_comp: Single<&mut Sprite, With<PaddleShadow>>,
    nlives_comp: Single<(&mut Text2d, &mut LivesDisplay)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game: ResMut<Game>,
) {
    // if last ball then take life
    if game.nballs == 0 {
        game.nlives -= 1;

        // if lives is zero, end game
        if game.nlives == 0 {
            game_state.set(GameState::Transition);
            return;
        }

        // if not last life, reset paddle...
        let (mut p_tr, mut p_sprite, mut paddle) = paddle_comp.into_inner();
        let mut sprite_shadow = shadow_comp.into_inner();
        paddle.variant = 0;
        *p_sprite = Sprite::from_image(game.h_paddles[0].clone());
        *sprite_shadow = Sprite::from_image(game.h_paddle_shadows[0].clone());
        p_tr.translation.x = GAMEAREA_CENTER_X;

        // ... and spawn new ball
        spawn_new_ball(&mut commands, paddle.variant, p_tr.translation.x, &mut game);
        commands.spawn((
            AudioPlayer::new(game.sound_start.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    // update display if necessary
    let (mut text, mut display) = nlives_comp.into_inner();
    if display.nlives_displayed != game.nlives {
        text.clear();
        text.push_str(format!("Lives: {}", game.nlives).as_str());
        display.nlives_displayed = game.nlives;
    }
}
