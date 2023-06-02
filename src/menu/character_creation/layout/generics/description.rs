use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            constants::{LIST_DESCRIPTION_TEXT_STYLE, LIST_ITEM_TITLE_STYLE},
            generics::{SubTab, SubTabListParent, Tab},
            layout::{generics::list_traits, resource::*},
        },
        components::SelectedWrapper,
        styles::*,
    },
    systems::game::{character::PlayableRace, class::PlayableClass},
    technical::{class::ClassAsset, race_load::RaceAsset},
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::select_item::{BuiltLists, ListName};

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

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct RaceItemDescription;
#[derive(Component, Copy, Clone, Debug, Default)]
pub struct ClassItemDescription;

pub fn build_description_list<R, S, T, U, V>(
    tab_identifier: R,
    subtab_identifier: S,
    build_enum: ListName,
) -> impl FnMut(
    Commands,
    Query<Entity, (With<ListParent>, With<U>)>,
    Res<Assets<T>>,
    Res<AssetServer>,
    Res<CentralListBundles>,
    ResMut<BuiltLists>,
)
where
    // The tab identifier specified when the function is called,
    // e.g. CreationTab::RaceTab
    R: Tab + Component + Copy + Clone + std::fmt::Debug,
    // This is the subtab enum identifier
    S: SubTab + Component + Copy + Clone + std::fmt::Debug,
    // This is the CustomAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasDescr + list_traits::HasKey<V>,
    // This is the list Label
    U: Component + Default,
    // This is the identifying enum
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
    (bevy::prelude::NodeBundle, SubTabListParent<R, S>): bevy::prelude::Bundle,
{
    // This is a generic function to build a description list in the style of the character
    // creation menu.
    //
    // It takes three generics:
    //   T: The custom asset, which must have its CustomAssetLoadState<T> initialized, and
    //       have been loaded with a custom plugin, e.g. app.add_plugin(MyRaceAssetPlugin)
    //       The custom asset should have some kind of identifying enum, for example with
    //       race descriptions the enum is PlayableRace.
    //   U: The label added as a component to the spawned items. This should be a unit struct
    //       that is unique to that list, so the list may be displayed or hidden by assigning
    //       the display.style to Display::Flex or Display::None.
    //   V: The identifying enum noted in T above. This should be an enum without variants
    //       that is used to associate the text description in the custom asset with a quality
    //       of the object being described, e.g. PlayableRace, ClassName, etc.
    //
    // The function also takes a list_resource parameter, which is used as a template for the
    // text, node, or button bundles to allow for a more unified presentation of multiple lists
    // spawned by this function.
    //
    // Finally, the function takes a query_parent parameter, which is used to identify the
    // parent node to which the spawned list will belong as a child. This parent must share
    // the same label as the list being used. It is not necessary for the parent to only have
    // one set of children spawned by this function, as items of the list may be shown/hidden,
    // but it would be best to have a child node in the target area which is the parent of nodes
    // spawned with this function, so all nodes spawned with this function may be hidden at the
    // same time without affecting other children in the target area.
    //
    // So in order for this function to work:
    //   1. Setup the custom asset.
    //   2. Setup the list_resource.
    //   3. Setup the list parent.
    //   4. run the function.
    move |mut commands: Commands,
          query_parent: Query<Entity, (With<ListParent>, With<U>)>,
          custom_asset: Res<Assets<T>>,
          asset_server: Res<AssetServer>,
          list_resource: Res<CentralListBundles>,
          mut res_built: ResMut<BuiltLists>
          // try to remove this later
          | {
        if !res_built.inner_mut().contains(&build_enum) {
        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let parent_entity = query_parent.get_single().unwrap();
        let key_vec = V::vec();
        let key_array = key_vec.as_slice();
        println!("{} assets loaded", custom_asset.len());
            let list_id = commands
                .spawn((
                    list_resource.subtab_list_parent.clone(),
                    SubTabListParent::from(tab_identifier, subtab_identifier),
                    Name::from("description nodes list parent"),
                    RaceItemDescription,
                ))
                .set_parent(parent_entity)
                .id();
            for (asset_key, descr_text) in custom_asset.iter().map(|(_handle, asset)| {
                println!("asset found: {}", asset.key());
                (asset.key(), asset.description())
            }) {
                if key_array.contains(&asset_key) {
                    let key = asset_key;
                    println!("--> building description node for {}", key);
                    commands
                        .spawn((
                            // Each of these nodes is one row.
                            Name::from("Race description node"),
                            ListNode,
                            list_resource.list_node.clone(),
                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            // SelectionEnum
                            key,
                            // Label
                            U::default(),
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
                                ListTitle,
                                U::default(),
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
                                    U::default(),
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
                                        U::default(),
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                        })
                        .set_parent(list_id);
                }
            }
            res_built.inner_mut().push(build_enum)
        }
        }
}
