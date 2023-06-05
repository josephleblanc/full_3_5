use crate::menu::components::SelectedWrapper;
use crate::menu::{
    character_creation::components::*,
    styles::{RACE_BUTTON_COLOR, RACE_BUTTON_COLOR_HOVERED, RACE_BUTTON_COLOR_SELECTED},
};

use bevy::prelude::*;
// TODO: change the below functions to work with generics, so they can be used in other menu
// items outside the context of character creation

// change display of list container upon event, both on entering and exiting a tab
pub fn new_display_tab_list(
    mut tab_event_reader: EventReader<SelectTabEvent>,
    mut query_list_parent: Query<(&mut Style, &TabListParent)>,
) {
    for tab_event in tab_event_reader.iter() {
        for (mut style, list_parent) in query_list_parent.iter_mut() {
            if tab_event.tab == Into::<Tab>::into(*list_parent) {
                match tab_event.tab_state {
                    InTab::Exiting => {
                        style.display = Display::None;
                    }
                    InTab::Entering => {
                        style.display = Display::Flex;
                    }
                }
            }
        }
    }
}

pub fn debug_new_display_tab_list(
    mut tab_event_reader: EventReader<SelectTabEvent>,
    query_list_parent: Query<(&mut Style, &TabListParent)>,
) {
    for tab_event in tab_event_reader.iter() {
        for (_, list_parent) in &query_list_parent {
            if tab_event.tab == Into::<Tab>::into(*list_parent) {
                match tab_event.tab_state {
                    InTab::Exiting => {
                        println!("received event: Exiting tab {:#?}", tab_event)
                    }
                    InTab::Entering => {
                        println!("received event: Entering tab {:#?}", tab_event)
                    }
                }
            }
        }
    }
}

// change display of subtab list container upon event, both on entering and exiting a tab
pub fn new_display_subtab_list(
    mut subtab_event_reader: EventReader<SelectSubTabEvent>,
    mut query_subtab_list_parent: Query<(&mut Style, &SubTabListParent), Without<TabListParent>>,
) {
    for subtab_event in subtab_event_reader.iter() {
        for (mut style, subtab_list_parent) in &mut query_subtab_list_parent {
            println!(
                "new_display_subtab_list queried: tab = {}, subtab = {}",
                subtab_list_parent.tab, subtab_list_parent.subtab
            );
            if subtab_list_parent.subtab == subtab_event.subtab {
                println!(
                    "--> match found: subtab list parent = {}, subtab event = {}",
                    subtab_list_parent.subtab, subtab_event.subtab,
                );
                match subtab_event.tab_state {
                    InTab::Exiting => {
                        style.display = Display::None;
                    }
                    InTab::Entering => {
                        style.display = Display::Flex;
                    }
                }
            }
        }
    }
}

// change display of subtab list container upon event, both on entering and exiting a tab
pub fn debug_new_display_subtab_list(
    mut subtab_event_reader: EventReader<SelectSubTabEvent>,
    mut query_subtab_list_parent: Query<(&SubTabListParent), Without<TabListParent>>,
) {
    if let Some(subtab_event) = subtab_event_reader.iter().last() {
        for (subtab_list_parent) in &mut query_subtab_list_parent {
            if subtab_list_parent.subtab == subtab_event.subtab {
                match subtab_event.tab_state {
                    InTab::Exiting => {
                        println!("received event: Exiting subtab {:#?}", subtab_event);
                    }
                    InTab::Entering => {
                        println!("received event: Entering subtab {:#?}", subtab_event);
                    }
                }
            }
        }
    }
}

// change tab button color depending on interaction, but does not change the selected item,
// which is handled in another function below that uses events
pub fn tab_button_color(
    mut interaction_query: Query<(&mut BackgroundColor, &Interaction, &Tab), With<Button>>,
    selected: Res<SelectedTab>,
) {
    for (mut background_color, interaction, tab) in interaction_query.iter_mut() {
        if (*selected).selected() != *tab {
            match *interaction {
                Interaction::Clicked => {
                    *background_color = RACE_BUTTON_COLOR_SELECTED.into();
                }
                Interaction::Hovered => {
                    *background_color = RACE_BUTTON_COLOR_HOVERED.into();
                }
                Interaction::None => {
                    *background_color = RACE_BUTTON_COLOR.into();
                }
            }
        } else {
            *background_color = RACE_BUTTON_COLOR_SELECTED.into();
        }
    }
}

// change subtab button color depending on interaction, but does not change the selected item,
// which is handled in another function below that uses events
pub fn subtab_button_color(
    mut interaction_query: Query<(&mut BackgroundColor, &Interaction, &SubTab), With<Button>>,
    selected: Res<SelectedSubTab>,
) {
    for (mut background_color, interaction, subtab) in interaction_query.iter_mut() {
        if selected.selected() != *subtab {
            match *interaction {
                Interaction::Clicked => {
                    if (*selected).selected() == *subtab {
                        *background_color = RACE_BUTTON_COLOR_SELECTED.into();
                    }
                }
                Interaction::Hovered => {
                    if (*selected).selected() != *subtab {
                        *background_color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if (*selected).selected() != *subtab {
                        *background_color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        } else {
            *background_color = RACE_BUTTON_COLOR_SELECTED.into();
        }
    }
}

// Broadcast SelectTabEvent so lists may change visibility, and change the selected tab to reflect
// the new choice.
pub fn tab_button_select(
    mut interaction_query: Query<
        (&Interaction, Entity, &Tab),
        (Changed<Interaction>, With<Button>),
    >,
    mut tab_event_writer: EventWriter<SelectTabEvent>,
    mut selected: ResMut<SelectedTab>,
) {
    if let Some((interaction, entity, &tab)) = interaction_query.iter_mut().last() {
        if selected.selected() != tab {
            match *interaction {
                Interaction::Clicked => {
                    println!(
                        "sending event: {:#?}",
                        SelectTabEvent {
                            entity,
                            tab: selected.selected(),
                            tab_state: InTab::Exiting,
                        }
                    );
                    tab_event_writer.send(SelectTabEvent {
                        entity,
                        tab: selected.selected(),
                        tab_state: InTab::Exiting,
                    });
                    println!("changing selected tab from {}", selected.0);
                    selected.0 = tab;
                    println!("to {:->10}", selected.0);
                    tab_event_writer.send(SelectTabEvent {
                        entity,
                        tab: selected.selected(),
                        tab_state: InTab::Entering,
                    });
                    println!(
                        "sending event: {:#?}",
                        SelectTabEvent {
                            entity,
                            tab: selected.selected(),
                            tab_state: InTab::Entering,
                        }
                    );
                }
                _ => {}
            }
        }
    }
}

// Broadcast SelectSubTabEvent so lists may change visibility, and change the selected tab to reflect
// the new choice.
pub fn subtab_button_select(
    mut interaction_query: Query<
        (&Interaction, Entity, &SubTab),
        (Changed<Interaction>, With<Button>),
    >,
    mut subtab_event_writer: EventWriter<SelectSubTabEvent>,
    mut selected: ResMut<SelectedSubTab>,
) {
    if let Some((interaction, entity, &subtab)) = interaction_query.iter_mut().last() {
        if selected.selected() != subtab {
            match *interaction {
                Interaction::Clicked => {
                    subtab_event_writer.send(SelectSubTabEvent {
                        entity,
                        tab: subtab.into(),
                        subtab: selected.selected(),
                        tab_state: InTab::Exiting,
                    });
                    println!(
                        "sending event: {:#?}",
                        SelectSubTabEvent {
                            entity,
                            tab: subtab.into(),
                            subtab: selected.selected(),
                            tab_state: InTab::Exiting,
                        }
                    );
                    println!("changing selected subtab from {}", selected.0);
                    selected.0 = subtab;
                    println!("to {:->10}", selected.0);
                    println!(
                        "sending event: {:#?}",
                        SelectSubTabEvent {
                            entity,
                            tab: subtab.into(),
                            subtab: selected.selected(),
                            tab_state: InTab::Entering,
                        }
                    );
                    subtab_event_writer.send(SelectSubTabEvent {
                        entity,
                        tab: subtab.into(),
                        subtab: selected.selected(),
                        tab_state: InTab::Entering,
                    });
                }
                _ => {}
            }
        }
    }
}
