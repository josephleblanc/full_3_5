use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            constants::{SUBTAB_BUTTON_BUNDLE_COLOR, SUBTAB_BUTTON_TEXT_COLOR},
        },
        styles::*,
    },
};
use bevy::prelude::*;

use super::{build_tab_buttons::ButtonNodeTemplate, list_traits::AsVec};

// Unit Label used to build the Tab buttons for character creation.
#[derive(Component, Default)]
pub struct CharacterCreationSubTabs;
impl ButtonNodeTemplate for CharacterCreationSubTabs {
    fn button_bundle() -> ButtonBundle {
        ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(5.)),
                ..default()
            },
            background_color: SUBTAB_BUTTON_BUNDLE_COLOR.into(),
            ..default()
        }
    }
    fn text_bundle(button_text: String, font_handle: Handle<Font>) -> TextBundle {
        TextBundle {
            text: Text::from_section(
                button_text.to_string(),
                TextStyle {
                    font: font_handle.clone(),
                    font_size: SUBTAB_BUTTON_FONT,
                    color: TEXT_COLOR,
                },
            ),
            background_color: SUBTAB_BUTTON_TEXT_COLOR.into(),
            ..default()
        }
    }
}

// Vec of tab buttons which have finished building.
#[derive(Resource, Clone, Default)]
pub struct BuiltSubTabButtons(Vec<SubTab>);
impl BuiltSubTabButtons {
    pub fn inner_ref_mut(&mut self) -> &mut Vec<SubTab> {
        &mut self.0
    }
    pub fn inner_ref(&self) -> &Vec<SubTab> {
        &self.0
    }
    pub fn is_built(button: SubTab) -> impl Fn(Res<Self>) -> bool {
        move |built_buttons: Res<Self>| built_buttons.inner_ref().contains(&button)
    }
}

pub fn build_subtab_buttons<T, V>(
) -> impl FnMut(Commands, Query<Entity, With<T>>, Res<AssetServer>, ResMut<BuiltSubTabButtons>)
where
    // TODO: Revisit this function and possibly improve it later,
    // More details in build_tab_buttons.rs
    //
    // Used for two things:
    // 1. Identify the parent container of the buttons.
    // 2. Stores the methods which build template buttons.
    T: Component + ButtonNodeTemplate,
    // The Tab struct used as the enum identifier. This does not have to actually be Tab,
    // but it should be an enum used to differentiate the item buttons, and should be
    // used to send events when the button is selected.
    // e.g. Tab
    V: Component + AsVec + std::fmt::Display + Copy + Into<SubTab>,
    // This could be used to wrap the identifying enum, e.g. TabButton(Tab), but for now we'll see
    // if just using Tab is satisfactory
    // A: Component + Copy,
{
    move |mut commands: Commands,
          query_parent: Query<Entity, With<T>>,
          asset_server: Res<AssetServer>,
          mut built_buttons: ResMut<BuiltSubTabButtons>| {
        let font = asset_server.load(PATH_SIMPLE_FONT);
        let parent_entity = query_parent.get_single().unwrap();
        for sub_tab in V::vec() {
            if !built_buttons
                .inner_ref()
                .iter()
                .any(|&built_tab| built_tab == sub_tab.into())
            {
                commands
                    // The important bit here is the `tab`, which is used to identify the selected tab
                    // and send events when the tab is clicked to make changes elsewhere in the menu
                    .spawn((
                        T::button_bundle(),
                        sub_tab,
                        Name::from("{sub_tab} button bundle"),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            T::text_bundle(sub_tab.to_string(), font.clone()),
                            Name::from("{tab} text bundle in button"),
                        ));
                    })
                    .set_parent(parent_entity);
                built_buttons.inner_ref_mut().push(sub_tab.into())
            }
        }
    }
}
