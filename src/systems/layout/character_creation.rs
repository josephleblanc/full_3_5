use crate::systems::{
    game::character::PlayableRace,
    menu::{
        character_creation::*,
        components::{
            RaceDescriptionNode, RaceDescriptionNodeParent, RaceSelectButton, ScrollingList,
            StagesOfCreationButton,
        },
        styles::*,
    },
};
use bevy::prelude::*;

pub fn build_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shared_font = asset_server.load("fonts/simple_font.TTF");

    //// First level Container
    // Top-level container
    let top_level_container_id = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .id();

    //// Second level containers
    // Bar near top of screen with title and stages of character creation.
    let high_container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    margin: UiRect {
                        bottom: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
            Name::from("high container"),
        ))
        .set_parent(top_level_container_id)
        .id();
    // Middle of the screen, where the details are selected and character
    // creation choices are made.
    // Three partitions, with left and right reserved for possible future
    // menu items.
    let mid_container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::new(Val::Percent(100.), Val::Percent(75.)),
                    min_size: Size::new(Val::Auto, Val::Percent(70.)),
                    flex_grow: 1.0,
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::from("middle container"),
        ))
        .set_parent(top_level_container_id)
        .id();
    // Bottom of the screen, where some information may be stored,
    // idk right now.
    let bottom_container = commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    margin: UiRect {
                        top: Val::Percent(3.),
                        ..default()
                    },
                    ..default()
                },
                background_color: Color::GREEN.into(),
                ..default()
            },
            Name::from("low container"),
        ))
        .set_parent(top_level_container_id)
        .id();

    //////  Level Three Containers followed by their contents
    //// high_container children
    // Title container with text
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    ..default()
                },
                background_color: Color::MAROON.into(),
                ..default()
            },
            Name::from("title container"),
        ))
        .set_parent(high_container)
        .with_children(|title_container| {
            title_container.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Character Creation",
                        TextStyle {
                            font: shared_font.clone(),
                            font_size: 60.,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                Name::from("title text"),
            ));
        });
    // Container for Stages of Character Creation
    // Stages of Character Cration
    // Race -> Ability Scores -> Class -> Skills -> Feats -> Optionals
    //  Optionals are: Spells, Class Feats, Animal Companion, etc.
    let stages_of_creation_container_id = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::GOLD.into(),
                ..default()
            },
            Name::from("stages of creation container"),
        ))
        .set_parent(high_container)
        .id();
    // Race Button and text
    let button_name = Name::from("stages of creation button");
    let button_text = [
        "Race",
        "Ability Scores",
        "Class",
        "Skills",
        "Feats",
        "Bonus Feats",
        "Optional",
    ];
    let text_bundle_name = [
        "race button text",
        "ability scores button text",
        "class button text",
        "skills button text",
        "feats button text",
        "bonus feats button text",
        "optional button text",
    ];
    let button_type = [
        StagesOfCreationButton::Race,
        StagesOfCreationButton::AbilityScores,
        StagesOfCreationButton::Class,
        StagesOfCreationButton::Skills,
        StagesOfCreationButton::Feats,
        StagesOfCreationButton::BonusFeats,
        StagesOfCreationButton::Optional,
    ];
    for ((&button_text, &text_bundle_name), &button_type) in button_text
        .iter()
        .zip(text_bundle_name.iter())
        .zip(button_type.iter())
    {
        commands
            .spawn((
                ButtonBundle {
                    style: STAGES_OF_CREATION_BUTTON,
                    background_color: Color::PURPLE.into(),
                    ..default()
                },
                button_name.clone(),
                button_type,
            ))
            .with_children(|race_button| {
                race_button.spawn((
                    TextBundle {
                        text: Text::from_section(
                            button_text,
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: STAGES_OF_CREATION_FONT_SIZE,
                                color: STAGES_OF_CREATION_TEXT_COLOR,
                            },
                        ),
                        style: STAGES_OF_CREATION_TEXT_STYLE,
                        ..default()
                    },
                    Name::from(text_bundle_name),
                ));
            })
            .set_parent(stages_of_creation_container_id);
    }

    ////
    //// mid_container children
    ////
    use bevy::a11y::{
        accesskit::{NodeBuilder, Role},
        AccessibilityNode,
    };
    // Setup for race select button panel, located on the left,
    // with a scrolling selection of buttons for player races.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::height(Val::Percent(90.)),
                    overflow: (Overflow::Hidden),
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            Name::from("Scrolling List container"),
        ))
        .with_children(|scrolling_list_container| {
            scrolling_list_container
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    ScrollingList::default(),
                    AccessibilityNode(NodeBuilder::new(Role::List)),
                    Name::from("moving panel"),
                    Interaction::default(),
                ))
                .with_children(|list| {
                    for race_enum in PlayableRace::iterator() {
                        list.spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(100.)),
                                    padding: UiRect::left(Val::Percent(7.)),
                                    ..default()
                                },
                                background_color: RACE_BUTTON_COLOR.into(),
                                ..default()
                            },
                            race_enum,
                            RaceSelectButton,
                        ))
                        .with_children(|list_button| {
                            list_button.spawn((
                                TextBundle::from_section(
                                    race_enum.to_string(),
                                    TextStyle {
                                        font: shared_font.clone(),
                                        font_size: 30.,
                                        color: TEXT_COLOR,
                                    },
                                ),
                                Name::new("race: moving list item"),
                                Label,
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                        });
                    }
                });
        })
        .set_parent(mid_container);

    // Setup for Race description area, located in the middle and right,
    // with text descriptions of the selected races.
    // This contains many nodes, but on startup most will be transformed into the
    // negative Z direction to hide them until the correct tab is checked.
    let default_racial_trait_rows = 20_usize;
    let center_area = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Percent(100.), Val::Percent(90.)),
                    overflow: (Overflow::Hidden),
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            Name::from("Scrolling text container"),
        ))
        .with_children(|scrolling_list_container| {
            scrolling_list_container
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            size: Size::width(Val::Percent(100.)),
                            ..default()
                        },
                        ..default()
                    },
                    ScrollingList::default(),
                    AccessibilityNode(NodeBuilder::new(Role::List)),
                    Name::from("moving panel - flavor text"),
                    Interaction::default(),
                    // Marks parent of the RaceDescriptionNode to be used when
                    // returning the child to the parent.
                    RaceDescriptionNodeParent,
                ))
                .with_children(|list| {
                    list.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.)),
                                padding: UiRect::left(Val::Percent(7.)),
                                ..default()
                            },
                            background_color: Color::OLIVE.into(),
                            ..default()
                        },
                        // Marks the node for use by display_racial_description_type,
                        // used to switch the content depending on the racial tab
                        // button pressed, e.g. Description, Racial Traits, etc.
                        RaceDescriptionNode(RacialChoicesButtonType::RaceDescription),
                    ))
                    .with_children(|text_area| {
                        // Holds the flavor text descriptions of various races.
                        text_area.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: shared_font.clone(),
                                        font_size: 30.,
                                        color: TEXT_COLOR,
                                    },
                                ),
                                style: Style {
                                    max_size: Size::width(Val::Px(1200.)),
                                    ..default()
                                },
                                ..default()
                            },
                            DescriptionSection,
                            ActiveRaceDescription(PlayableRace::Human),
                            Name::new("race: flavor text description"),
                            AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                        ));
                    });
                })
                .with_children(|list| {
                    list.spawn((
                        NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.)),
                                padding: UiRect::left(Val::Px(10.)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::OLIVE.into(),
                            ..default()
                        },
                        // Marks the node for use by display_racial_description_type,
                        // used to switch the content depending on the racial tab
                        // button pressed, e.g. Description, Racial Traits, etc.
                        RaceDescriptionNode(RacialChoicesButtonType::StandardRacialTraitNames),
                    ))
                    .with_children(|racial_traits| {
                        // _row_number is only used to make the containers which will
                        // be filled by the systems that manage the trait description
                        // text
                        for _row_number in 0..default_racial_trait_rows {
                            racial_traits
                                .spawn((
                                    // Button to select or deselect a racial trait
                                    ButtonBundle {
                                        style: Style {
                                            padding: UiRect::all(Val::Px(5.)),
                                            margin: UiRect::all(Val::Px(10.)),
                                            ..default()
                                        },
                                        background_color: Color::PURPLE.into(), // RACIAL_CHOICES_BUTTON_COLOR,
                                        ..default()
                                    },
                                    RacialTraitButton,
                                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                ))
                                .with_children(|racial_traits| {
                                    racial_traits.spawn((
                                        // Button text
                                        TextBundle {
                                            text: Text::from_section(
                                                "Select",
                                                TextStyle {
                                                    font: shared_font.clone(),
                                                    font_size: 30.,
                                                    color: TEXT_COLOR,
                                                },
                                            ),
                                            style: Style {
                                                max_size: Size::width(Val::Px(1200.)),
                                                margin: UiRect::all(Val::Px(5.)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        RacialTraitButtonText,
                                        AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                                    ));
                                });
                            racial_traits.spawn((
                                // Trait description
                                TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: shared_font.clone(),
                                            font_size: 30.,
                                            color: TEXT_COLOR,
                                        },
                                    ),
                                    style: Style {
                                        max_size: Size::width(Val::Px(1200.)),
                                        margin: UiRect::left(Val::Px(20.)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                DefaultTraitDescriptionText,
                                AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                            ));
                        }
                    });
                });
        })
        .set_parent(mid_container)
        .id();

    // Panel with chosen racial traits and favored class.
    // Should be located on the right of the screen
    // This panel should:
    //  - update when new racial traits are chosen
    //  - indicate when it is displaying the default choices
    //  - have a reset button that restores the default choices
    //  - provide a description of the chosen option when hovered over
    //  - include boxes with the displayed changes to stats for the
    //    chosen race.
    use crate::systems::game::character::AbilityScore;
    let displayed_racial_stats = ["Ability Score Modifiers"];
    let displayed_ability_scores = AbilityScore::as_array();
    let right_panel_id = commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(400.), Val::Percent(100.)),
                    align_self: AlignSelf::Center,
                    max_size: Size::width(Val::Px(400.)),
                    padding: UiRect::all(Val::Px(15.)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(), // RACIAL_CHOICES_NODE_COLOR,
                ..default()
            },
            Name::from("Current Racial Trait Stat Effects"),
        ))
        .set_parent(mid_container)
        .id();
    // Ability Score Modifiers title in right panel area
    // Should appear at the top
    commands
        .spawn((
            TextBundle {
                text: Text::from_section(
                    "Ability Scores Modifiers".to_string(),
                    TextStyle {
                        font: shared_font.clone(),
                        font_size: 25.,
                        color: TEXT_COLOR,
                    },
                ),
                background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
                ..default()
            },
            Name::from("Ability Scores Title"),
        ))
        .set_parent(right_panel_id);
    // These should spawn the name of the ability score and an
    // empty square that can display the current modifier,
    // and they should be side-by-side.
    for ability_score in displayed_ability_scores {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        max_size: Size::width(Val::Px(400.)),
                        justify_content: JustifyContent::SpaceBetween,
                        margin: UiRect::new(Val::Px(20.), Val::Px(140.), Val::Px(8.), Val::Px(8.)),
                        ..default()
                    },
                    background_color: Color::INDIGO.into(),
                    ..default()
                },
                Name::from("Ability Scores Container"),
            ))
            .with_children(|ability_scores_container| {
                ability_scores_container.spawn((
                    TextBundle {
                        text: Text::from_section(
                            ability_score.to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    },
                    Name::from("Ability Score Name Text"),
                ));
                ability_scores_container.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "-",
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::BLACK.into(),
                        ..default()
                    },
                    Name::from("Ability Score Modifier Display"),
                ));
            })
            .set_parent(right_panel_id);
    }
    // Section with traits common to all races, e.g. Speed, Size, etc.
    // Should contain text followed by an empty square that can contain the
    // associated value, and update on choosing different races and alternate
    // traits.
    let common_traits_container_id = commands
        .spawn((
            NodeBundle {
                background_color: Color::YELLOW_GREEN.into(),
                ..default()
            },
            Name::from("Common Traits Container"),
        ))
        .set_parent(right_panel_id)
        .id();
    // First row with Size and Speed
    let common_traits_col_1_id = commands
        .spawn((
            NodeBundle {
                background_color: Color::YELLOW_GREEN.into(),
                ..default()
            },
            Name::from("Column 1 - Common Traits"),
        ))
        .set_parent(common_traits_container_id)
        .id();
    let common_traits_col_2_id = commands
        .spawn((
            NodeBundle {
                background_color: Color::YELLOW_GREEN.into(),
                ..default()
            },
            Name::from("Column 1 - Common Traits"),
        ))
        .set_parent(common_traits_container_id)
        .id();

    let common_traits = CommonTraits::as_array();
    let col_1_traits_index = common_traits.len() / 2;
    // Col 1
    for (common_trait, _) in common_traits.iter().zip(0..col_1_traits_index) {
        commands
            // Row 1
            .spawn((
                NodeBundle {
                    background_color: Color::YELLOW_GREEN.into(),
                    ..default()
                },
                Name::from("Column 1 - Common Traits"),
            ))
            .set_parent(common_traits_col_1_id)
            .with_children(|col1_row1| {
                col1_row1.spawn((
                    TextBundle {
                        text: Text::from_section(
                            common_trait.to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::VIOLET.into(),
                        ..default()
                    },
                    // Name::new("Racial Choices Made Display Text"),
                ));
            })
            .with_children(|col1_row1| {
                // Speed value
                col1_row1.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "-".to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::VIOLET.into(),
                        ..default()
                    },
                    *common_trait,
                    // Name::new("Racial Choices Made Display Text"),
                ));
            });
    }
    // Col 2
    let col_2_traits = common_traits.iter().skip(col_1_traits_index);
    for common_trait in col_2_traits {
        commands
            // Row 1
            .spawn((
                NodeBundle {
                    background_color: Color::YELLOW_GREEN.into(),
                    ..default()
                },
                Name::from("Column 1 - Common Traits"),
            ))
            .set_parent(common_traits_col_2_id)
            .with_children(|col1_row1| {
                // Size Title
                col1_row1.spawn((
                    TextBundle {
                        text: Text::from_section(
                            common_trait.to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::VIOLET.into(),
                        ..default()
                    },
                    // Name::new("Racial Choices Made Display Text"),
                ));
            })
            .with_children(|col1_row1| {
                // Size value
                col1_row1.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "-".to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::VIOLET.into(),
                        ..default()
                    },
                    *common_trait,
                    // Name::new("Racial Choices Made Display Text"),
                ));
            });
    }

    // Area displaying currently chosen traits, starting with defaults
    let chosen_traits_id = commands
        .spawn((
            NodeBundle {
                background_color: Color::YELLOW_GREEN.into(), // RACIAL_CHOICES_PANEL_COLOR,
                ..default()
            },
            Name::from("Container Panel - Racial Choices Made Buttons"),
        ))
        .set_parent(right_panel_id)
        .id();
    for racial_choices_made_button_type in displayed_racial_stats {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(5.)),
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    background_color: Color::PURPLE.into(), // RACIAL_CHOICES_BUTTON_COLOR,
                    ..default()
                },
                RacialChoiceButton,
                Name::from("Racial Choices Made Button"),
            ))
            .with_children(|list_button| {
                list_button.spawn((
                    TextBundle {
                        text: Text::from_section(
                            racial_choices_made_button_type.to_string(),
                            TextStyle {
                                font: shared_font.clone(),
                                font_size: 25.,
                                color: TEXT_COLOR,
                            },
                        ),
                        background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
                        ..default()
                    },
                    // Name::new("Racial Choices Made Display Text"),
                ));
            })
            .set_parent(chosen_traits_id);
    }

    // Button panel with selections for which details of the selected race should
    // be displayed in the central description area.
    // Includes sections:
    //  - Race Description
    //  - Standard Racial Traits
    //  - Alternate Racial Traits
    //  - Racial Subtypes (maybe?)
    //  - Favored Class Options
    //  - Racial Feats
    // let racial_choices_button_titles = RacialChoicesButtonType::array();
    let description_select_buttons = RacialChoicesButtonType::array();
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BEIGE.into(), // RACIAL_CHOICES_NODE_COLOR,
                ..default()
            },
            Name::from("Current Racial Trait Stat Effects"),
        ))
        .with_children(|button_container| {
            button_container
                .spawn((
                    NodeBundle {
                        background_color: Color::YELLOW_GREEN.into(), // RACIAL_CHOICES_PANEL_COLOR,
                        ..default()
                    },
                    Name::from("Container Panel - Choose Description Content"),
                ))
                .with_children(|list| {
                    for description_button in description_select_buttons {
                        list.spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(5.)),
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                },
                                background_color: Color::PURPLE.into(), // RACIAL_CHOICES_BUTTON_COLOR,
                                ..default()
                            },
                            description_button,
                            Name::from("Choose Description Content"),
                        ))
                        .with_children(|list_button| {
                            list_button.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        description_button.to_string(),
                                        TextStyle {
                                            font: shared_font.clone(),
                                            font_size: 25.,
                                            color: TEXT_COLOR,
                                        },
                                    ),
                                    background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
                                    ..default()
                                },
                                // Name::new("Racial Choices Made Display Text"),
                            ));
                        });
                    }
                });
        })
        .set_parent(high_container);
}
