use crate::constants::*;
use crate::menu::character_creation::components::*;
use crate::menu::character_creation::constants::*;
use crate::menu::styles::*;
use bevy::a11y::accesskit::NodeBuilder;
use bevy::a11y::accesskit::Role;
use bevy::a11y::AccessibilityNode;
use bevy::prelude::*;
use std::sync::Arc;

pub fn build_description_list(
    mut commands: Commands,
    query_parent: Query<Entity, With<TabListParent>>,
    asset_server: Res<AssetServer>,
) {
    let parent_entity = query_parent.get_single().unwrap();
    let shared_font = asset_server.load(PATH_SIMPLE_FONT);

    let list_item_title = Arc::new(|| TextBundle {
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
    });
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
            width: Val::Percent(100.),
            // padding: UiRect::left(Val::Percent(7.)),
            ..default()
        },
        background_color: Color::DARK_GREEN.into(),
        ..default()
    };
    let list_button_text = Arc::new(|| {
        TextBundle::from_section(
            "List Button Text".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: LIST_BUTTON_TEXT_SIZE,
                color: TEXT_COLOR,
            },
        )
    });
    let skill_replaces_text = Arc::new(|| {
        TextBundle::from_section(
            "Replaces".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: 30.,
                color: TEXT_COLOR,
            },
        )
    });
    let skill_replacement_item_text = Arc::new(|| TextBundle {
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
    });
    let list_description_text = Arc::new(|| TextBundle {
        text: Text::from_section(
            "",
            TextStyle {
                font: shared_font.clone(),
                font_size: 30.,
                color: TEXT_COLOR,
            },
        ),
        style: Style {
            max_width: Val::Px(900.),
            margin: UiRect::left(Val::Px(20.)),
            ..default()
        },
        ..default()
    });

    commands
        .spawn((
            // Each of these nodes is one row. The AltRacialTrait Component
            // can be used to identify this node in a systems and set
            // Display::Flex to show the alt trait and all it's children.
            Name::from("Race Trait description"),
            // Label
            ListNode,
            RaceItem,
            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
        ))
        .with_children(|alt_racial_trait_container| {
            alt_racial_trait_container.spawn((
                // Alternate Racial Trait Title
                (list_item_title)(),
                ListTitle,
                RaceItem,
                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
            ));
        })
        .with_children(|row_node| {
            row_node
                .spawn((
                    list_row_node.clone(),
                    // Container node for select button and alt racial
                    // trait description
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                    RaceItem,
                ))
                // Node Containing button to select trait and list of
                // traits it replaces.
                //  Show during:
                //  - Alternate Traits
                //  - Favored Skill
                .with_children(|button_and_descr_node| {
                    button_and_descr_node
                        .spawn((
                            list_col_node.clone(),
                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ListButtonColumn,
                            RaceItem,
                        ))
                        // Selection button
                        // Show during:
                        // - Alternate Trait
                        // - Favored Skill
                        .with_children(|button_and_replace_node| {
                            button_and_replace_node
                                .spawn((
                                    list_button.clone(),
                                    ListButton,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    RaceItem,
                                ))
                                .with_children(|alt_race_select_button| {
                                    alt_race_select_button.spawn((
                                        (list_button_text)(),
                                        ButtonText,
                                        Name::new("race: moving list item"),
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                        RaceItem,
                                    ));
                                });
                            // List of the traits this trait will replace.
                            // Used to load the titles of the traits it will replace, and
                            // select them below the racial trait button.
                            button_and_replace_node.spawn((
                                (skill_replaces_text)(),
                                RaceItem,
                                ReplacesText,
                                Name::new("'replaces' text"),
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                            button_and_replace_node.spawn((
                                skill_replacement_item_text(),
                                RaceItem,
                                ReplacesContent,
                                Name::new("Text names of replaced traits"),
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                AltTraitReplaces(Vec::new()),
                            ));
                        });
                    // Text with descrition of selected content, can be
                    // - Flavor Text,
                    // - Standard trait description
                    // - Alternate Trait description
                    // - Favored Class description
                    button_and_descr_node.spawn((
                        // Trait description
                        (list_description_text)(),
                        Description,
                        RaceItem,
                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                    ));
                });
        })
        .set_parent(parent_entity);
}
