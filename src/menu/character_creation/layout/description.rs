use crate::constants::*;
use crate::menu::character_creation::components::*;
use crate::menu::character_creation::constants::*;
use crate::menu::styles::*;
use crate::technical::race_load::RaceAsset;
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CentralListBundles {
    list_node: NodeBundle,
    list_item_title: TextBundle,
    list_row_node: NodeBundle,
    list_col_node: NodeBundle,
    list_button: ButtonBundle,
    list_button_text: TextBundle,
    skill_replaces_text: TextBundle,
    skill_replacement_item_text: TextBundle,
    list_description_text: TextBundle,
}

impl CentralListBundles {
    pub fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let list_node = NodeBundle {
            style: LIST_PARENT_NODE_STYLE,
            background_color: Color::BLACK.into(), // RACIAL_CHOICES_BUTTON_COLOR,
            ..default()
        };

        let list_item_title = TextBundle {
            text: Text::from_section(
                "Select Me!",
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: LIST_ITEM_TITLE_STYLE,
            ..default()
        };
        let list_row_node = NodeBundle {
            style: Style {
                // padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(10.)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::GRAY.into(), // RACIAL_CHOICES_BUTTON_COLOR,
            ..default()
        };
        let list_col_node = NodeBundle {
            style: Style {
                // padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(10.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        };
        let list_button = ButtonBundle {
            style: Style {
                size: Size::width(Val::Percent(100.)),
                // padding: UiRect::left(Val::Percent(7.)),
                ..default()
            },
            background_color: Color::DARK_GREEN.into(),
            ..default()
        };
        let list_button_text = TextBundle::from_section(
            "List Button Text".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: LIST_BUTTON_TEXT_SIZE,
                color: TEXT_COLOR,
            },
        );
        let skill_replaces_text = TextBundle::from_section(
            "Replaces".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: 30.,
                color: TEXT_COLOR,
            },
        );
        let skill_replacement_item_text = TextBundle {
            text: Text::from_section(
                "Alt Race Replaces:".to_string(),
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: Style {
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        };
        let list_description_text = TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: Style {
                max_size: Size::width(Val::Px(900.)),
                margin: UiRect::left(Val::Px(20.)),
                ..default()
            },
            ..default()
        };
        let central_list_bundles = CentralListBundles {
            list_node,
            list_item_title,
            list_row_node,
            list_col_node,
            list_button,
            list_button_text,
            skill_replaces_text,
            skill_replacement_item_text,
            list_description_text,
        };
        commands.insert_resource(central_list_bundles);
    }
}

pub trait AsVec
where
    Self: Sized,
{
    fn vec() -> Vec<Self>;
}

use crate::systems::game::character::PlayableRace;

impl AsVec for PlayableRace {
    fn vec() -> Vec<Self> {
        Vec::from(PlayableRace::array())
    }
}

use bevy::asset::HandleId;
use std::iter::Filter;

pub trait HasDescr {
    fn description(&self) -> &String;
}

impl HasDescr for RaceAsset {
    fn description(&self) -> &String {
        &self.text
    }
}

pub trait HasKey<T> {
    fn key(&self) -> T;
}

impl HasKey<PlayableRace> for RaceAsset {
    fn key(&self) -> PlayableRace {
        self.race
    }
}

use bevy::reflect::TypeUuid;
pub fn build_description_list<T, U, V>(
    mut commands: Commands,
    query_parent: Query<Entity, (With<ListParent>, With<U>)>,
    race_asset: Res<Assets<T>>,
    asset_server: Res<AssetServer>,
    list_resource: Res<CentralListBundles>,
) where
    // This is the CustomAsset
    T: TypeUuid + Send + Sync + 'static + HasDescr + HasKey<V>,
    // This is the list Label
    U: Component + Default,
    // This is the identifying enum
    V: Component + AsVec + Eq + PartialEq + std::fmt::Display,
{
    let shared_font = asset_server.load(PATH_SIMPLE_FONT);
    let parent_entity = query_parent.get_single().unwrap();

    for race in V::vec().iter() {
        if let Some(descr_text) = race_asset
            .iter()
            .filter(|(_handle_id, asset)| asset.key() == *race)
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
                                race.to_string(),
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
