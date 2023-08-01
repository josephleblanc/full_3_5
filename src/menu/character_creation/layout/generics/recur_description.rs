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
};
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::{a11y::accesskit::NodeBuilder, reflect::TypePath};

use super::select_item::BuiltLists;

pub fn build_item_desc_list<T, V, Q>(
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
    // e.g. RaceAsset, ClassAsset
    T: TypeUuid
        + Send
        + Sync
        + 'static
        + list_traits::HasKey<V>
        + list_traits::HasItemVec<Q>
        + TypePath,
    // This is the identifying enum
    // e.g. PlayableRace, PlayableClass
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
    // The defining enum of the vector of items inside the asset, which shall be listed with this
    // function.
    Q: Component + Copy + Clone + std::fmt::Debug,
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
          mut res_built: ResMut<BuiltLists>
          // try to remove this later
          | {
        let subtab_list_parent = SubTabListParent {
            tab,
            subtab,
        };
        if !res_built.inner_ref().contains(&subtab_list_parent) {
            let shared_font = asset_server.load(PATH_SIMPLE_FONT);
            let key_vec = V::vec();
            let key_array = key_vec.as_slice();
            if let Some((parent_entity, _list_parent)) = query_parent.iter().filter(|(_, &list_parent)| list_parent == tab.into()).next() {
                let list_id = commands
                    .spawn((
                        list_resource.subtab_list_parent.clone(),
                        Name::from(format!("{tab} {subtab} select description node parent")),
                        subtab_list_parent,
                    ))
                    .set_parent(parent_entity)
                    .id();
                for (asset_key, asset_items_vec) in custom_asset.iter().map(|(_handle, asset)| {
                    (asset.key(), asset.vec())
                }) {
                    for (enum_name, title, descr_text) in asset_items_vec {
                        if key_array.contains(&asset_key) {
                            let key = asset_key;
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

                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                    list_node
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
                                }
                        }
                    }
                }
            }
            res_built.inner_mut().push(subtab_list_parent)
        }
}
