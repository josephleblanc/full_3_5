use bevy::prelude::*;

pub fn get_window_size(query: Query<&Window>) -> (f32, f32) {
    match query.get_single() {
        Ok(window) => (window.height(), window.width()),
        Err(_) => {
            panic!(
                "More or less than one window found when trying to \
        get window size"
            );
        }
    }
}
