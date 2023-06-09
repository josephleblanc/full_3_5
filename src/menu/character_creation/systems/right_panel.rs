use crate::menu::character_creation::components::{RightPanelContainer, SelectTabEvent};

use bevy::prelude::*;

// Display or hide the right panel depending on the tab selected.
// pub fn display(
//     mut query_panel: Query<&mut Style, RightPanelContainer>,
//     mut tab_event_reader: EventReader<SelectTabEvent>,
// ) {
//     if let Some(tab_event) = tab_event_reader.iter().last() {
//         for (mut style, ) in query_panel.iter_mut() {
//             if list.tab == tab_event.tab {
//                 style.display = Display::Flex;
//             } else {
//                 style.display = Display::None;
//             }
//         }
//     }
// }
