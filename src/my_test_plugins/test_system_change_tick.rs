use bevy::prelude::*;

pub struct TestSystemChangeTick;

impl Plugin for TestSystemChangeTick {
    fn build(&self, app: &mut App) {
        app.add_system(test_system_change_tick);
    }
}

use bevy::ecs::system::SystemChangeTick;

fn test_system_change_tick(my_tick: SystemChangeTick) {
    println!("change_tick: {}", my_tick.change_tick());
}
