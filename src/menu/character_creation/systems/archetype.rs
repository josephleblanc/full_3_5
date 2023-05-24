use super::super::components::*;
use crate::menu::styles::{SUBTAB_BUTTON_FONT, TEXT_COLOR};
use crate::systems::game::archetype::ArchetypeName;
use crate::systems::game::class::PlayableClass;
use crate::technical::archetype::ArchetypeAsset;
use bevy::prelude::*;

pub fn archetype_panel_title(
    mut query_panel_title: Query<&mut Text, (With<LeftPanelTitle>, With<ArchetypePanel>)>,
    selected_class: Res<SelectedClass>,
    asset_server: Res<AssetServer>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let mut archetypes_iter = archetype_asset
        .iter()
        .filter(|(_handle, archetype)| archetype.class_name == selected_class.inner());
    let mut text = query_panel_title.get_single_mut().unwrap();
    if !archetypes_iter.next().is_none() {
        *text = Text::from_section(
            selected_class.inner().to_string(),
            TextStyle {
                font: font.clone(),
                font_size: SUBTAB_BUTTON_FONT,
                color: TEXT_COLOR,
            },
        );
    } else {
        *text = Text::from_section(
            "Not Implemented Yet",
            TextStyle {
                font: font.clone(),
                font_size: SUBTAB_BUTTON_FONT,
                color: TEXT_COLOR,
            },
        );
    }
}
pub fn archetype_panel_text(
    mut query_list_text: Query<
        (&mut Style, &PlayableClass),
        (
            With<LeftPanelText>,
            With<ArchetypePanel>,
            With<ArchetypeName>,
        ),
    >,
    selected_tab: Res<SelectedClassTab>,
    selected_class: Res<SelectedClass>,
) {
    for (mut style, class) in query_list_text.iter_mut() {
        if *class == selected_class.inner() && selected_tab.inner() == ClassTab::Archetypes {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
pub fn archetype_panel_display(
    mut query_list_node: Query<&mut Style, (With<LeftPanelList>, With<ArchetypePanel>)>,
    selected_tab: Res<SelectedClassTab>,
    creation_tab: Res<CreationTabSelected>,
) {
    if selected_tab.inner() == ClassTab::Archetypes && creation_tab.inner() == CreationTab::Class {
        query_list_node.get_single_mut().unwrap().display = Display::Flex;
    } else {
        query_list_node.get_single_mut().unwrap().display = Display::None;
    }
}

pub fn panel_title(
    mut query_panel_title: Query<&mut Text, (With<LeftPanelTitle>, With<ArchetypePanel>)>,
    selected_class: Res<SelectedClass>,
    asset_server: Res<AssetServer>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let mut archetypes_iter = archetype_asset
        .iter()
        .filter(|(_handle, archetype)| archetype.class_name == selected_class.inner());
    let mut text = query_panel_title.get_single_mut().unwrap();
    if !archetypes_iter.next().is_none() {
        *text = Text::from_section(
            selected_class.inner().to_string(),
            TextStyle {
                font: font.clone(),
                font_size: SUBTAB_BUTTON_FONT,
                color: TEXT_COLOR,
            },
        );
    } else {
        *text = Text::from_section(
            "Not Implemented Yet",
            TextStyle {
                font: font.clone(),
                font_size: SUBTAB_BUTTON_FONT,
                color: TEXT_COLOR,
            },
        );
    }
}
pub fn panel_text(
    mut query_list_text: Query<
        (&mut Style, &PlayableClass),
        (
            With<LeftPanelText>,
            With<ArchetypePanel>,
            With<ArchetypeName>,
        ),
    >,
    selected_tab: Res<SelectedClassTab>,
    selected_class: Res<SelectedClass>,
) {
    for (mut style, class) in query_list_text.iter_mut() {
        if *class == selected_class.inner() && selected_tab.inner() == ClassTab::Archetypes {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
