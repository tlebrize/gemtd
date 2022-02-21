use crate::{GRID_SIZE, TILE_SIZE, TILE_SPACER};
use bevy::prelude::*;

pub fn vec2_to_position(vec: Vec2) -> (f32, f32) {
    let (mut x, mut y) = (
        ((vec.x + (GRID_SIZE * TILE_SIZE - TILE_SIZE + TILE_SPACER) / 2.0)
            / (TILE_SIZE + TILE_SPACER))
            .round(),
        (((vec.y * -1.0) + ((GRID_SIZE * TILE_SIZE - TILE_SIZE) / 2.0) + (TILE_SPACER * 1.5))
            / (TILE_SIZE + TILE_SPACER))
            .round(),
    );
    if x == -0.0 {
        x = 0.0
    }
    if y == -0.0 {
        y = 0.0
    }
    (x, y)
}

pub fn fit_to_grid(pos: (f32, f32)) -> Option<(usize, usize)> {
    if 0.0 > pos.0 || 0.0 > pos.1 || pos.0 >= GRID_SIZE || pos.1 >= GRID_SIZE {
        None
    } else {
        Some((pos.0 as usize, pos.1 as usize))
    }
}

pub fn position_to_translation(x: f32, y: f32) -> Vec3 {
    Vec3::new(
        x * TILE_SIZE - (GRID_SIZE * TILE_SIZE / 2.0) + (0.5 * TILE_SIZE) + x * TILE_SPACER
            - TILE_SPACER * 0.5,
        -1.0 * (y * TILE_SIZE - (GRID_SIZE * TILE_SIZE / 2.0)
            + (0.5 * TILE_SIZE)
            + y * TILE_SPACER
            - TILE_SPACER * 1.5),
        1.0,
    )
}

pub fn position_to_transform(x: f32, y: f32) -> Transform {
    Transform::from_translation(position_to_translation(x, y))
}

pub fn flat_distance(t1: Transform, t2: Transform) -> f32 {
    Vec2::new(t1.translation.x, t1.translation.y)
        .distance(Vec2::new(t2.translation.x, t2.translation.y))
}
