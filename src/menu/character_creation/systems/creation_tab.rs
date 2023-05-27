use crate::menu::character_creation::components::*;
use crate::menu::styles::*;
use bevy::prelude::*;

// Makes sure other race buttons are the default color.
pub fn cleanup_tab_button(
    query_change: Query<&CreationTab, Changed<Interaction>>,
    mut query_others: Query<(&CreationTab, &Interaction, &mut BackgroundColor)>,
    selected_tab: Res<CreationTabSelected>,
) {
    if !query_change.is_empty() {
        for (tab, interaction, mut color) in query_others.iter_mut() {
            if *interaction == Interaction::None && *tab != selected_tab.inner() {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

// Changes the color of the selected race button, and sets CreationTabSelected
pub fn selected_tab(
    mut interaction_query: Query<
        (&Interaction, &CreationTab, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_tab: ResMut<CreationTabSelected>,
) {
    let selection_copy = (*selected_tab).inner();
    for (interaction, tab, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected_tab = CreationTabSelected(*tab);
                }
                println!("Changing CreationTabSelected to {:?}", selected_tab);
            }
            Interaction::Hovered => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}
