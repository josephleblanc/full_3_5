use crate::constants::PATH_SIMPLE_FONT;
use crate::menu::character_creation::components::*;
use crate::menu::styles::{SUBTAB_BUTTON_FONT, TEXT_COLOR};
use bevy::prelude::*;

pub fn display(
    mut query_button: Query<(&mut Style, &mut SubTabButton)>,
    selected_tab: Res<SelectedCreationTab>,
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

pub fn text(
    mut query_button_text: Query<&mut Text, With<SubTabButtonText>>,
    selected_tab: Res<SelectedCreationTab>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(PATH_SIMPLE_FONT);
    match selected_tab.into_inner().inner() {
        CreationTab::Race => {
            let subtabs = RaceTab::array();
            let subtabs_iter = subtabs.iter();
            for (mut subtab_text, subtab) in query_button_text.iter_mut().zip(subtabs_iter) {
                *subtab_text = Text::from_section(
                    subtab.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: SUBTAB_BUTTON_FONT,
                        color: TEXT_COLOR,
                    },
                );
            }
        }
        CreationTab::Class => {
            let subtabs = ClassTab::array();
            let subtabs_iter = subtabs.iter();
            for (mut subtab_text, subtab) in query_button_text.iter_mut().zip(subtabs_iter) {
                *subtab_text = Text::from_section(
                    subtab.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: SUBTAB_BUTTON_FONT,
                        color: TEXT_COLOR,
                    },
                );
            }
        }
        _ => {}
    }
}
