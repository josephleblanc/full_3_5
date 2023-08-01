use bevy::prelude::*;

pub struct TestSystemWorldRef;

impl Plugin for TestSystemWorldRef {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, my_system_ref_world);
    }
}

pub fn my_system_ref_world(world: &World) {
    println!("world_id is {:?}", world.id());
}
