use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

// Simple setup for camera with custom component for easy query
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        Name::new("My main camera"),
    ));
}
