use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::*,
            generics::{SubTab, SubTabListParent, Tab},
            layout::{generics::list_traits, resource::*},
        },
        components::SelectedWrapper,
        styles::*,
    },
    systems::game::character,
    technical::default_race_traits::DefaultTraitAsset,
};
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub fn display_list<A, B, R, P, U>(
    mut query_list_parent: Query<(&mut Style, &SubTabListParent<A, B>), With<Node>>,
    selected_tab: Res<R>,
    selected_sub_tab: Res<P>,
) where
    // The type of the tab this node should be displayed in
    A: Tab + Component + Copy + Clone + Eq,
    // The type of the subtab this node should be displayed in
    B: SubTab + Component + Copy + Clone + Eq,
    // This is the wrapper for the selected tab,
    // e.g. SelectedCreationTab
    R: SelectedWrapper<A> + Resource,
    // This is the wrapper for the selected subtab,
    // e.g. SelectedClassTab
    P: SelectedWrapper<B> + Resource,
    // This is the list Label
    // e.g. ClassItem, RaceItem
    U: Component + Default,
{
    // This is the generic way to show or hide a list of items with a selected
    // tab and subtab.
    for (mut style, list_parent) in &mut query_list_parent {
        if list_parent.tab == selected_tab.selected()
            && list_parent.sub_tab == selected_sub_tab.selected()
        {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}

use crate::systems::layout::plugin::BuiltRaceSelectItems;
pub const BUILT_LEN: usize = 3;
pub fn build_button_desc_list<R, S, T, U, V, Q>(
    tab_identifier: R,
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
    // The tab identifier specified when the function is called,
    // e.g. CreationTab::RaceTab
    R: Tab + Component + Copy + Clone,
    // This is the subtab identifier specified when the function is called,
    // e.g. RaceTab::AlternateTraits, ClassTab::ClassFeatures
    S: Component + Clone + Copy + SubTab,
    // This is the CustomAsset
    // e.g. RaceAsset, ClassAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasKey<V> + list_traits::HasItemVec<Q>,
    Q: Component + Copy + Clone,
    // This is the list Label
    // e.g. RaceItem, ClassItem
    U: Component + Default,
    // This is the identifying enum
    // e.g. PlayableRace, PlayableClass
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
        if has_run.0 == false {
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
                    list_resource.list_node.clone(),
                    SubTabListParent::from(tab_identifier, subtab_identifier),
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
                            "--> building node for {} with title: {}",
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
                                                    title.to_string(),
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
                                                    AccessibilityNode(NodeBuilder::new(
                                                        Role::ListItem,
                                                    )),
                                                ));
                                            });
                                    });
                            })
                            .set_parent(list_id);
                    }
                }
            }
            has_run.0 = true;
        }
    }
}

impl list_traits::HasKey<character::PlayableRace> for DefaultTraitAsset {
    fn key(&self) -> character::PlayableRace {
        self.race
    }
}
