use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            constants::{LIST_DESCRIPTION_TEXT_STYLE, LIST_ITEM_TITLE_STYLE},
            layout::{generics::list_traits, resource::*},
        },
        components::SelectedWrapper,
        styles::*,
    },
    systems::game::{archetype::MyArchetypeName, character::PlayableRace, class::PlayableClass},
    technical::{archetype::ArchetypeAsset, class::ClassAsset, race_load::RaceAsset},
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::select_item::BuiltLists;

pub fn display_node<S, V, U>(
    mut query_node: Query<(&mut Style, &V), (With<U>, With<ListNode>)>,
    selected: Res<S>,
) where
    S: SelectedWrapper<V> + Resource,
    // This is the identifying enum
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
    // This is the list Label
    U: Component + Default,
{
    // Generic way to display the node containing one of the items from the description list
    // built below.
    // This should be run each time the selected resource changes.
    for (mut node_style, node_enum) in &mut query_node {
        if *node_enum == selected.selected() {
            println!("{} display set to flex", node_enum);
            node_style.display = Display::Flex;
        } else if node_style.display == Display::Flex {
            node_style.display = Display::None;
        }
    }
}
// traits needed for generics in character creation layout
impl list_traits::HasDescr for RaceAsset {
    fn description(&self) -> &String {
        &self.text
    }
}
impl list_traits::HasDescr for ClassAsset {
    fn description(&self) -> &String {
        &self.description
    }
}
impl list_traits::HasDescr for ArchetypeAsset {
    fn description(&self) -> &String {
        &self.description
    }
}

impl list_traits::HasKey<PlayableRace> for RaceAsset {
    fn key(&self) -> PlayableRace {
        self.race
    }
}
impl list_traits::HasKey<PlayableClass> for ClassAsset {
    fn key(&self) -> PlayableClass {
        self.class_name
    }
}
impl list_traits::HasKey<MyArchetypeName> for ArchetypeAsset {
    fn key(&self) -> MyArchetypeName {
        self.archetype_name
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct RaceItemDescription;
#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ClassItemDescription;

pub fn build_description_list<T, V>(
    tab: Tab,
    subtab: SubTab,
) -> impl FnMut(
    Commands,
    Query<(Entity, &TabListParent)>,
    Res<Assets<T>>,
    Res<AssetServer>,
    Res<CentralListBundles>,
    ResMut<BuiltLists>,
)
where
    // This is the CustomAsset
    // e.g. RaceAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasDescr + list_traits::HasKey<V>,
    // This is the identifying enum
    // e.g. PlayableRace, PlayableClass
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
{
    move |mut commands: Commands,
          query_parent: Query<(Entity, &TabListParent)>,
          custom_asset: Res<Assets<T>>,
          asset_server: Res<AssetServer>,
          list_resource: Res<CentralListBundles>,
          mut res_built: ResMut<BuiltLists>
          // try to remove this later
          | {
        let subtab_list_parent = SubTabListParent { tab, subtab };
        if !res_built.inner_mut().contains(&subtab_list_parent) {
        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let key_vec = V::vec();
        let key_array = key_vec.as_slice();
        if let Some((parent_entity, _list_parent)) = query_parent.iter().filter(|(_, &list_parent)| list_parent == tab.into()).next() {
            let list_id = commands
                .spawn((
                    list_resource.subtab_list_parent.clone(),
                    Name::from("description nodes list parent"),
                    subtab_list_parent,
                ))
                .set_parent(parent_entity)
                .id();
            for (asset_key, descr_text) in custom_asset.iter().map(|(_handle, asset)| {
                (asset.key(), asset.description())
            }) {
                if key_array.contains(&asset_key) {
                    let key = asset_key;
                    commands
                        .spawn((
                            // Each of these nodes is one row.SubTabListParent
                            Name::from("Race description node"),
                            ListNode,
                            list_resource.list_node.clone(),
                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            // SelectionEnum
                            key,
                        ))
                        .set_parent(list_id)
                        .with_children(|row_node| {
                            row_node.spawn((
                                // Alternate Racial Trait Title
                                TextBundle {
                                    text: Text::from_section(
                                        key.to_string(),
                                        TextStyle {
                                            font: shared_font.clone(),
                                            font_size: DESCRIPTION_FONT_SIZE,
                                            color: TEXT_COLOR,
                                        },
                                    ),
                                    style: LIST_ITEM_TITLE_STYLE,
                                        ..default()
                                    },
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                        })
                        .with_children(|row_node| {
                            row_node
                                .spawn((
                                    list_resource.list_row_node.clone(),
                                    // Container node for select button and alt racial
                                    // trait description
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ))
                                .with_children(|inner_row_node| {
                                    // Item description
                                    inner_row_node.spawn((
                                        TextBundle {
                                            text: Text::from_section(
                                                descr_text,
                                                TextStyle {
                                                    font: shared_font.clone(),
                                                    font_size: DESCRIPTION_FONT_SIZE,
                                                    color: TEXT_COLOR,
                                                },
                                            ),
                                            style: LIST_DESCRIPTION_TEXT_STYLE,
                                            ..default()
                                        },
                                        Description,
                                        // Label
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                        })
                        .set_parent(list_id);
                }
            }
            res_built.inner_mut().push(subtab_list_parent)
        }
        }
        }
}
