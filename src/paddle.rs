use crate::balls_and_bullets::*;
use crate::consts::*;
use crate::game::*;
use crate::*;
use bevy::audio::*;
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
/// Component for the player-controlled paddle
pub struct Paddle {
    /// Current paddle variant/type (affects abilities)
    pub variant: usize,
    /// Timer for gun paddle shooting cooldown
    pub gun_timer: Timer,
}

#[derive(Component)]
/// Component for the paddle's shadow visual effect
pub struct PaddleShadow {}

/// System that handles paddle movement and control
/// Processes keyboard input to move the paddle and fire bullets
pub fn paddle_update(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    paddle_comp: Single<(&mut Transform, &mut Paddle)>,
    mut ball_query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    mut game: ResMut<Game>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut paddle_tr, mut paddle) = paddle_comp.into_inner();
    let dt = game.avg_delta;

    // Paddle direction
    let mut dir = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        dir -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        dir += 1.0;
    }

    // Speed
    let paddle_speed = if input.pressed(KeyCode::ShiftLeft) {
        PADDLE_MAX_SPEED
    } else {
        PADDLE_MIN_SPEED
    };

    // Move paddle
    let mut delta_x = 0.0;
    if dir != 0.0 {
        let mut new_x = paddle_tr.translation.x + dir * paddle_speed * dt;
        let min_x = BALLAREA_MINX + PADDLE_SIZES[paddle.variant].x / 2.0;
        let mut max_x = BALLAREA_MAXX - PADDLE_SIZES[paddle.variant].x / 2.0;

        // Change xmax and possibly level if portal is open
        if game.portal_open {
            max_x += 20.;
            if new_x > max_x {
                game.current_level += 1;
                game_state.set(GameState::Transition);
                return;
            }
        }

        new_x = new_x.clamp(min_x, max_x);
        delta_x = new_x - paddle_tr.translation.x;
        paddle_tr.translation.x = new_x;
    }

    // Move and/or release catched balls
    let uncatch = input.pressed(KeyCode::Space);
    for (mut transform, mut ball) in &mut ball_query {
        if ball.caught {
            transform.translation.x += delta_x;
            ball.caught = !uncatch;
        }
    }

    // Fire Gun
    paddle.gun_timer.tick(Duration::from_secs_f32(dt));
    if paddle.variant == PADDLE_GUN && input.pressed(KeyCode::Space) && paddle.gun_timer.finished()
    {
        let mut left_bullet_tr: Transform = *paddle_tr;
        left_bullet_tr.translation.x += GUN_LEFT_X;
        left_bullet_tr.translation.y += GUN_Y;
        left_bullet_tr.translation.z = LAYER_GUN;
        commands.spawn((
            Sprite::from_image(game.h_bullet.clone()),
            left_bullet_tr,
            Bullet,
            GameTag,
        ));
        commands.spawn((
            AudioPlayer::new(game.sound_fire_bullet.clone()),
            PlaybackSettings::DESPAWN,
        ));

        let mut right_bullet_tr: Transform = *paddle_tr;
        right_bullet_tr.translation.x += GUN_RIGHT_X;
        right_bullet_tr.translation.y += GUN_Y;
        right_bullet_tr.translation.z = LAYER_GUN;
        commands.spawn((
            Sprite::from_image(game.h_bullet.clone()),
            right_bullet_tr,
            Bullet,
            GameTag,
        ));
        paddle.gun_timer.reset()
    }
}


