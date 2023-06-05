use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{character_creation::components::*, styles::*},
};
use bevy::prelude::*;

use super::list_traits::AsVec;

pub trait ButtonNodeTemplate {
    fn button_bundle() -> ButtonBundle;
    fn text_bundle(button_text: String, font_handle: Handle<Font>) -> TextBundle;
}

// TODO: Look into loading these nodes with scenes in the future to allow for hot-loading and
// perhaps live editing of their parameters.
// Unit Label used to build the Tab buttons for character creation.
#[derive(Component)]
pub struct CharacterTabs;
impl ButtonNodeTemplate for CharacterTabs {
    fn button_bundle() -> ButtonBundle {
        ButtonBundle {
            style: STAGES_OF_CREATION_BUTTON,
            background_color: Color::PURPLE.into(),
            ..default()
        }
    }
    fn text_bundle(button_text: String, font_handle: Handle<Font>) -> TextBundle {
        TextBundle {
            text: Text::from_section(
                button_text,
                TextStyle {
                    font: font_handle.clone(),
                    font_size: STAGES_OF_CREATION_FONT_SIZE,
                    color: STAGES_OF_CREATION_TEXT_COLOR,
                },
            ),
            style: STAGES_OF_CREATION_TEXT_STYLE,
            ..default()
        }
    }
}

// Vec of tab buttons which have finished building.
#[derive(Resource, Clone, Default)]
pub struct BuiltTabButtons(Vec<Tab>);
impl BuiltTabButtons {
    pub fn inner_ref_mut(&mut self) -> &mut Vec<Tab> {
        &mut self.0
    }
    pub fn inner_ref(&self) -> &Vec<Tab> {
        &self.0
    }
    pub fn is_built(button: Tab) -> impl Fn(Res<Self>) -> bool {
        move |built_buttons: Res<Self>| built_buttons.inner_ref().contains(&button)
    }
}

pub fn build_tab_buttons<T, V>(
) -> impl FnMut(Commands, Query<Entity, With<T>>, Res<AssetServer>, ResMut<BuiltTabButtons>)
where
    // TODO: Revisit this function and possibly improve it later.
    // 1. It may or may not be a good design to store the methods for creating the bundles in the same
    //      struct used to identify the parent node.
    // 2. Another choice to revisit is building in `BuiltTabButtons` struct as opposed to making it
    //      another generic. Whether or not this is a good choice depends on whether I use this function
    //      in methods of other structs later on.
    // 3. There is no resource tracking whether these are built or not, because they do not depend
    //      on loading a CustomAsset like the list builder functions. This should work as long as
    //      this function is run after the parent node has been built and there is a commands
    //      flush, however it may be a source of errors.
    //
    // Used for two things:
    // 1. Identify the parent container of the buttons.
    // 2. Stores the methods which build template buttons.
    // e.g. CharacterTabs,
    //
    T: Component + ButtonNodeTemplate,
    // The Tab struct used as the enum identifier. This does not have to actually be Tab,
    // but it should be an enum used to differentiate the item buttons, and should be
    // used to send events when the button is selected.
    // e.g. Tab
    V: Component + AsVec + std::fmt::Display + Copy + Into<Tab>,
    // This could be used to wrap the identifying enum, e.g. TabButton(Tab), but for now we'll see
    // if just using Tab is satisfactory
    // A: Component + Copy,
{
    move |mut commands: Commands,
          query_parent: Query<Entity, With<T>>,
          asset_server: Res<AssetServer>,
          mut built_lists: ResMut<BuiltTabButtons>| {
        let font = asset_server.load(PATH_SIMPLE_FONT);
        let parent_entity = query_parent.get_single().unwrap();
        for tab in V::vec() {
            if !built_lists
                .inner_ref()
                .iter()
                .any(|&built_tab| built_tab == tab.into())
            {
                commands
                    // The important bit here is the `tab`, which is used to identify the selected tab
                    // and send events when the tab is clicked to make changes elsewhere in the menu
                    .spawn((
                        T::button_bundle(),
                        tab,
                        Name::from(format!("{tab} button bundle")),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            T::text_bundle(tab.to_string(), font.clone()),
                            Name::from(format!("{tab} text bundle in button")),
                        ));
                    })
                    .set_parent(parent_entity);
                built_lists.inner_ref_mut().push(tab.into())
            }
        }
    }
}
