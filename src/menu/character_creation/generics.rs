use crate::menu::{
    character_creation::components::{RaceTab, SelectedRaceTab},
    styles::*,
};
use bevy::prelude::*;

pub trait SubTabWrapper<U>
where
    U: Copy + Clone + Component,
{
    fn sub_tab(&self) -> U;
}

impl SubTabWrapper<RaceTab> for SelectedRaceTab {
    fn sub_tab(&self) -> RaceTab {
        self.0
    }
}

// Holds the subtab under which a list should be displayed
#[derive(Component, Clone, Copy)]
pub struct SubTabListParent<V>
where
    V: Copy + Clone + Component,
{
    pub sub_tab: V,
}
impl<V> SubTabListParent<V>
where
    V: Copy + Clone + Component,
{
    pub fn from(other: V) -> SubTabListParent<V> {
        SubTabListParent { sub_tab: other }
    }
}

pub fn display_sub_tab<T, V>(
    subtab: Res<T>,
    mut query_sub_tab_parent: Query<(&mut Style, &SubTabListParent<V>), With<Node>>,
) where
    // The selected subtab resource wrapping the value of the selected tab
    T: SubTabWrapper<V> + Resource,
    // The parent list component containing the tab under which this node should
    // be displayed
    // U: SubTabListParent<V>,
    // The type of the subtab
    V: Component + Copy + Eq + PartialEq,
{
    for (mut node_style, list_sub_tab) in &mut query_sub_tab_parent {
        if subtab.sub_tab() == list_sub_tab.sub_tab {
            node_style.display = Display::Flex;
        } else {
            node_style.display = Display::None;
        }
    }
}

pub fn new_selected_tab<T, U>(
) -> impl FnMut(Query<(&Interaction, &U, &mut BackgroundColor), Changed<Interaction>>, ResMut<T>) -> ()
where
    T: Resource + Copy + Eq + PartialEq + std::fmt::Debug,
    U: Component + Copy + Into<T> + Eq + PartialEq,
{
    // This function takes one component and one resource, and when an item with the
    // component is clicked on, the resouce is changed to wrap a value equal to the
    // clicked component.
    //
    // This is useful for setting up buttons that need to store information. For example,
    // during character creation a player needs to choose a class. This function allows
    // you to take a Component, e.g. ClassButton, which is attached to all the buttons with
    // class selections, and a Resource, e.g. SelectedClass, and let the selected class be
    // a different color than the other entities with ClassButton.
    move |mut interaction_query: Query<
        (&Interaction, &U, &mut BackgroundColor),
        Changed<Interaction>,
    >,
          mut selected_tab: ResMut<T>| {
        let selection_copy = *selected_tab;
        for (interaction, tab, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    if selection_copy != Into::<T>::into(*tab) {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected_tab = Into::<T>::into(*tab);
                    }
                    println!("Changing T to {:?}", selected_tab);
                }
                Interaction::Hovered => {
                    if selection_copy != Into::<T>::into(*tab) {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if selection_copy != Into::<T>::into(*tab) {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        }
    }
}

pub fn cleanup_tab_button<T, U>() -> impl FnMut(
    Query<&U, Changed<Interaction>>,
    Query<(&U, &Interaction, &mut BackgroundColor)>,
    Res<T>,
) -> ()
where
    T: Resource + Copy + Eq + PartialEq + std::fmt::Debug,
    U: Component + Copy + Into<T> + Eq + PartialEq,
{
    // Cleans up the component-resource pair in selected_tab so the slected button does not
    // continue to be the changed color after another color has been selected.
    |query_change: Query<&U, Changed<Interaction>>,
     mut query_others: Query<(&U, &Interaction, &mut BackgroundColor)>,
     selected: Res<T>| {
        if !query_change.is_empty() {
            for (button, interaction, mut color) in query_others.iter_mut() {
                if *interaction == Interaction::None && Into::<T>::into(*button) != *selected {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}
