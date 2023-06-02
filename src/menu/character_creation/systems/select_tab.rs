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
    mut query_list_parent: Query<(&mut Style, &ListParent)>,
) {
    for tab_event in tab_event_reader.iter() {
        for (mut style, list_parent) in &mut query_list_parent {
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

// change display of subtab list container upon event, both on entering and exiting a tab
pub fn new_display_subtab_list(
    mut subtab_event_reader: EventReader<SelectSubTabEvent>,
    mut query_subtab_list_parent: Query<(&mut Style, &SubTabListParent), Without<ListParent>>,
) {
    for subtab_event in subtab_event_reader.iter() {
        for (mut style, subtab_list_parent) in &mut query_subtab_list_parent {
            if subtab_list_parent.subtab == subtab_event.subtab {
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

// change tab button color depending on interaction, but does not change the selected item,
// which is handled in another function below that uses events
pub fn tab_button_color(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction, &Tab),
        (Changed<Interaction>, With<Button>),
    >,
    selected: Res<SelectedTab>,
) {
    for (mut background_color, interaction, tab) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if (*selected).selected() == *tab {
                    *background_color = RACE_BUTTON_COLOR_SELECTED.into();
                }
            }
            Interaction::Hovered => {
                if (*selected).selected() != *tab {
                    *background_color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if (*selected).selected() != *tab {
                    *background_color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// change subtab button color depending on interaction, but does not change the selected item,
// which is handled in another function below that uses events
pub fn subtab_button_color(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction, &SubTab),
        (Changed<Interaction>, With<Button>),
    >,
    selected: Res<SelectedSubTab>,
) {
    for (mut background_color, interaction, subtab) in &mut interaction_query {
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
    for (interaction, entity, &tab) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                tab_event_writer.send(SelectTabEvent {
                    entity,
                    tab,
                    tab_state: InTab::Entering,
                });
                selected.0 = tab;
                tab_event_writer.send(SelectTabEvent {
                    entity,
                    tab,
                    tab_state: InTab::Exiting,
                });
            }
            _ => {}
        }
    }
}

// Broadcast SelectSubTabEvent so lists may change visibility, and change the selected tab to reflect
// the new choice.
pub fn subtab_button_select(
    mut interaction_query: Query<
        (&Interaction, Entity, &SubTabListParent),
        (Changed<Interaction>, With<Button>),
    >,
    mut subtab_event_writer: EventWriter<SelectSubTabEvent>,
    mut selected: ResMut<SelectedSubTab>,
) {
    for (interaction, entity, &subtab) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                subtab_event_writer.send(SelectSubTabEvent {
                    entity,
                    tab: subtab.tab,
                    subtab: subtab.subtab,
                    tab_state: InTab::Exiting,
                });
                selected.0 = subtab.subtab;
                subtab_event_writer.send(SelectSubTabEvent {
                    entity,
                    tab: subtab.tab,
                    subtab: subtab.subtab,
                    tab_state: InTab::Entering,
                });
            }
            _ => {}
        }
    }
}
