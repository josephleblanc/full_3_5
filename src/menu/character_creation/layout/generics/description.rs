use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            layout::{generics::list_traits, resource::*},
        },
        styles::*,
    },
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;

use bevy::reflect::TypeUuid;
pub fn build_description_list<T, U, V>(
    mut commands: Commands,
    query_parent: Query<Entity, (With<ListParent>, With<U>)>,
    race_asset: Res<Assets<T>>,
    asset_server: Res<AssetServer>,
    list_resource: Res<CentralListBundles>,
) where
    // This is the CustomAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasDescr + list_traits::HasKey<V>,
    // This is the list Label
    U: Component + Default,
    // This is the identifying enum
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display,
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
    let shared_font = asset_server.load(PATH_SIMPLE_FONT);
    let parent_entity = query_parent.get_single().unwrap();

    for key in V::vec().iter() {
        if let Some(descr_text) = race_asset
            .iter()
            .filter(|(_handle_id, asset)| asset.key() == *key)
            .map(|(_handle, asset)| asset.description())
            .next()
        {
            commands
                .spawn((
                    // Each of these nodes is one row.
                    Name::from("Race Trait description"),
                    ListNode,
                    list_resource.list_node.clone(),
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                    // Label
                    U::default(),
                ))
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
                            style: Style {
                                max_size: Size::width(Val::Px(DESCRIPTION_MAX_WIDTH)),
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
                                    style: Style {
                                        max_size: Size::width(Val::Px(DESCRIPTION_MAX_WIDTH)),
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
        }
    }
    // Description Text
}
