use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            constants::{LIST_DESCRIPTION_TEXT_STYLE, LIST_ITEM_TITLE_STYLE},
            layout::{generics::list_traits, resource::*},
        },
        styles::*,
    },
    systems::game::{character, race::RacialTraitName},
    technical::{alternate_traits::AltTraitAsset, default_race_traits::DefaultTraitAsset},
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;

impl list_traits::HasKey<character::PlayableRace> for DefaultTraitAsset {
    fn key(&self) -> character::PlayableRace {
        self.race
    }
}
impl HasItemVec<RacialTraitName> for DefaultTraitAsset {
    fn vec(&self) -> Vec<(&RacialTraitName, &String, &String)> {
        self.default_traits
            .iter()
            .map(|default_traits| {
                (
                    &default_traits.my_trait_name,
                    &default_traits.title,
                    &default_traits.description,
                )
            })
            .collect()
    }
}

impl list_traits::HasKey<character::PlayableRace> for AltTraitAsset {
    fn key(&self) -> character::PlayableRace {
        self.race
    }
}
impl HasItemVec<RacialTraitName> for AltTraitAsset {
    fn vec(&self) -> Vec<(&RacialTraitName, &String, &String)> {
        self.alternate_traits
            .iter()
            .map(|alternate_traits| {
                (
                    &alternate_traits.my_trait_name,
                    &alternate_traits.title,
                    &alternate_traits.description,
                )
            })
            .collect()
    }
}

use bevy::reflect::TypeUuid;

// TODO: adjust this function for the new button select
pub fn display_list() {}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct RaceItemDefaultTrait;
#[derive(Component, Copy, Clone, Debug, Default)]
pub struct RaceItemAltTrait;

#[derive(Resource, Clone, Debug, Default)]
pub struct BuiltLists(pub Vec<SubTabListParent>);

// TODO: Change this to receive events from building the lists
impl BuiltLists {
    pub fn inner_mut(&mut self) -> &mut Vec<SubTabListParent> {
        &mut self.0
    }
    pub fn inner_ref(&self) -> &Vec<SubTabListParent> {
        &self.0
    }
    pub fn is_built(list: SubTabListParent) -> impl Fn(Res<BuiltLists>) -> bool {
        move |built_lists: Res<BuiltLists>| built_lists.inner_ref().contains(&list)
    }
}

// TODO: adjust this function for the new button select
use super::list_traits::HasItemVec;
pub fn build_button_desc_list<T, V, Q>(
    tab: Tab,
    subtab: SubTab,
    with_replaces: bool,
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
    // e.g. RaceAsset, ClassAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasKey<V> + list_traits::HasItemVec<Q>,
    // This is the identifying enum
    // e.g. PlayableRace, PlayableClass
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
    // The defining enum of the vector of items inside the asset, which shall be listed with this
    // function.
    Q: Component + Copy + Clone,
{
    //
    // So in order for this function to work:
    //   1. Setup the custom asset.
    //   2. Setup the list_resource.
    //   3. Setup the list parent.
    //   4. add a system with the function, using the subtab_identifier parameter.
    move |mut commands: Commands,
          query_parent: Query<(Entity, &TabListParent)>,
          custom_asset: Res<Assets<T>>,
          asset_server: Res<AssetServer>,
          list_resource: Res<CentralListBundles>,
          mut res_built: ResMut<BuiltLists>,
          // try to remove this later
          | {
        let subtab_list_parent = SubTabListParent {
            tab,
            subtab,
        };
        if !res_built.inner_mut().contains(&subtab_list_parent) {
            println!(
                "custom_asset len when running build_button desc_list: {}",
                custom_asset.len()
            );
            if let Some((parent_entity, _list_parent)) = query_parent.iter().filter(|(_, &list_parent)| list_parent == tab.into()).next() {
            let shared_font = asset_server.load(PATH_SIMPLE_FONT);
            let key_vec = V::vec();
            let key_array = key_vec.as_slice();
                let list_id = commands
                    .spawn((
                        list_resource.list_node.clone(),
                        // SubTabListParent::from(tab_identifier, subtab_identifier),
                        Name::from("select description node parent"),
                        subtab_list_parent,
                    ))
                    .set_parent(parent_entity)
                    .id();
                for (asset_key, asset_items_vec) in custom_asset.iter().map(|(_handle, asset)| {
                    println!("asset found: {}", asset.key());
                    (asset.key(), asset.vec())
                }) {
                    for (enum_name, title, descr_text) in asset_items_vec {
                        if key_array.contains(&asset_key) {
                            let key = asset_key;
                            println!(
                                "--> building select descr node for {} with title: {}",
                                key,
                                title.to_string()
                            );
                            commands
                                .spawn((
                                    Name::from("select_item node"),
                                    list_resource.list_node.clone(),
                                    ListNode,
                                    key,
                                    *enum_name,
                                ))
                                .set_parent(list_id)
                                .with_children(|list_node| {
                                            list_node.spawn((
                                                Name::from("Node description title"),
                                                TextBundle {
                                                    text: Text::from_section(
                                                        title.to_string(),
                                                        TextStyle {
                                                            font: shared_font.clone(),
                                                            font_size: DESCRIPTION_FONT_SIZE,
                                                            color: TEXT_COLOR,
                                                        },
                                                    ),
                                                    style: LIST_ITEM_TITLE_STYLE,
                                                    ..default()
                                                },
                                                ListTitle,
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                    list_node
                                        .spawn((
                                            // Each of these nodes is one row,
                                            // they are shown alongside the button column above
                                            Name::from("Node text description container"),
                                            list_resource.list_row_node.clone(),
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            // Label
                                        ))
                                        .with_children(|row_node| {
                                    if with_replaces {
                                    row_node
                                        .spawn((
                                            Name::from("button column"),
                                            list_resource.list_col_node.clone(),
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        ))
                                        .with_children(|button_col| {
                                            button_col
                                                // button to choose item
                                                .spawn((
                                                    Name::from("button to choose item"),
                                                    list_resource.list_button.clone(),
                                                    AccessibilityNode(NodeBuilder::new(
                                                        Role::Column,
                                                    )),
                                                ))
                                                .with_children(|button| {
                                                    // button text
                                                    button.spawn((
                                                        Name::from("button text"),
                                                        list_resource.list_button_text.clone(),
                                                        AccessibilityNode(NodeBuilder::new(
                                                            Role::Button,
                                                        )),
                                                    ));
                                                });
                                            if with_replaces {
                                            button_col.spawn((
                                                Name::from("text that reads 'replace'"),
                                                list_resource.skill_replaces_text.clone(),
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                            button_col.spawn((
                                                Name::from("items that will be replaced"),
                                                list_resource.skill_replacement_item_text.clone(),
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                            }
                                        });
                                            }
                                    row_node
                                        .spawn((
                                            // Each of these nodes is one row,
                                            // they are shown alongside the button column above
                                            Name::from("Node text description container"),
                                            list_resource.list_node.clone(),
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            // Label
                                        ))
                                        .with_children(|row_node| {
                                            row_node
                                                .spawn((
                                                    Name::from("text container"),
                                                    list_resource.list_row_node.clone(),
                                                    AccessibilityNode(NodeBuilder::new(
                                                        Role::ListItem,
                                                    )),
                                                ))
                                                .with_children(|inner_row_node| {
                                                    // Item description
                                                    inner_row_node.spawn((
                                                        Name::from("item description text"),
                                                        TextBundle {
                                                            text: Text::from_section(
                                                                descr_text,
                                                                TextStyle {
                                                                    font: shared_font.clone(),
                                                                    font_size:
                                                                        DESCRIPTION_FONT_SIZE,
                                                                    color: TEXT_COLOR,
                                                                },
                                                            ),
                                                            style: LIST_DESCRIPTION_TEXT_STYLE,
                                                            ..default()
                                                        },
                                                        // Description,
                                                        // Label
                                                        AccessibilityNode(NodeBuilder::new(
                                                            Role::ListItem,
                                                        )),
                                                    ));
                                                });
                                        });
                                        });
                                });
                        }
                    }
                    }
                }
            res_built.inner_mut().push(subtab_list_parent)
        }
            }
}
