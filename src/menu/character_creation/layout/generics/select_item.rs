use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            generics::SubTabListParent,
            layout::{generics::list_traits, resource::*},
        },
        components::SelectedWrapper,
        styles::*,
    },
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub fn display_node<S, V, U>(
    mut query_node: Query<(&mut Style, &V), (With<U>, With<Node>)>,
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

use crate::systems::layout::plugin::BuiltRaceSelectItems;
pub const BUILT_LEN: usize = 3;
pub fn build_button_desc_list<S, T, U, V>(
    subtab_identifier: S,
) -> impl FnMut(
    Commands,
    Query<Entity, (With<ListParent>, With<U>)>,
    Res<Assets<T>>,
    Res<AssetServer>,
    Res<CentralListBundles>,
    ResMut<BuiltRaceSelectItems>,
)
where
    // This is the subtab identifier specified when the function is called,
    // e.g. RaceTab::AlternateTraits, ClassTab::ClassFeatures
    S: Component + Clone + Copy,
    // This is the CustomAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasDescr + list_traits::HasKey<V>,
    // This is the list Label
    U: Component + Default,
    // This is the identifying enum
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
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
          // try to remove this later
          mut has_run: ResMut<BuiltRaceSelectItems>| {
        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let parent_entity = query_parent.get_single().unwrap();
        let key_vec = V::vec();
        let key_array = key_vec.as_slice();
        println!("{} assets loaded", custom_asset.len());

        if custom_asset.len() == 2 && has_run.0 == false {
            let list_id = commands
                .spawn((
                    list_resource.list_node.clone(),
                    SubTabListParent::from(subtab_identifier),
                ))
                .id();
            for (asset_key, descr_text) in custom_asset.iter().map(|(_handle, asset)| {
                println!("asset found: {}", asset.key());
                (asset.key(), asset.description())
            }) {
                if key_array.contains(&asset_key) {
                    let key = asset_key;
                    println!("--> building node for {}", key);
                    commands
                        .spawn((
                            Name::from("select_item node"),
                            list_resource.list_node.clone(),
                            key,
                            U::default(),
                        ))
                        .set_parent(list_id)
                        .with_children(|list_node| {
                            list_node
                                .spawn((
                                    Name::from("button column"),
                                    list_resource.list_col_node.clone(),
                                    U::default(),
                                ))
                                .with_children(|button_col| {
                                    button_col
                                        // button to choose item
                                        .spawn((
                                            Name::from("button to choose item"),
                                            list_resource.list_button.clone(),
                                            U::default(),
                                        ))
                                        .with_children(|button| {
                                            // button text
                                            button.spawn((
                                                Name::from("button text"),
                                                list_resource.list_button_text.clone(),
                                                U::default(),
                                            ));
                                        });
                                    button_col.spawn((
                                        Name::from("text that reads 'replace'"),
                                        list_resource.skill_replaces_text.clone(),
                                        U::default(),
                                    ));
                                    button_col.spawn((
                                        Name::from("items that will be replaced"),
                                        list_resource.skill_replacement_item_text.clone(),
                                        U::default(),
                                    ));
                                });
                            list_node
                                .spawn((
                                    // Each of these nodes is one row,
                                    // they are shown alongside the button column above
                                    Name::from("Node text description container"),
                                    ListNode,
                                    list_resource.list_node.clone(),
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    // Label
                                    U::default(),
                                ))
                                .with_children(|row_node| {
                                    row_node.spawn((
                                        Name::from("Node description title"),
                                        TextBundle {
                                            text: Text::from_section(
                                                key.to_string(),
                                                TextStyle {
                                                    font: shared_font.clone(),
                                                    font_size: DESCRIPTION_FONT_SIZE,
                                                    color: TEXT_COLOR,
                                                },
                                            ),
                                            style: Style {
                                                max_size: Size::width(Val::Px(
                                                    DESCRIPTION_MAX_WIDTH,
                                                )),
                                                margin: UiRect::left(Val::Px(20.)),
                                                ..default()
                                            },
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
                                            Name::from("text container"),
                                            list_resource.list_row_node.clone(),
                                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            U::default(),
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
                                                            font_size: DESCRIPTION_FONT_SIZE,
                                                            color: TEXT_COLOR,
                                                        },
                                                    ),
                                                    style: Style {
                                                        max_size: Size::width(Val::Px(
                                                            DESCRIPTION_MAX_WIDTH,
                                                        )),
                                                        margin: UiRect::left(Val::Px(20.)),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                Description,
                                                // Label
                                                U::default(),
                                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                            ));
                                        });
                                })
                                .set_parent(parent_entity);
                        });
                }
            }
            has_run.0 = true;
        }
    }
}
