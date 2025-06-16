use std::time::Duration;

use crate::game::*;
use bevy::prelude::*;

pub enum AnimationType {
    Repeat,
    Despawn,
    Freeze,
}

#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub frames: Vec<Handle<Image>>,
    pub frozen: bool,
    pub variant: AnimationType,
    pub current_frame: usize,
    pub reverse: bool,
    pub velocity: Vec2,
}

pub fn animate(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut Transform, &mut Animation)>,
    game: Res<Game>,
) {
    let dt = game.avg_delta;
    for (anim_entity, mut sprite, mut transform, mut anim) in &mut query {
        // Move sprite
        transform.translation.x += anim.velocity.x * dt;
        transform.translation.y += anim.velocity.y * dt;

        // Animate sprite
        anim.timer.tick(Duration::from_secs_f32(dt));
        if !anim.frozen && anim.timer.just_finished() {
            let mut first = 0;
            let mut last = anim.frames.len() - 1;
            if anim.reverse {
                (first, last) = (last, first);
            }
            if anim.current_frame == last {
                match anim.variant {
                    AnimationType::Repeat => {
                        anim.current_frame = first;
                    }
                    AnimationType::Freeze => {
                        anim.frozen = true;
                        continue;
                    }
                    AnimationType::Despawn => {
                        commands.entity(anim_entity).despawn();
                        continue;
                    }
                }
            } else if !anim.reverse {
                anim.current_frame += 1;
            } else {
                anim.current_frame -= 1;
            }
            *sprite = Sprite::from_image(anim.frames[anim.current_frame].clone());
        }
    }
}
