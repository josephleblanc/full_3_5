use bevy::prelude::*;

// Test the order in which systems run.

// Order the systems using .before() and .after()
pub struct TestSystemOrderMethodOne;

impl Plugin for TestSystemOrderMethodOne {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(PrintTracker {
                first: false,
                second: false,
                third: false,
            })
            .add_system(print_1st.before(print_2nd))
            .add_system(print_2nd)
            .add_system(print_3rd.after(print_2nd));
    }
}

// Order the systems using .chain()
pub struct TestSystemOrderMethodTwo;

impl Plugin for TestSystemOrderMethodTwo {
    fn build(&self, app: &mut App) {
        app.insert_resource(TestTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(PrintTracker {
                first: false,
                second: false,
                third: false,
            })
            .add_systems((print_1st, print_2nd, print_3rd).chain());
    }
}

#[derive(Resource)]
struct TestTimer(Timer);

#[derive(Resource)]
struct PrintTracker {
    first: bool,
    second: bool,
    third: bool,
}

fn print_1st(time: Res<Time>, mut timer: ResMut<TestTimer>, mut is_print: ResMut<PrintTracker>) {
    if timer.0.tick(time.delta()).just_finished() && !is_print.first {
        is_print.first = true;
        println!("first");
    }
}

fn print_2nd(mut is_print: ResMut<PrintTracker>) {
    if !is_print.second && is_print.first {
        is_print.second = true;
        println!("second");
    }
}

fn print_3rd(mut is_print: ResMut<PrintTracker>) {
    if !is_print.third && (true, true) == (is_print.first, is_print.second) {
        (is_print.first, is_print.second, is_print.third) = (false, false, false);
        println!("third");
    }
}
