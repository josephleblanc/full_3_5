use crate::menu::{
    character_creation::components::{RaceTab, SelectedRaceTab},
    styles::*,
};
use bevy::prelude::*;

use super::components::{ClassTab, CreationTab, SelectedClassTab, SelectedCreationTab};

pub trait TabWrapper<U>
where
    U: Copy + Clone + Component + Tab,
{
    fn tab(&self) -> U;
}

impl TabWrapper<CreationTab> for SelectedCreationTab {
    fn tab(&self) -> CreationTab {
        self.0
    }
}

pub trait SubTabWrapper<U>
where
    U: Copy + Clone + Component + SubTab,
{
    fn sub_tab(&self) -> U;
}

impl SubTabWrapper<RaceTab> for SelectedRaceTab {
    fn sub_tab(&self) -> RaceTab {
        self.0
    }
}

impl SubTabWrapper<ClassTab> for SelectedClassTab {
    fn sub_tab(&self) -> ClassTab {
        self.0
    }
}

// Holds the subtab under which a list should be displayed
#[derive(Component, Clone, Copy, Debug)]
pub struct SubTabListParent<R, V>
where
    R: Copy + Clone + Component + Tab + std::fmt::Debug,
    V: Copy + Clone + Component + SubTab + std::fmt::Debug,
{
    pub tab: R,
    pub sub_tab: V,
}
impl<R, V> SubTabListParent<R, V>
where
    R: Copy + Clone + Component + Tab + std::fmt::Debug,
    V: Copy + Clone + Component + SubTab + std::fmt::Debug,
{
    pub fn from(tab: R, sub_tab: V) -> SubTabListParent<R, V> {
        println!("creating SubTabListParent from {tab:#?} and {sub_tab:#?}");
        SubTabListParent { tab, sub_tab }
    }
}

impl Tab for CreationTab {}
impl SubTab for RaceTab {}
impl SubTab for ClassTab {}

pub trait Tab {}
pub trait SubTab {}

pub fn display_sub_tab<R, U, V, T>(
    subtab: Res<T>,
    mut query_sub_tab_parent: Query<(&mut Style, &SubTabListParent<R, V>), With<Node>>,
) where
    R: Tab + Component + Copy + Clone + std::fmt::Debug,
    // The selected tab resource wrapping the value of the selected tab
    U: TabWrapper<R> + Resource,
    // The type of the subtab
    V: SubTab + Component + Copy + Eq + PartialEq + std::fmt::Debug,
    // The selected subtab resource wrapping the value of the selected tab
    T: SubTabWrapper<V> + Resource,
{
    for (mut node_style, list_sub_tab) in &mut query_sub_tab_parent {
        if subtab.sub_tab() == list_sub_tab.sub_tab {
            node_style.display = Display::Flex;
        } else if node_style.display == Display::Flex {
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
