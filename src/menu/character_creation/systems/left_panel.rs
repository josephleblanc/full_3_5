use crate::menu::character_creation::components::*;
use crate::menu::components::SelectedWrapper;
use crate::menu::styles::*;
use crate::systems::game::character::PlayableRace;
use crate::systems::game::class::PlayableClass;
use bevy::prelude::*;
//////////////////////// Left Button Panel ////////////////////////////////
// Display or hide the left panel
// Only display when the race tab is selected, otherwise hide.

pub fn race_panel(
    mut query_list: Query<&mut Style, (With<LeftPanelList>, With<RacePanel>)>,
    selected_tab: Res<SelectedTab>,
) {
    if selected_tab.into_inner().selected() == Tab::Race {
        query_list.get_single_mut().unwrap().display = Display::Flex;
    } else {
        query_list.get_single_mut().unwrap().display = Display::None;
    }
}
pub fn class_panel(
    mut query_list: Query<&mut Style, (With<LeftPanelList>, With<ClassPanel>)>,
    selected_tab: Res<SelectedTab>,
) {
    if selected_tab.selected() == Tab::Class {
        query_list.get_single_mut().unwrap().display = Display::Flex;
    } else {
        query_list.get_single_mut().unwrap().display = Display::None;
    }
}
pub fn archetype_panel(
    mut query_list: Query<&mut Style, (With<LeftPanelList>, With<ArchetypePanel>)>,
    selected_tab: Res<SelectedTab>,
    selected_class_tab: Res<SelectedClassTab>,
) {
    if selected_tab.selected() == Tab::Class && selected_class_tab.inner() == ClassTab::Archetypes {
        query_list.get_single_mut().unwrap().display = Display::Flex;
    } else {
        query_list.get_single_mut().unwrap().display = Display::None;
    }
}

// Makes sure other race buttons are the default color.
pub fn cleanup_selected_race_description_button(
    query_change: Query<(&Interaction, &RaceTab)>,
    mut query_others: Query<(&Interaction, &mut BackgroundColor, &RaceTab)>,
    selected_race: Res<SelectedRaceTab>,
) {
    if !query_change.is_empty() {
        for (interaction, mut color, button_type) in query_others.iter_mut() {
            if *interaction == Interaction::None && *button_type != selected_race.inner() {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn selected_race_description_type(
    mut selected: ResMut<SelectedRaceTab>,
    mut interaction_query: Query<
        (&Interaction, &RaceTab, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    let selection_copy = selected.inner();
    for (interaction, interacted_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected = SelectedRaceTab(*interacted_button);
                }
            }
            Interaction::Hovered => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// The systems managing the Left Panel of character creation, used to display
// things like the list of available races when the race tab is selected,
// or the list of available classes when the class tab is selected.
// This should be scrollable, and should load most of its assets from custom
// asset loaders.

pub fn set_list_text(
    mut query_list_text: Query<&mut Text, With<LeftPanelText>>,
    mut query_list_button: Query<(&mut Style, &mut LeftPanelEnum), With<LeftPanelButton>>,
    selected_tab: Res<SelectedTab>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    // system should have a conditional to run when SelectedTab changes.
    let left_enums: Option<Vec<LeftPanelEnum>> = match selected_tab.selected() {
        Tab::Race => Some(
            PlayableRace::iterator()
                .map(|race| LeftPanelEnum::Race(race))
                .collect(),
        ),
        Tab::Class => Some(
            PlayableClass::iterator()
                .map(|class| LeftPanelEnum::Class(class))
                .collect(),
        ),
        _ => None,
    };
    if let Some(left_enums) = left_enums {
        let mut left_iter = left_enums.iter();
        for ((mut button_style, mut button_enum), mut text) in
            query_list_button.iter_mut().zip(query_list_text.iter_mut())
        {
            if let Some(left_enum) = left_iter.next() {
                button_style.display = Display::Flex;
                *button_enum = *left_enum;
                println!("button_enum is now: {:?}", button_enum);
                *text = Text::from_section(
                    left_enum.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: LEFT_PANEL_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                );
            } else {
                button_style.display = Display::None;
            }
        }
    }
}
// Makes sure other left panel buttons are the default color when not selected.
pub fn cleanup_buttons(
    query_change: Query<&LeftPanelButton, Changed<Interaction>>,
    mut query_others: Query<(
        &LeftPanelEnum,
        &Interaction,
        &mut BackgroundColor,
        &LeftPanelButton,
    )>,
    selected_race: Res<SelectedRace>,
    selected_class: Res<SelectedClass>,
) {
    if !query_change.is_empty() {
        for (player_race, interaction, mut color, _) in query_others.iter_mut() {
            if let Some(player_race) = player_race.get_race() {
                if *interaction == Interaction::None && player_race != selected_race.0 {
                    *color = RACE_BUTTON_COLOR.into();
                }
            } else if let Some(class) = player_race.get_class() {
                if *interaction == Interaction::None && class != selected_class.inner() {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}
pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &LeftPanelEnum, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_race: ResMut<SelectedRace>,
    mut selected_class: ResMut<SelectedClass>,
) {
    let selection_copy = selected_race.inner();
    for (interaction, left_enum, mut color) in &mut interaction_query {
        if let Some(player_race) = left_enum.get_race() {
            match *interaction {
                Interaction::Clicked => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected_race = SelectedRace(player_race);
                    }
                }
                Interaction::Hovered => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        } else if let Some(player_class) = left_enum.get_class() {
            match *interaction {
                Interaction::Clicked => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected_class = SelectedClass(player_class);
                        println!("button left_enum changed to: {:?}", player_class);
                    }
                }
                Interaction::Hovered => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        }
    }
}

pub fn cleanup_race_description_type_button(
    query_change: Query<&RaceTab, Changed<Interaction>>,
    mut query_others: Query<(&RaceTab, &Interaction, &mut BackgroundColor)>,
    selected_description_type: Res<SelectedRaceTab>,
) {
    if !query_change.is_empty() {
        for (description_button, interaction, mut color) in query_others.iter_mut() {
            if *interaction == Interaction::None
                && *description_button != selected_description_type.inner()
            {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}
