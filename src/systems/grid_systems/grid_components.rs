use bevy::prelude::*;

// The minimum x-value displayed on the window,
// in reference to a given camera.
#[derive(Component)]
pub struct WindowXMin(f32);

#[derive(Component)]
pub struct WindowYMin(f32);

#[derive(Component)]
pub struct SquareSize(usize);
