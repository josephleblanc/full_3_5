use crate::{
    systems::{
        game::character::PlayableRace,
        game::race::RacialTraitName,
        menu::{
            components::RaceSelectButton,
            styles::{RACE_BUTTON_COLOR, RACE_BUTTON_COLOR_HOVERED, RACE_BUTTON_COLOR_SELECTED},
        },
    },
    technical::default_race_traits::DefaultTraitAsset,
    technical::is_custom_asset_loaded::CustomAssetLoadState,
    technical::race_load::RaceAsset,
};
use bevy::prelude::*;
const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";
const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/race/default_traits";
// Stores the race selected by the player
// Also used to make sure only one selected button has its background changed.
#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct SelectedRaceButton(pub PlayableRace);

impl SelectedRaceButton {
    fn inner(&self) -> PlayableRace {
        self.0
    }
}

// Bottom container buttons
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct CharacterSheetButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct PreviousButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct NextButton;

// Right Panel Titles
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenStandardTraitTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenAlternateTraitTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct FavoredClassTitle;
// Right Panel Values
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenStandardTrait;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenAlternateTrait;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct FavoredClassValueText;
// Manage visibility of chosen racial traits located in the right panel.
// It should:
//  - Copy loaded racial trait title into an already constructed TextBundle's
//      Text field,
//  - Set display to Display::Flex if the text is present
//  - Otherwise Display::None,
pub fn standard_traits_visibility(
    selected_race: Res<SelectedRaceButton>,
    mut query_title: Query<
        &mut Style,
        (With<ChosenStandardTraitTitle>, Without<ChosenStandardTrait>),
    >,
    mut query_trait: Query<
        (&mut Style, &mut Text, &mut TooltipText),
        (With<ChosenStandardTrait>, Without<ChosenStandardTraitTitle>),
    >,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<DefaultTraitAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected_race.inner());
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    // let mut text = query_text.get_single_mut().unwrap();
    for (_handle, trait_asset) in assets.iter() {
        println!("--------------------trait asset loaded--------------------------");
        if trait_asset.race == selected_race.inner() {
            let selected_traits = trait_asset;
            for (i, (mut trait_style, mut trait_text, mut tooltip_text)) in
                query_trait.iter_mut().enumerate()
            {
                if i < selected_traits.default_traits.len() {
                    *trait_text = Text::from_section(
                        selected_traits.default_traits[i].title.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 25.,
                            color: Color::WHITE,
                        },
                    );
                    tooltip_text.0 = Text::from_section(
                        selected_traits.default_traits[i].description.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                    );
                    (*trait_style).display = Display::Flex;
                } else {
                    (*trait_style).display = Display::None;
                }
            }
        }
    }
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Tooltip;
#[derive(Component, Clone, Debug)]
pub struct TooltipText(pub Text);
#[derive(Resource, Clone)]
pub struct TooltipTimer(pub Timer);
impl TooltipTimer {
    pub fn inner_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}
pub fn chosen_trait_tooltip(
    query_trait: Query<(&Interaction, &TooltipText)>,
    query_change: Query<&Interaction, (Changed<Interaction>, With<TooltipText>)>,
    mut timer: ResMut<TooltipTimer>,
    time: Res<Time>,
    mut event_reader: EventReader<CursorMoved>,
    mut query_tooltip: Query<(&mut Style, &mut Text), With<Tooltip>>,
) {
    for (interaction, tooltip_text) in &query_trait {
        if *interaction == Interaction::Hovered && timer.inner_mut().tick(time.delta()).finished() {
            if let Some(cursor_event) = event_reader.iter().last() {
                let (mut tooltip_style, mut tooltip) = query_tooltip.get_single_mut().unwrap();
                tooltip_style.display = Display::Flex;
                let mut calculated_tooltip_left = Val::Px(cursor_event.position.x - 20.);
                calculated_tooltip_left
                    .try_sub_assign(tooltip_style.size.width)
                    .unwrap();
                tooltip_style.position = UiRect {
                    left: calculated_tooltip_left,
                    bottom: Val::Px(cursor_event.position.y),
                    ..default()
                };
                *tooltip = tooltip_text.0.clone();
                println!("tooltip position: {:#?}", tooltip_style.position);
                println!("tooltip position: {:#?}", cursor_event.position);
            }
        }
    }
    for changed_interaction in &query_change {
        if *changed_interaction == Interaction::None {
            let (mut tooltip_style, mut tooltip) = query_tooltip.get_single_mut().unwrap();
            tooltip_style.display = Display::None;
            *tooltip = Text::default();
            timer.inner_mut().reset();
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct ActiveRaceDescription(pub PlayableRace);

#[derive(Component, Clone, Copy, Debug)]
pub struct DescriptionSection;

#[derive(Component, Clone, Copy, Debug)]
pub struct RacialChoiceButton;

// #[derive(Component, Clone, Debug)]
// pub struct RacialChoicesButton;

#[derive(Component, Clone, Debug)]
pub struct RightPanel;

#[derive(Component, Clone, Debug)]
pub struct RacialTraitNameSelections {
    traits_chosen: Vec<RacialTraitName>,
    race: PlayableRace,
}

use crate::systems::game::character::AbilityScore;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialABSDisplay(AbilityScore);

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButtonText;

// Label for buttons that let you select a racial trait to replace a default
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButton;

// Label for default race description text bundles
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct DefaultTraitDescriptionText;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct DefaultRacialTraitRace(pub PlayableRace);

// Enum for the tabs of the race section of character creation.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum RacialChoicesButtonType {
    #[default]
    RaceDescription,
    StandardRacialTraitNames,
    AlternateRacialTraitNames,
    RacialSubtypes,
    FavoredClassOption,
}

// Common traits displayed in the right panel of race selection in
// character creation.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum CommonTraits {
    #[default]
    Size,
    Speed,
    Type,
    Subtype,
    /* more here */
}

impl CommonTraits {
    pub fn as_array() -> [CommonTraits; 4] {
        [Self::Size, Self::Speed, Self::Type, Self::Subtype]
    }
}

impl std::fmt::Display for CommonTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Size => write!(f, "Size"),
            Self::Speed => write!(f, "Speed"),
            Self::Type => write!(f, "Type"),
            Self::Subtype => write!(f, "Subtype"),
        }
    }
}

impl std::fmt::Display for RacialChoicesButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::RaceDescription => write!(f, "Race Description"),
            Self::StandardRacialTraitNames => write!(f, "Standard Racial Traits"),
            Self::AlternateRacialTraitNames => write!(f, "Alternate Racial Traits"),
            Self::RacialSubtypes => write!(f, "Racial Subtypes"),
            Self::FavoredClassOption => write!(f, "Favored Class Option"),
        }
    }
}

impl RacialChoicesButtonType {
    pub fn array() -> [RacialChoicesButtonType; 5] {
        [
            Self::RaceDescription,
            Self::StandardRacialTraitNames,
            Self::AlternateRacialTraitNames,
            Self::RacialSubtypes,
            Self::FavoredClassOption,
        ]
    }
}

// Makes sure custom assets are loaded before other functions try to use them.
pub fn setup_assets(
    mut races_asset_struct: ResMut<CustomAssetLoadState<RaceAsset>>,
    mut default_trait_struct: ResMut<CustomAssetLoadState<DefaultTraitAsset>>,
    asset_server: Res<AssetServer>,
) {
    let finding_assets = asset_server.load_folder(RACE_DESCRIPTION_FOLDER);
    if let Ok(found_assets) = finding_assets {
        println!("------->race asset found at {}", RACE_DESCRIPTION_FOLDER);
        for handle in found_assets {
            races_asset_struct.add_untyped(&handle);
        }
    } else {
        println!("-----race asset not found at {}", RACE_DESCRIPTION_FOLDER);
    }
    let finding_assets = asset_server.load_folder(RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER);
    if let Ok(found_assets) = finding_assets {
        println!(
            "------->trait asset found at {}",
            RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER
        );
        for handle in found_assets {
            default_trait_struct.add_untyped(&handle);
        }
    } else {
        println!(
            "-----trait asset not found at {}",
            RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER
        );
    }
}

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct FlavorTextSetup(bool);
pub fn setup_flavor_text(
    mut query_text: Query<&mut Text, With<DescriptionSection>>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<RaceAsset>>,
    mut has_run: ResMut<FlavorTextSetup>,
) {
    if !has_run.0 {
        println!("{:-<40}", "Starting flavor text setup");
        let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
        let mut text = query_text.get_single_mut().unwrap();
        println!("--- Assets {:#?}", assets.iter().next());
        for (_handle, race_asset) in assets.iter() {
            println!("inside loop");
            if race_asset.race == PlayableRace::Human {
                println!("{:-<50}", "Setting startup flavor text");
                *text = Text::from_section(
                    race_asset.text.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                )
            }
        }
        has_run.0 = true;
    }
}

// Manages the displayed flavor text for the selected race.
// Loads in text from a custom asset, which is ensured to be loaded by other
// systems earlier in the plugin.
pub fn selected_race_visibility(
    selected_race: Res<SelectedRaceButton>,

    mut query_text: Query<&mut Text, With<DescriptionSection>>,
    asset_server: Res<AssetServer>,
    //     asset_handles: Res<RacesLoadState>,
    assets: Res<Assets<RaceAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected_race.inner());
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let mut text = query_text.get_single_mut().unwrap();
    for (_handle, race_asset) in assets.iter() {
        if race_asset.race == selected_race.0 {
            *text = Text::from_section(
                race_asset.text.clone(),
                TextStyle {
                    font: font.clone(),
                    font_size: 30.,
                    color: Color::WHITE,
                },
            )
        }
    }
}

// Manages the selected race's default trait text description content.
pub fn selected_default_traits_visibility(
    selected_race: Res<SelectedRaceButton>,
    mut query_button: Query<
        &mut Text,
        (
            With<RacialTraitButtonText>,
            Without<DefaultTraitDescriptionText>,
        ),
    >,
    mut query_text: Query<
        &mut Text,
        (
            With<DefaultTraitDescriptionText>,
            Without<RacialTraitButtonText>,
        ),
    >,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<DefaultTraitAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected_race.inner());
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    // let mut text = query_text.get_single_mut().unwrap();
    for (_handle, trait_asset) in assets.iter() {
        if trait_asset.race == selected_race.0 {
            for (race_trait, (mut button, mut text)) in trait_asset
                .default_traits
                .iter()
                .zip(query_button.iter_mut().zip(query_text.iter_mut()))
            {
                *text = Text::from_section(
                    race_trait.description.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                );
                *button = Text::from_section(
                    race_trait.title.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.,
                        color: Color::WHITE,
                    },
                );
            }
        }
    }
}

// Hides the default racial trait description text when the race is not selected
// in the left race selection bar
pub fn hide_racial_trait_text(
    selected_race: Res<SelectedRaceButton>,
    mut query_text: Query<
        (&mut Style, &DefaultRacialTraitRace),
        (
            With<DefaultTraitDescriptionText>,
            Without<RacialTraitButton>,
        ),
    >,
) {
    for (mut style, race) in query_text.iter_mut() {
        if selected_race.inner() == race.0 {
            println!("Setting Display::Flex for DefaultTraitDescriptionText");
            style.display = bevy::ui::Display::Flex;
        } else {
            style.display = bevy::ui::Display::None;
        }
    }
}

// Hides the default racial trait button when the race is not selected
// in the left race selection bar
pub fn hide_racial_trait_button(
    selected_race: Res<SelectedRaceButton>,
    mut query_button: Query<
        (&mut Style, &DefaultRacialTraitRace),
        (
            With<RacialTraitButton>,
            Without<DefaultTraitDescriptionText>,
        ),
    >,
) {
    for (mut style, race) in query_button.iter_mut() {
        if selected_race.inner() == race.0 {
            println!("Setting Display::Flex for RacialTraitButton");
            style.display = bevy::ui::Display::Flex;
        } else {
            style.display = bevy::ui::Display::None;
        }
    }
}

// Display the selected race tab.
use super::components::RaceDescriptionNode;
pub fn display_racial_tab(
    mut query: Query<(&mut Style, &RaceDescriptionNode)>,
    selected: Res<SelectedRacialDescriptionType>,
) {
    let active_button = selected.inner();
    for (mut style, description_node) in query.iter_mut() {
        if description_node.inner() == active_button {
            println!("Setting Display::Flex for RaceDescriptionNode");
            style.display = bevy::ui::Display::Flex;
        } else if style.display == bevy::ui::Display::Flex {
            style.display = bevy::ui::Display::None;
        }
    }
}

// Makes sure other race buttons are the default color.
pub fn cleanup_selected_race_description_button(
    query_change: Query<(&Interaction, &RacialChoicesButtonType)>,
    mut query_others: Query<(&Interaction, &mut BackgroundColor, &RacialChoicesButtonType)>,
    selected_race: Res<SelectedRacialDescriptionType>,
) {
    if !query_change.is_empty() {
        for (interaction, mut color, button_type) in query_others.iter_mut() {
            if *interaction == Interaction::None && *button_type != selected_race.inner() {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

// Size,
// Speed,
// Type,
// Subtype,
use crate::systems::game::character::{CharacterSize, CreatureSubtypes, CreatureType, GroundSpeed};
use crate::systems::layout::character_creation::COMMON_TRAIT_FONT_SIZE;
use crate::systems::menu::styles::TEXT_COLOR;
pub fn update_common_traits_display(
    mut query: Query<(&mut Text, &CommonTraits)>,
    selected_race: Query<
        (
            &CreatureType,
            &CreatureSubtypes,
            &GroundSpeed,
            &CharacterSize,
        ),
        With<CharacterBuilder>,
    >,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let (creature_type, creature_subtypes, speed, size) = selected_race.get_single().unwrap();
    for (mut text, my_trait) in &mut query {
        match my_trait {
            // CommonTraits::Size => build_text(size.category.to_string(), font.clone()),
            // CommonTraits::Speed => build_text(speed.to_string(), font.clone()),
            // CommonTraits::Type => build_text(creature_type.to_string(), font.clone()),
            // // Subtype is a little trickier because Subtypes is a
            // // vec and a creature can have more than one subtype.
            // // Check the Display impl of CreatureSubtype if there
            // // are problems later.
            // // Returns a String in the form
            // // "Subtype1, Subtype2, Subtype3"
            // CommonTraits::Subtype => build_text(creature_subtypes.to_string(), font.clone()),
            CommonTraits::Size => {
                *text = Text::from_section(
                    size.category.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: COMMON_TRAIT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                )
            }
            CommonTraits::Speed => {
                *text = Text::from_section(
                    speed.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: COMMON_TRAIT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                )
            }
            CommonTraits::Type => {
                *text = Text::from_section(
                    creature_type.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: COMMON_TRAIT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                )
            }
            CommonTraits::Subtype => {
                *text = Text::from_section(
                    creature_subtypes.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: COMMON_TRAIT_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                )
            }
            _ => (),
        }
    }
}
pub fn reset_race(builder: Query<Entity, With<CharacterBuilder>>, mut commands: Commands) {
    let race = builder.get_single().unwrap();
    commands.get_entity(race).unwrap().despawn();
    commands.spawn(CharacterBuilder);
}
fn build_text(string: String, font: Handle<Font>) -> Text {
    // use crate::systems::layout::character_creation::COMMON_TRAIT_FONT_SIZE;
    Text::from_section(
        string.to_string(),
        TextStyle {
            font: font.clone(),
            font_size: COMMON_TRAIT_FONT_SIZE,
            color: TEXT_COLOR,
        },
    )
}

// Add default race traits to RaceBuilder when race is changed.
// This is required for the build_race system.
use crate::systems::game::race::{CharacterBuilder, RaceBuilder};
pub fn update_race_builder(
    mut race_builder: ResMut<RaceBuilder>,
    selected_race: Res<SelectedRaceButton>,
) {
    race_builder.inner_mut().clear();
    race_builder
        .inner_mut()
        .append(&mut RacialTraitName::default_traits(&selected_race.inner()));
}

// Holds the currently selected race for reference by other functions.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct SelectedRacialDescriptionType(pub RacialChoicesButtonType);
impl SelectedRacialDescriptionType {
    fn inner(&self) -> RacialChoicesButtonType {
        self.0
    }
}

// Changes the color of the selected racial tab button
pub fn selected_race_description_type(
    mut selected: ResMut<SelectedRacialDescriptionType>,
    mut interaction_query: Query<
        (&Interaction, &RacialChoicesButtonType, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    let selection_copy = selected.inner();
    for (interaction, interacted_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected = SelectedRacialDescriptionType(*interacted_button);
                }
            }
            Interaction::Hovered => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// Changes the color of the selected race button
pub fn race_select_button_system(
    mut interaction_query: Query<
        (&Interaction, &PlayableRace, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_race: ResMut<SelectedRaceButton>,
) {
    let selection_copy = (*selected_race).inner();
    for (interaction, player_race, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *player_race {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected_race = SelectedRaceButton(*player_race);
                }
            }
            Interaction::Hovered => {
                if selection_copy != *player_race {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if selection_copy != *player_race {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// Makes sure other race buttons are the default color.
pub fn cleanup_race_select_button(
    // query_change: Query<(&Interaction, &PlayableRace, &RaceSelectButton)>,
    query_change: Query<&RaceSelectButton, Changed<Interaction>>,
    mut query_others: Query<(
        &PlayableRace,
        &Interaction,
        &mut BackgroundColor,
        &RaceSelectButton,
    )>,
    selected_race: Res<SelectedRaceButton>,
) {
    if !query_change.is_empty() {
        for (player_race, interaction, mut color, _) in query_others.iter_mut() {
            if *interaction == Interaction::None && *player_race != selected_race.0 {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn cleanup_race_description_type_button(
    query_change: Query<&RacialChoicesButtonType, Changed<Interaction>>,
    mut query_others: Query<(&RacialChoicesButtonType, &Interaction, &mut BackgroundColor)>,
    selected_description_type: Res<SelectedRacialDescriptionType>,
) {
    if !query_change.is_empty() {
        for (description_button, interaction, mut color) in query_others.iter_mut() {
            if *interaction == Interaction::None
                && *description_button != selected_description_type.inner()
            {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct TestChosenStandardTrait;

pub fn track_trait(query: Query<&Style, (With<TestChosenStandardTrait>, Changed<Style>)>) {
    for changed_style in &query {
        println!("{:?}", changed_style.display);
    }
}
