use crate::menu::character_creation::components::*;
use bevy::prelude::*;

pub fn display(
    mut query_button: Query<(&mut Style, &mut SubTabButton)>,
    selected_tab: Res<CreationTabSelected>,
) {
    match selected_tab.into_inner().inner() {
        CreationTab::Race => {
            let race_subtabs = RaceTab::array();
            let mut race_iter = race_subtabs.iter();
            for (mut button_style, mut subtab) in query_button.iter_mut() {
                if let Some(race) = race_iter.next() {
                    button_style.display = Display::Flex;
                    *subtab = SubTabButton::Race(*race);
                } else {
                    button_style.display = Display::None;
                }
            }
        }
        CreationTab::Class => {
            let class_subtabs = ClassTab::array();
            let mut class_subtabs_iter = class_subtabs.iter();
            for (mut button_style, mut subtab) in query_button.iter_mut() {
                if let Some(class) = class_subtabs_iter.next() {
                    button_style.display = Display::Flex;
                    *subtab = SubTabButton::Class(*class);
                } else {
                    button_style.display = Display::None;
                }
            }
        }
        _ => {
            for (_i, (mut button_style, _)) in query_button.iter_mut().enumerate() {
                button_style.display = Display::None;
            }
        }
    }
}
