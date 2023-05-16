use crate::systems::{
    game::{character::PlayableRace, race::CharacterBuilder},
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

pub const COMMON_TRAIT_FONT_SIZE: f32 = 25.;

pub fn build_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shared_font = asset_server.load("fonts/simple_font.TTF");
    commands.spawn(CharacterBuilder);

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
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    gap: Size::width(Val::Px(30.)),
                    margin: UiRect {
                        top: Val::Percent(3.),
                        ..default()
                    },
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
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
                        let mut color = RACE_BUTTON_COLOR;
                        if race_enum == PlayableRace::Human {
                            color = RACE_BUTTON_COLOR_SELECTED;
                        }
                        list.spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::width(Val::Percent(100.)),
                                    padding: UiRect::left(Val::Percent(7.)),
                                    ..default()
                                },
                                background_color: color.into(),
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
    // This contains many nodes, but on startup most will be set to
    // Display::None until the correct tab is checked.
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
            Name::from("Central content area"),
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
                                display: Display::Flex,
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
                                    "Something",
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
                                display: Display::None,
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
                                    NodeBundle {
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
                                                "Select Me!",
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
            ScrollingList::default(),
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
    let trait_column = NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::width(Val::Percent(100.)),
            ..default()
        },
        background_color: Color::YELLOW_GREEN.into(),
        ..default()
    };
    let trait_row = NodeBundle {
        style: Style {
            size: Size::width(Val::Percent(100.)),
            margin: UiRect::all(Val::Px(8.)),
            flex_direction: FlexDirection::Row,
            gap: Size::width(Val::Px(20.)),
            ..default()
        },
        background_color: Color::YELLOW_GREEN.into(),
        ..default()
    };
    let common_trait_title_style = Style {
        padding: UiRect::all(Val::Px(8.)),
        align_self: AlignSelf::Start,
        ..default()
    };
    let common_trait_value_style = Style {
        align_self: AlignSelf::End,
        padding: UiRect::right(Val::Px(20.)),
        ..default()
    };
    let common_trait_text_style = TextStyle {
        font: shared_font.clone(),
        font_size: COMMON_TRAIT_FONT_SIZE,
        color: TEXT_COLOR,
    };
    let common_traits_col_1_id = commands
        .spawn((trait_column.clone(), Name::from("Column 1 - Common Traits")))
        .set_parent(common_traits_container_id)
        .id();
    let common_traits_col_2_id = commands
        .spawn((trait_column.clone(), Name::from("Column 2 - Common Traits")))
        .set_parent(common_traits_container_id)
        .id();

    let common_traits = CommonTraits::as_array();
    let col_1_traits_index = common_traits.len() / 2;
    // Col 1
    for (common_trait, _) in common_traits.iter().zip(0..col_1_traits_index) {
        commands
            // Row 1
            .spawn((trait_row.clone(), Name::from("Row - Common Traits")))
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
                        style: common_trait_title_style.clone(),
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
                        style: common_trait_value_style.clone(),
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
            .spawn((trait_row.clone(), Name::from("Column 1 - Common Traits")))
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
                        style: common_trait_title_style.clone(),
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
                                font_size: 20.,
                                color: TEXT_COLOR,
                            },
                        ),
                        style: common_trait_value_style.clone(),
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
                background_color: Color::YELLOW_GREEN.into(),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },

                ..default()
            },
            Name::from("Container Panel - Racial Choices Made Buttons"),
        ))
        .set_parent(right_panel_id)
        .id();

    // Chosen standard traits, without description, located in the right panel.
    // Make 20 of these, then set Display::None, and set Display::Flex when
    // updating with content in a system.
    // These should:
    //  - update when selecting a race,
    //  - become grayed out when replaced by an alternate racial trait,
    //  - provide description on hover
    //
    //  Title
    let chosen_traits_title_node = NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            display: Display::Flex,
            ..default()
        },
        background_color: Color::PURPLE.into(), // RACIAL_CHOICES_BUTTON_COLOR,
        ..default()
    };
    let chosen_traits_title_style = TextStyle {
        font: shared_font.clone(),
        font_size: 25.,
        color: TEXT_COLOR,
    };
    let chosen_standard_traits_id = commands
        .spawn((
            //     chosen_traits_title_node.clone(),
            //     Name::from("Chosen Standard Trait Title - Right Panel"),
            // ))
            // .with_children(|list_button| {
            //     list_button.spawn((
            TextBundle {
                text: Text::from_section(
                    "Standard Traits".to_string(),
                    chosen_traits_title_style.clone(),
                ),
                background_color: Color::VIOLET.into(),
                style: Style {
                    display: Display::None,
                    ..default()
                },

                ..default()
            },
            Name::from("Chosen Standard Trait Title - Right Panel"),
            ChosenStandardTraitTitle,
        )) // ;
        // })
        .set_parent(chosen_traits_id)
        .id();
    let chosen_trait_node = (NodeBundle {
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            display: Display::None,
            ..default()
        },
        background_color: Color::PURPLE.into(), // RACIAL_CHOICES_BUTTON_COLOR,
        ..default()
    },);
    let chosen_trait_text = (TextBundle {
        text: Text::from_section(
            "",
            TextStyle {
                font: shared_font.clone(),
                font_size: 25.,
                color: TEXT_COLOR,
            },
        ),
        style: Style {
            display: Display::None,
            ..default()
        },
        background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
        ..default()
    },);
    //  Chosen Standard Traits - make 20 and use as needed
    for i in 0..20 {
        commands
            .spawn((
                chosen_trait_text.clone(),
                ChosenStandardTrait,
                Name::from(format!("Chosen Standard Trait Name Text {}", i)),
            )) // ;
            //     })
            .set_parent(chosen_traits_id);
    }
    //  Alternate Racial Traits
    //  This should be the list of chosen alternate traits, they should:
    //  - Say which traits they are replacing,
    //  - Have a button that removes them and replaces the default traits
    //  - Provide description when hovered over
    //
    //  Alternate Racial Traits Title
    let alternate_chosen_racial_traits_id = commands
        .spawn((
            //     chosen_traits_title_node.clone(),
            //     // Label, shared with text below
            //     ChosenAlternateTraitTitle,
            //     Name::from("Chosen Alternate Trait Title Node"),
            // ))
            // .with_children(|list_button| {
            //     list_button.spawn((
            TextBundle {
                text: Text::from_section(
                    "Alternate Traits".to_string(),
                    chosen_traits_title_style.clone(),
                ),
                ..default()
            },
            // Label, shared with node above
            ChosenAlternateTraitTitle,
            Name::from("Chosen Alternate Trait Title Text"),
        )) // ;
        // })
        .set_parent(chosen_traits_id)
        .id();
    //  Alternate Racial Traits
    for i in 0..20 {
        commands
            .spawn((
                //         chosen_trait_node.clone(),
                //         // Label, shared with text below
                //         ChosenAlternateTrait,
                //         Name::from(format!("Chosen Alternate Trait Name {}", i)),
                //     ))
                //     .with_children(|list_button| {
                //         list_button.spawn((
                chosen_trait_text.clone(),
                // Label, shared with node above
                ChosenAlternateTrait,
                Name::from(format!("Chosen Standard Trait Name Text {}", i)),
            )) // ;
            //     })
            .set_parent(chosen_traits_id);
    }
    //  Favored Class
    //  This should update when chosen, and provide a text description when hovered over.
    let favored_class_id = commands
        .spawn((
            //     chosen_traits_title_node.clone(),
            //     Name::from("Favored Class Title"),
            //     FavoredClassTitle,
            // ))
            // .with_children(|list_button| {
            //     list_button.spawn((
            TextBundle {
                text: Text::from_section(
                    "Favored Class".to_string(),
                    chosen_traits_title_style.clone(),
                ),
                background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
                ..default()
            },
            FavoredClassTitle,
            Name::new("Favored Class Title Text - Right Panel"),
        )) // ;
        // })
        .set_parent(chosen_traits_id)
        .id();

    commands
        .spawn((
            //     chosen_traits_title_node.clone(),
            //     Name::from("Favored Class Value"),
            //     FavoredClassValueText,
            // ))
            // .with_children(|list_button| {
            //     list_button.spawn((
            TextBundle {
                text: Text::from_section(
                    "Favored Class Here".to_string(),
                    chosen_traits_title_style.clone(),
                ),
                background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
                ..default()
            },
            FavoredClassValueText,
            Name::new("Favored Class Title Text - Right Panel"),
        )) // ;
        // })
        .set_parent(chosen_traits_id);

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
            Name::from("Choose Description Content Container"),
        ))
        .with_children(|button_container| {
            button_container
                .spawn((
                    NodeBundle {
                        background_color: Color::YELLOW_GREEN.into(), // RACIAL_CHOICES_PANEL_COLOR,
                        ..default()
                    },
                    Name::from("Button Container - Choose Description Content"),
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
                            Name::from("Button: Choose Description Content"),
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

    let bottom_button = ButtonBundle {
        style: Style {
            padding: UiRect::all(Val::Px(20.)),
            margin: UiRect::all(Val::Px(20.)),
            ..default()
        },
        background_color: RACE_BUTTON_COLOR.into(),
        ..default()
    };
    let bottom_button_text_style = TextStyle {
        font: shared_font.clone(),
        font_size: 25.,
        color: TEXT_COLOR,
    };
    commands
        .spawn((bottom_button.clone(), Name::from("Previous Button")))
        .with_children(|previous_button| {
            previous_button.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Previous".to_string(),
                        bottom_button_text_style.clone(),
                    ),
                    ..default()
                },
                PreviousButton,
            ));
        })
        .set_parent(bottom_container);
    commands
        .spawn((bottom_button.clone(), Name::from("Character Sheet Button")))
        .with_children(|previous_button| {
            previous_button.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Character Sheet".to_string(),
                        bottom_button_text_style.clone(),
                    ),
                    ..default()
                },
                CharacterSheetButton,
            ));
        })
        .set_parent(bottom_container);
    commands
        .spawn((bottom_button.clone(), Name::from("Next Button")))
        .with_children(|previous_button| {
            previous_button.spawn((
                TextBundle {
                    text: Text::from_section("Next".to_string(), bottom_button_text_style.clone()),
                    ..default()
                },
                NextButton,
            ));
        })
        .set_parent(bottom_container);
}
