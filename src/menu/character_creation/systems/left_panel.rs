use crate::menu::character_creation::components::*;
use crate::menu::components::SelectedWrapper;
use crate::menu::styles::*;
use crate::systems::game::archetype::ArchetypeName;
use crate::systems::game::character::PlayableRace;
use crate::systems::game::class::PlayableClass;
use bevy::prelude::*;
//////////////////////// Left Button Panel ////////////////////////////////
// Display or hide the left panel
// Only display when the race tab is selected, otherwise hide.

/// Display the panel when a tab or subtab event is received.
/// Should run after the tab and subtab events
pub fn panel_recv_tab_display(
    mut query_panel: Query<(&mut Style, &LeftPanelList)>,
    mut tab_event_reader: EventReader<SelectTabEvent>,
    mut subtab_event_reader: EventReader<SelectSubTabEvent>,
) {
    if let Some(tab_event) = tab_event_reader.iter().last() {
        for (mut style, list) in query_panel.iter_mut() {
            if list.tab == tab_event.tab {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    }
    if let Some(subtab_event) = subtab_event_reader.iter().last() {
        for (mut style, list) in query_panel.iter_mut() {
            if list.tab == subtab_event.tab {
                if list.subtab.is_none() || list.subtab.unwrap() == subtab_event.subtab {
                    style.display = Display::Flex;
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}

/// Makes sure other unselected race buttons are the default color.
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

/// Detects interaction on left panel, then changes selected race and button color
/// for left panel
pub fn selected_race(
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

/// Makes sure other left panel buttons are the default color when not selected.
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
    mut left_panel_event: EventWriter<LeftPanelEvent>,
) {
    let selection_copy = selected_race.inner();
    for (interaction, left_enum, mut color) in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            left_panel_event.send(Into::<LeftPanelEvent>::into(*left_enum));
        }
    }
}

// TODO: Make the buttons change color when hovered or selected,
// maybe upon receiving an event.

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
