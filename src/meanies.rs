use crate::animation::*;
use crate::balls_and_bullets::*;
use crate::collisions::*;
use crate::game::*;
use crate::paddle::*;
use crate::shop::*;
use crate::*;
use bevy::math::*;
use bevy::prelude::*;
use rand::seq::*;
use rand::*;
use std::f32::consts::*;

#[derive(Component)]
pub struct Meanie {
    variant: usize,
    pub speed: f32,
    pub angle: f32,
}

pub fn meanie_destroy(commands: &mut Commands, tr: &Transform, meanie: Entity, game: &mut Game) {
    commands.spawn((
        Sprite::from_image(game.h_ball_impact_frames[0].clone()),
        Transform::from_xyz(tr.translation.x, tr.translation.y, LAYER_EXPLOSIONS),
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
    commands.entity(meanie).despawn();
    game.nmeanies -= 1;
}

pub fn meanies_update(
    mut commands: Commands,
    mut meanies: Query<(Entity, &mut Meanie, &mut Transform, &Animation)>,
    mut bullets: Query<(Entity, &Transform, &Bullet), Without<Meanie>>,
    mut balls: Query<(&Transform, &mut Ball), Without<Meanie>>,
    paddle_comp: Single<(&Transform, &Paddle), Without<Meanie>>,
    mut game: ResMut<Game>,
) {
    let dt = game.avg_delta;
    let (p_tr, paddle) = paddle_comp.into_inner();
    let p_size = PADDLE_SIZES[paddle.variant];

    // Move meanies
    'meanie_loop: for (entity, mut meanie, mut tr, anim) in &mut meanies {
        if tr.translation.y > MEANIES_MAXY {
            tr.translation.y -= dt * meanie.speed;
        } else {
            // Meanie follows a random walk down the screen
            meanie.angle += rand::rng().random_range(-PI / 16.0..=PI / 16.0);
            if meanie.angle < MEANIES_MIN_ANGLE {
                meanie.angle = 2. * MEANIES_MIN_ANGLE - meanie.angle;
            } else if meanie.angle > MEANIES_MAX_ANGLE {
                meanie.angle = 2. * MEANIES_MAX_ANGLE - meanie.angle;
            }

            // Adjust angle if meanie is in collision

            let direction = Vec2::from_angle(meanie.angle);

            // meanies are slow so we don't need to animate pixel by pixel
            tr.translation.y += dt * direction.y * meanie.speed;
            tr.translation.x += dt * direction.x * meanie.speed;
            if tr.translation.x < MEANIES_MINX {
                tr.translation.x = 2. * MEANIES_MINX - tr.translation.x;
                meanie.angle = PI - meanie.angle;
            } else if tr.translation.x > MEANIES_MAXX {
                tr.translation.x = 2. * MEANIES_MAXX - tr.translation.x;
                meanie.angle = -PI - meanie.angle;
            }

            // Bottom border
            if tr.translation.y < MEANIES_MINY {
                commands.entity(entity).despawn();
                game.nmeanies -= 1;
                continue 'meanie_loop;
            }
        }

        // Meanie size and position
        let variant = meanie.variant;
        let frame = anim.current_frame;
        let meanie_size = MEANIES_SIZES[variant][frame];
        let meanie_tr = tr.translation.truncate();

        // Check collision with paddle
        let body = Body::Rectangular(p_tr.translation.truncate(), p_size);
        if let Some(_collides) = collision(body, meanie_tr, meanie_size) {
            meanie_destroy(&mut commands, &tr, entity, &mut game);
            continue 'meanie_loop;
        }

        // Check collision with bullet
        for (bullet_entity, bullet_tr, _) in &mut bullets {
            let body = Body::Rectangular(bullet_tr.translation.truncate(), BULLET_SIZE);
            if let Some(_collides) = collision(body, meanie_tr, meanie_size) {
                meanie_destroy(&mut commands, &tr, entity, &mut game);
                commands.entity(bullet_entity).despawn();
                commands.spawn((
                    AudioPlayer::new(
                        game.sound_bullet_hit
                            .choose(&mut rand::rng())
                            .unwrap()
                            .clone(),
                    ),
                    PlaybackSettings::DESPAWN,
                ));
                continue 'meanie_loop;
            }
        }

        // Check collision with balls
        for (ball_tr, mut ball) in &mut balls {
            // Skip balls caught by the paddle
            if ball.caught {
                continue;
            }
            let body = Body::Round(ball_tr.translation.truncate(), BALL_RADIUS);
            if let Some(_collides) = collision(body, meanie_tr, meanie_size) {
                meanie_destroy(&mut commands, &tr, entity, &mut game);
                ball.direction.y *= -1.;
                continue 'meanie_loop;
            }
        }
    }

    // Spawn new meanie
    if game.nmeanies < MEANIES_MAX && secret_is_unlocked(Secret::Meanies as usize, &game) {
        let spawn_prob: f32 = MEANIES_PER_SECOND * dt;
        if rand::rng().random::<f32>() < spawn_prob {
            let variant: usize = rand::rng().random_range(0..MEANIES_TYPES);
            let portal: usize = rand::rng().random_range(0..2);
            let _new_meanie = commands
                .spawn((
                    Sprite::from_image(game.h_meanies_frames[variant][0].clone()),
                    Transform {
                        translation: Vec3::new(
                            MEANIES_PORTAL_X[portal],
                            MEANIES_PORTAL_Y,
                            LAYER_MEANIES,
                        ),
                        ..default()
                    },
                    Animation {
                        timer: Timer::from_seconds(1.0 / MEANIES_FRAMERATE, TimerMode::Repeating),
                        frozen: false,
                        variant: AnimationType::Repeat,
                        frames: game.h_meanies_frames[variant].clone(),
                        current_frame: 0,
                        reverse: false,
                        velocity: Vec2::new(0., 0.),
                    },
                    Meanie {
                        variant,
                        speed: MEANIES_SPEED,
                        angle: -FRAC_PI_2,
                    },
                    GameTag,
                ))
                .id();

            // let meanie_shadow = commands
            //     .spawn((
            //         Sprite::from_image(game.h_meanies_shadows[variant].clone()),
            //         Transform::from_xyz(SHADOW_DX, SHADOW_DY, LAYER_SHADOWS),
            //     ))
            //     .id();
            // commands.entity(new_ball).add_child(ball_shadow);

            game.nmeanies += 1;
        }
    }
}
