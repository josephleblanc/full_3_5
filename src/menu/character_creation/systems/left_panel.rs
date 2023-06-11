use crate::menu::character_creation::components::*;
use crate::menu::components::SelectedWrapper;
use crate::menu::styles::*;
use bevy::prelude::*;
//////////////////////// Left Button Panel ////////////////////////////////
/// These functions run as systems for the left panel in character creation,
/// providing functionality to the buttons which allow selection of race, class,
/// archetype, (more later).
///
/// It is necessary to put these functions into system sets to ensure the event producers run
/// before the event consumers. They should be organized like so:
/// app
///     .configure_sets((
///         SenderSet,
///         ReceiverSet.before(SenderSet)
///     ))
///     .add_systems((
///         button_event::<PlayableRace, SelectedRace>,
///         button_event::<PlayableClass, SelectedClass>,
///         /* other sending sets here */
///     ).in_set(SenderSet))
///     .add_systems((
///         panel_recv_tab_display,
///         select_race,
///         select_class,
///         select_archetype,
///     ).in_set(ReceiverSet))

/// Display the panel when a tab or subtab event is received.
/// Should run after the tab and subtab events
pub fn panel_recv_tab_display(
    mut query_panel: Query<(&mut Style, &Panel)>,
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
                    if list.excluded_subtab.is_none() {
                        style.display = Display::Flex;
                    } else if list.excluded_subtab.unwrap() == subtab_event.subtab {
                        style.display = Display::None;
                    }
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}

pub fn display_nodes() {}

/// Detects interaction on left panel, then changes button color for left panel
pub fn button_color(
    selected_race: Res<SelectedRace>,
    selected_class: Res<SelectedClass>,
    selected_archetype: Res<SelectedArchetype>,
    mut interaction_query: Query<
        (
            &Interaction,
            &LeftPanelEnum,
            &LeftPanelButton,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
) {
    let mut matches = false;
    for (interaction, left_enum, _, mut color) in &mut interaction_query {
        match left_enum {
            LeftPanelEnum::Race(race) => {
                if selected_race.inner() == *race {
                    matches = true;
                }
            }
            LeftPanelEnum::Class(class) => {
                if selected_class.inner() == *class {
                    matches = true;
                }
            }
            LeftPanelEnum::Archetype(archetype) => {
                if selected_archetype.inner() == *archetype {
                    matches = true;
                }
            }
        };
        if !matches {
            match *interaction {
                Interaction::Clicked => {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                }
                Interaction::Hovered => {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
                Interaction::None => {
                    *color = RACE_BUTTON_COLOR.into();
                }
                _ => (),
            }
        };
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
    selected_archetype: Res<SelectedArchetype>,
) {
    if !query_change.is_empty() {
        for (button, interaction, mut color, _) in query_others.iter_mut() {
            if let Some(race) = button.get_race() {
                if *interaction == Interaction::None && race != selected_race.0 {
                    *color = RACE_BUTTON_COLOR.into();
                }
            } else if let Some(class) = button.get_class() {
                if *interaction == Interaction::None && class != selected_class.inner() {
                    *color = RACE_BUTTON_COLOR.into();
                }
            } else if let Some(archetype) = button.get_archetype() {
                if *interaction == Interaction::None && archetype != selected_archetype.inner() {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// TODO: Make the buttons change color when hovered or selected,
// maybe upon receiving an event.
pub fn button_event<S, T>(
    interaction_query: Query<(&Interaction, &LeftPanelEnum), Changed<Interaction>>,
    mut left_panel_event: EventWriter<LeftPanelEvent>,
    selected: Res<T>,
) where
    S: Copy + Clone + Component + Eq + Into<LeftPanelEvent>,
    T: SelectedWrapper<S> + Resource,
{
    for (interaction, left_enum) in &interaction_query {
        if *interaction == Interaction::Clicked {
            let exit_event: LeftPanelEvent = selected.selected().into();
            info!("sending event {exit_event:#?}");
            left_panel_event.send(exit_event.set_status(Status::Exiting));

            let event: LeftPanelEvent = (*left_enum).into();
            info!("sending event {event:#?}");
            left_panel_event.send(event.set_status(Status::Entering));
        }
    }
}

pub fn select_race(
    mut event_reader: EventReader<LeftPanelEvent>,
    mut selected_race: ResMut<SelectedRace>,
) {
    if let Some(event) = event_reader.iter().last() {
        if event.status.unwrap() == Status::Entering {
            if let Some(race) = event.race {
                let start_string =
                    format!("SelectedRace changing from {} to ", selected_race.inner());
                selected_race.set(race);
                println!("{start_string} {}", race);
            }
        }
    }
}
pub fn select_class(
    mut event_reader: EventReader<LeftPanelEvent>,
    mut selected_class: ResMut<SelectedClass>,
) {
    for event in event_reader.iter() {
        if event.status.unwrap() == Status::Entering {
            if let Some(class) = event.class {
                selected_class.set(class);
            }
        }
    }
}
pub fn select_archetype(
    mut event_reader: EventReader<LeftPanelEvent>,
    mut selected_archetype: ResMut<SelectedArchetype>,
) {
    for event in event_reader.iter() {
        if event.status.unwrap() == Status::Entering {
            if let Some(archetype) = event.archetype {
                selected_archetype.set(archetype);
            }
        }
    }
}
