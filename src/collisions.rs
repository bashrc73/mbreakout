use crate::animation::*;
use crate::barrels::*;
use crate::consts::*;
use crate::game::*;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::math::*;
use bevy::prelude::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Copy, Clone)]
pub enum Body {
    Rectangular(Vec2, Vec2),
    Round(Vec2, f32),
}

/// Converts game world coordinates to grid row/column coordinates
pub fn game_xy_to_rc(x: f32, y: f32) -> (usize, usize) {
    (
        ((BALLAREA_MAXY - y) / BRICK_HEIGHT - 0.5) as usize,
        ((x - BALLAREA_MINX) / BRICK_WIDTH - 0.5) as usize,
    )
}

/// Converts grid row/column coordinates to game world coordinates
pub fn game_rc_to_xy(r: usize, c: usize) -> (f32, f32) {
    (
        BALLAREA_MINX + (c as f32 + 0.5) * BRICK_WIDTH,
        BALLAREA_MAXY - (r as f32 + 0.5) * BRICK_HEIGHT,
    )
}

/// Converts grid row/column coordinates to a linear index in the grid array
pub fn game_rc_to_idx(r: usize, c: usize) -> usize {
    r * GRID_COLS + c
}

pub fn collision(body: Body, obst_pos: Vec2, obst_size: Vec2) -> Option<Collision> {
    let obst = Aabb2d::new(obst_pos, obst_size / 2.0);
    let side: Collision;

    match body {
        Body::Rectangular(body_pos, body_size) => {
            let body_r = Aabb2d::new(body_pos, body_size / 2.0);
            if !body_r.intersects(&obst) {
                return None;
            }

            // Calculate overlap in both axes
            let x_overlap = (body_r.max.x - body_r.min.x + obst.max.x - obst.min.x) / 2.0
                - (body_pos.x - obst_pos.x).abs();
            let y_overlap = (body_r.max.y - body_r.min.y + obst.max.y - obst.min.y) / 2.0
                - (body_pos.y - obst_pos.y).abs();

            // The smaller overlap indicates the collision side
            if x_overlap < y_overlap {
                side = if body_pos.x < obst_pos.x {
                    Collision::Left
                } else {
                    Collision::Right
                };
            } else {
                side = if body_pos.y < obst_pos.y {
                    Collision::Bottom
                } else {
                    Collision::Top
                };
            }
        }
        Body::Round(body_pos, body_radius) => {
            let circ = BoundingCircle::new(body_pos, body_radius);
            if !circ.intersects(&obst) {
                return None;
            }

            // Calculate the direction from the closest point to the circle center
            let closest_point = obst.closest_point(body_pos);
            let dx = body_pos.x - closest_point.x;
            let dy = body_pos.y - closest_point.y;

            // Determine which side based on the largest component
            if dx.abs() > dy.abs() {
                side = if body_pos.x < closest_point.x {
                    Collision::Left
                } else {
                    Collision::Right
                };
            } else {
                side = if body_pos.y < closest_point.y {
                    Collision::Bottom
                } else {
                    Collision::Top
                };
            }
        }
    }
    Some(side)
}

pub fn collision_with_bricks(
    commands: &mut Commands,
    body: Body,
    body_direction: Vec2,
    bricks: &mut Query<&mut Brick>,
    game: &mut Game,
) -> Vec<Collision> {
    let mut collisions: Vec<Collision> = Vec::new();
    let body_pos = match body {
        Body::Rectangular(pos, _) => pos,
        Body::Round(pos, _) => pos,
    };

    // Check collisions with bricks in the vicinity of the ball
    let (body_r, body_c) = game_xy_to_rc(body_pos.x, body_pos.y);
    let min_row = if body_r > 0 { body_r - 1 } else { 0 };
    let max_row = (body_r + 1).min(GRID_ROWS - 1);
    let min_col = if body_c > 0 { body_c - 1 } else { 0 };
    let max_col = (body_c + 1).min(GRID_COLS - 1);

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            // Get brick at position
            let grid_idx: usize = game_rc_to_idx(r, c);

            // Skip loop if no brick at this position
            let Some(brick_entity) = game.grid[grid_idx] else {
                continue;
            };

            // Check collision
            let brick_pos: Vec2 = game_rc_to_xy(r, c).into();
            let Some(side) = collision(body, brick_pos, BRICK_SIZE) else {
                continue;
            };

            if (side == Collision::Left && body_direction.x <= 0.)
                || (side == Collision::Right && body_direction.x >= 0.)
                || (side == Collision::Bottom && body_direction.y <= 0.)
                || (side == Collision::Top && body_direction.y >= 0.)
            {
                continue;
            }
            collisions.push(side);

            let Ok(mut b) = bricks.get_mut(brick_entity) else {
                continue;
            };

            commands.spawn((
                Sprite::from_image(game.h_brick_frames[b.variant][0].clone()),
                Transform::from_xyz(brick_pos.x, brick_pos.y, LAYER_EXPLOSIONS),
                Animation {
                    timer: Timer::from_seconds(1.0 / BRICK_FRAMERATE, TimerMode::Repeating),
                    frozen: false,
                    variant: AnimationType::Despawn,
                    frames: game.h_brick_frames[b.variant].clone(),
                    current_frame: 0,
                    reverse: false,
                    velocity: Vec2::ZERO,
                },
                GameTag,
            ));

            if b.variant < 12 || b.variant == 14 {
                // Eliminate old brick
                commands.entity(brick_entity).despawn();
                game.grid[grid_idx] = None;
                game.bricks_left -= 1;

                // Extra life if all bricks destroyed
                if game.bricks_left == 0 {
                    game.nlives += 1;
                }

                // Randomly spawn barrels upon impact
                barrels_spawn(commands, Vec3::new(brick_pos.x, brick_pos.y, 0.), game);
            } else if b.variant == 12 {
                b.variant = 11;
            }
        }
    }
    collisions
}
