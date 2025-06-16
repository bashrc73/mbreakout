use crate::animation::*;
use crate::balls_and_bullets::*;
use crate::collisions::*;
use crate::countdown_and_portal::*;
use crate::game::*;
use crate::paddle::*;
use crate::*;
use bevy::math::*;
use bevy::prelude::*;
use rand::distr::Distribution;
use std::time::Duration;

#[derive(Component)]
pub struct Barrel {
    variant: usize,
}

pub fn barrels_spawn(commands: &mut Commands, mut translation: Vec3, game: &Game) {
    if rand::random_bool(BARREL_CHANCE) {
        let variant: usize = game.barrel_dist.sample(&mut rand::rng());
        translation.z = LAYER_BARRELS;
        let barrel = commands
            .spawn((
                Sprite::from_image(game.h_barrel_frames[variant][0].clone()),
                Transform {
                    translation,
                    ..default()
                },
                Animation {
                    timer: Timer::from_seconds(1.0 / BARREL_FRAMERATE, TimerMode::Repeating),
                    frozen: false,
                    variant: AnimationType::Repeat,
                    frames: game.h_barrel_frames[variant].clone(),
                    current_frame: 0,
                    reverse: false,
                    velocity: Vec2::new(0., -BARREL_SPEED),
                },
                Barrel { variant },
                GameTag,
            ))
            .id();
        let barrel_shadow = commands
            .spawn((
                Sprite::from_image(game.h_barrel_shadow.clone()),
                Transform::from_xyz(SHADOW_DX, SHADOW_DY, LAYER_SHADOWS - LAYER_BARRELS),
            ))
            .id();
        commands.entity(barrel).add_child(barrel_shadow);
    }
}

pub fn barrels_update(
    mut commands: Commands,
    barrel_query: Query<(Entity, &Transform, &Barrel)>,
    mut balls: Query<(Entity, &Transform, &mut Ball)>,
    paddle_comp: Single<(&mut Sprite, &Transform, &mut Paddle), Without<PaddleShadow>>,
    shadow_comp: Single<&mut Sprite, With<PaddleShadow>>,
    countdown_comp: Single<&mut Countdown>,
    msg_comp: Single<(&mut Text2d, &mut InfoAreaMsg)>,
    mut game: ResMut<Game>,
) {
    let mut countdown = countdown_comp.into_inner();
    let (mut sprite, paddle_tr, mut paddle) = paddle_comp.into_inner();
    let paddle_size = PADDLE_SIZES[paddle.variant];
    let mut new_paddle_variant: usize = paddle.variant;
    let mut multiball: u8 = 0;
    let mut delta_speed: f32 = 1.0;
    let mut barrel_collisions: u8 = 0;
    let (mut text, mut text_timer) = msg_comp.into_inner();
    for (barrel_entity, barrel_tr, barrel) in &barrel_query {
        if collision(
            Body::Rectangular(barrel_tr.translation.truncate(), BARREL_SIZE),
            paddle_tr.translation.truncate(),
            paddle_size,
        )
        .is_some()
        {
            match barrel.variant {
                0 => new_paddle_variant = PADDLE_LARGE,
                1 => new_paddle_variant = PADDLE_GUN,
                2 => new_paddle_variant = PADDLE_SMALL,
                3 => new_paddle_variant = PADDLE_MAGNET,
                4 => multiball += 1,
                5 => delta_speed *= BALL_SPEED_DELTA,
                6 => delta_speed /= BALL_SPEED_DELTA,
                7 => {} // Portal
                8 => {} // Extra Life
                9 => {
                    // Extra Time
                    let remaining: u64 = countdown.timer.remaining_secs().ceil() as u64;
                    if remaining > 0 {
                        let duration = countdown.timer.duration();
                        let extra = Duration::from_secs(30);
                        countdown.timer.set_duration(duration + extra);
                    }
                }
                _ => {}
            }
            *text = Text2d::new(BARREL_TITLES[barrel.variant]);
            text_timer.timer.reset();
            barrel_collisions += 1;
            commands.entity(barrel_entity).despawn();
            commands.spawn((
                AudioPlayer::new(game.sound_barrels[barrel.variant].clone()),
                PlaybackSettings::DESPAWN,
            ));
        } else if barrel_tr.translation.y < GAMEAREA_MINY {
            commands.entity(barrel_entity).despawn();
        }
    }

    if barrel_collisions == 0 {
        return;
    }

    // Update paddle
    if new_paddle_variant != paddle.variant {
        let mut sprite_shadow = shadow_comp.into_inner();
        paddle.variant = new_paddle_variant;
        *sprite = Sprite::from_image(game.h_paddles[new_paddle_variant].clone());
        *sprite_shadow = Sprite::from_image(game.h_paddle_shadows[new_paddle_variant].clone());
    }

    // Update balls
    for (ball_entity, ball_tr, mut ball) in &mut balls {
        // Update speed
        if delta_speed != 1.0 {
            ball.speed = (ball.speed * delta_speed).clamp(BALL_MIN_SPEED, BALL_MAX_SPEED);
            ball.impacts = 0;
            // Only the speed of the last ball will be seen but that's OK
            *text = Text2d::new(format!(
                "Speed {}%",
                (ball.speed / BALL_INITIAL_SPEED * 100.).round()
            ));
        }

        // unmagnetize if paddle was changed
        if paddle.variant != PADDLE_MAGNET {
            ball.caught = false;
        }

        // Spawn multiball event
        let mut m = multiball;
        while m > 0 && !ball.caught && game.nballs < MULTIBALL_MAX - 1 {
            m -= 1;
            // Spawn 3 new balls
            for _ in 0..3 {
                let direction = nudge_ball(ball.direction, MULTIBALL_ANGLE_RANGE);
                let new_ball = commands
                    .spawn((
                        Sprite::from_image(game.h_ball.clone()),
                        *ball_tr,
                        Ball {
                            impacts: 0,
                            impacts_since_paddle: 0,
                            speed: ball.speed,
                            direction,
                            caught: false,
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
                commands.entity(new_ball).add_child(ball_shadow);
            }
            // Despawn old ball
            commands.entity(ball_entity).despawn();
            game.nballs += 2;
        }
    }
}
