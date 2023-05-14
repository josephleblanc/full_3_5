use bevy::prelude::*;
use bevy::window::{CursorEntered, CursorLeft, CursorMoved, WindowCreated, WindowResized};

fn print_window_events(
    created: EventReader<WindowCreated>,
    resized: EventReader<WindowResized>,
    cursor_entered: EventReader<CursorEntered>,
    cursor_left: EventReader<CursorLeft>,
    cursor_moved: EventReader<CursorMoved>,
) {
    let mut to_print: Vec<String> = Vec::new();
    if !created.is_empty() {
        to_print.push("WindowCreated".to_string());
    }
    if !resized.is_empty() {
        to_print.push("WindowResized".to_string());
    }
    if !cursor_entered.is_empty() {
        to_print.push("CursorEntered".to_string());
    }
    if !cursor_left.is_empty() {
        to_print.push("CursorLeft".to_string());
    }
    if !cursor_moved.is_empty() {
        to_print.push("CursorMoved".to_string());
    }
    for entry in to_print {
        println!("{}", entry);
    }
}
