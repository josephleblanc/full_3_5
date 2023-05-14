use crate::systems::interface::mouse::mouse_left_clicked;
use crate::{
    system_scheduling::states::AppState,
    systems::{
        game::character::PlayableRace,
        menu::{
            character_creation::*,
            components::{
                RaceDescriptionNode, RaceDescriptionNodeParent, RaceSelectButton, ScrollingList,
                StagesOfCreationButton,
            },
            mouse::mouse_scroll,
            styles::{
                CHARACTER_CREATION_TITLE_STYLE, RACE_BUTTON_COLOR, RACE_BUTTON_COLOR_HOVERED,
                RACE_BUTTON_COLOR_SELECTED, RACIAL_CHOICES_BUTTON_COLOR, RACIAL_CHOICES_NODE_COLOR,
                RACIAL_CHOICES_PANEL_COLOR, RACIAL_CHOICES_TEXT_BG_COLOR,
                STAGES_OF_CREATION_BUTTON, STAGES_OF_CREATION_BUTTON_COLOR,
                STAGES_OF_CREATION_FONT_SIZE, STAGES_OF_CREATION_TEXT_COLOR,
                STAGES_OF_CREATION_TEXT_STYLE, TEXT_COLOR,
            },
        },
    },
    technical::{
        default_race_traits::DefaultTraitAsset,
        is_custom_asset_loaded::{is_custom_asset_loaded, CustomAssetLoadState},
        race_load::RaceAsset,
    },
};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";
const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/races/default_traits";

pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app
            //// init resources & build layout
            .init_resource::<SelectedRaceButton>()
            .init_resource::<SelectedRacialDescriptionType>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .add_system(setup_assets.in_schedule(OnEnter(AppState::CharacterCreation)))
            .add_system(build_layout.in_schedule(OnEnter(AppState::CharacterCreation)))
            //// Configure Sets
            .configure_sets((
                ButtonSet::Super.run_if(
                    in_state(AppState::CharacterCreation).and_then(
                        is_custom_asset_loaded::<RaceAsset>()
                            .and_then(is_custom_asset_loaded::<DefaultTraitAsset>()),
                    ),
                ),
                ButtonSet::Clicked
                    .in_set(ButtonSet::Super)
                    .run_if(on_event::<MouseButtonInput>().and_then(mouse_left_clicked)),
                ButtonSet::RacialTab
                    .run_if(
                        resource_changed::<SelectedRaceButton>()
                            .or_else(resource_changed::<SelectedRacialDescriptionType>()),
                    )
                    .in_set(ButtonSet::Clicked),
            ))
            .configure_set(ButtonSet::AnyInteraction.in_set(ButtonSet::Super))
            //// add systems
            .add_systems(
                (race_select_button_system, cleanup_race_select_button)
                    .chain()
                    .in_set(ButtonSet::Clicked),
            )
            .add_systems(
                (
                    selected_race_description_type,
                    cleanup_selected_race_description_button,
                )
                    .chain()
                    .in_set(ButtonSet::Clicked),
            )
            .add_system(mouse_scroll.in_set(OnUpdate(AppState::CharacterCreation)))
            .add_systems(
                (
                    selected_default_traits_visibility,
                    selected_race_visibility,
                    display_racial_description_type,
                    hide_racial_trait_text,
                    hide_racial_trait_button,
                )
                    .in_set(ButtonSet::RacialTab),
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ButtonSet {
    Super,
    Clicked,
    AnyInteraction,
    RacialTab,
}
// Figure out if you can make sure that this only runs once when entering
// character creation, then use it to hold setup and custom asset loading
// systems.
// #[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
// enum Layout {
//     Setup,
// }

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

    //// Hides nodes that don't need to be displayed, but may be viewed again
    // commands.spawn((
    //     NodeBundle {
    //         style: Style {
    //             size: Size::all(Val::Percent(10.)),
    //             position: UiRect::bottom(Val::Percent(200.)),
    //             display: Display::None,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //      RacialChoicesNodeContainer,
    // ));

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
                    Name::from("moving panel"),
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
                            Name::new("race: text description"),
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
                        for row_number in 0..default_racial_trait_rows {
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
                                        RacialTraitListNumber(row_number),
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
                                RacialTraitDescriptionText,
                                RacialTraitListNumber(row_number),
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

    let displayed_racial_stats = ["Ability Score Modifiers"];
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(400.), Val::Percent(100.)),
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    max_size: Size::width(Val::Px(400.)),
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
                    Name::from("Container Panel - Racial Choices Made Buttons"),
                ))
                .with_children(|list| {
                    for racial_choices_made_button_type in displayed_racial_stats {
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
                        });
                    }
                });
        })
        .set_parent(mid_container);

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
