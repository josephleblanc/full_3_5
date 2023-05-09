// Test whether events are received deterministically or not across systems.
use bevy::prelude::*;

pub struct TestEvents;

impl Plugin for TestEvents {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestEventsTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_event::<TestEvent>()
        .add_system(my_system_send_event)
        .add_system(my_system_recv_event_one)
        .add_system(my_system_recv_event_two.before(my_system_send_event));
    }
}

#[derive(Resource)]
struct TestEventsTimer(Timer);

pub struct TestEvent(usize); // custom event type

fn my_system_send_event(
    mut writer: EventWriter<TestEvent>,
    time: Res<Time>,
    mut timer: ResMut<TestEventsTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        writer.send(TestEvent(7));
        println!("sending TestEvent(7)");
    }
}

fn my_system_recv_event_one(mut reader: EventReader<TestEvent>) {
    for event in reader.iter() {
        println!("receiver 1 gets event: value = {}", event.0);
    }
}

fn my_system_recv_event_two(mut reader: EventReader<TestEvent>) {
    for event in reader.iter() {
        println!("receiver 2 gets event: value = {}", event.0);
    }
    println!("recv2 count: {}", reader.iter().count());
}
