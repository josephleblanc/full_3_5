use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            constants::{LIST_DESCRIPTION_TEXT_STYLE, LIST_ITEM_TITLE_STYLE},
            generics::{SubTab, SubTabListParent, Tab},
            layout::{
                generics::{list_traits, select_item::RaceItemDefaultTrait},
                resource::*,
            },
        },
        styles::*,
    },
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use super::select_item::{BuiltLists, ListName};

pub fn build_item_desc_list<U, R, S, T, V, Q>(
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
    // This is the list Label
    // e.g. RaceItem, ClassItem
    U: Component + Default,
    // The tab identifier specified when the function is called,
    // e.g. CreationTab::RaceTab
    R: Tab + Component + Copy + Clone + std::fmt::Debug,
    // This is the subtab identifier specified when the function is called,
    // e.g. RaceTab::AlternateTraits, ClassTab::ClassFeatures
    S: Component + Clone + Copy + SubTab + std::fmt::Debug,
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
          query_parent: Query<Entity, (With<ListParent>, With<U>)>,
          custom_asset: Res<Assets<T>>,
          asset_server: Res<AssetServer>,
          list_resource: Res<CentralListBundles>,
        mut res_built: ResMut<BuiltLists>
          // try to remove this later
          | {
        if !res_built.inner_mut().contains(&build_enum) {
            println!(
                "custom_asset len when running build_button desc_list: {}",
                custom_asset.len()
            );
            let shared_font = asset_server.load(PATH_SIMPLE_FONT);
            let parent_entity = query_parent.get_single().unwrap();
            let key_vec = V::vec();
            let key_array = key_vec.as_slice();
                let list_id = commands
                    .spawn((
                        list_resource.subtab_list_parent.clone(),
                        SubTabListParent::from(tab_identifier, subtab_identifier),
                        Name::from("select description node parent"),
                        RaceItemDefaultTrait,
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
                                    U::default(),
                                ))
                                .set_parent(list_id)
                                .with_children(|list_node| {
                                    list_node
                                        .spawn((
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
                                            U::default(),
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                    list_node
                                        .spawn((
                                            Name::from("text container"),
                                            list_resource.list_row_node.clone(),
                                            AccessibilityNode(NodeBuilder::new(
                                                Role::ListItem,
                                            )),
                                            U::default(),
                                        ))
                                        .with_children(|inner_row_node| {
                                            println!(
                                            "--> building node for {} with description: {}",
                                            key,
                                            descr_text
                                        );
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
                                                U::default(),
                                                AccessibilityNode(NodeBuilder::new(
                                                    Role::ListItem,
                                                )),
                                            ));
                                        });
                                    });
                                }
                        }
                    }
                }
            res_built.inner_mut().push(build_enum)
        }
}
