use bevy::prelude::*;

pub struct TestSystemChangeTick;

impl Plugin for TestSystemChangeTick {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, test_system_change_tick);
    }
}

use bevy::ecs::system::SystemChangeTick;

fn test_system_change_tick(my_tick: SystemChangeTick) {
    println!("my_tick.this_run: {:#?}", my_tick.this_run());
}
