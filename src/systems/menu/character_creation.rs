use crate::technical::alternate_traits::AltTraitAsset;
use crate::{
    systems::{
        game::character::PlayableRace,
        game::race::RacialTraitName,
        menu::{
            components::RaceSelectButton,
            styles::{
                LIST_BUTTON_TEXT_SIZE, RACE_BUTTON_COLOR, RACE_BUTTON_COLOR_HOVERED,
                RACE_BUTTON_COLOR_SELECTED,
            },
        },
    },
    technical::{
        default_race_traits::DefaultTraitAsset, favored_class::FavoredClassAsset,
        is_custom_asset_loaded::CustomAssetLoadState, race_load::RaceAsset,
    },
};
use bevy::prelude::*;
const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";
const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/race/default_traits";
const CLASS_DESCRIPTIONS_FOLDER: &str = "text/descriptions/class";
// Stores the race selected by the player
// Also used to make sure only one selected button has its background changed.
#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct SelectedRaceButton(pub PlayableRace);

impl SelectedRaceButton {
    fn inner(&self) -> PlayableRace {
        self.0
    }
}

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct CreationTabSelected(pub CreationTab);

impl CreationTabSelected {
    fn inner(&self) -> CreationTab {
        self.0
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub enum CreationTab {
    #[default]
    Race,
    AbilityScores,
    Class,
    Skills,
    Feats,
    BonusFeats,
    Optional,
}
#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AltTraitReplaces(pub Vec<RacialTraitName>);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListParent;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListNode;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ButtonText;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Description;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListButtonColumn;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ReplacesText;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ReplacesContent;

// #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
// pub struct AlternateTrait;

pub fn set_list_node_display(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<&mut Style, With<ListNode>>,
    std_trait_asset: Res<Assets<DefaultTraitAsset>>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
    favored_class_asset: Res<Assets<FavoredClassAsset>>,
) {
    println!("--> running set_list_node_display");
    let len = match selected_race_tab.0 {
        RaceTab::RaceDescription => Some(1_usize),
        RaceTab::StandardTraitTab => {
            if let Some((_handle, asset)) = std_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                println!("----> race match found for standard trait");
                Some(asset.default_traits.len())
            } else {
                println!("----> race match not found");
                None
            }
        }
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.alternate_traits.len())
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => {
            if let Some((_handle, asset)) = favored_class_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.favored_classes.len())
            } else {
                None
            }
        }
    };
    if let Some(len) = len {
        println!("------> len = {}", len);
        for (i, mut style) in query_node.iter_mut().enumerate() {
            if i < len {
                println!("      > set display to Flex");
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for mut style in &mut query_node {
            style.display = Display::None;
        }
    }
}

pub fn set_list_title(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_title: Query<(&mut Style, &mut Text), With<ListTitle>>,
    asset_server: Res<AssetServer>,
    std_trait_asset: Res<Assets<DefaultTraitAsset>>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
    favored_class_asset: Res<Assets<FavoredClassAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let titles = match selected_race_tab.0 {
        RaceTab::RaceDescription => Some(vec!["Description".to_string()]),
        RaceTab::StandardTraitTab => {
            let titles: Vec<String>;
            if let Some((_handle, asset)) = std_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                titles = asset
                    .default_traits
                    .iter()
                    .map(|traits| traits.title.clone())
                    .collect();
                Some(titles)
            } else {
                None
            }
        }
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(
                    asset
                        .alternate_traits
                        .iter()
                        .map(|traits| traits.title.clone())
                        .collect(),
                )
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => {
            if let Some((_handle, asset)) = favored_class_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(
                    asset
                        .favored_classes
                        .iter()
                        .map(|favored_class| favored_class.class.to_string())
                        .collect(),
                )
            } else {
                None
            }
        }
    };
    if let Some(titles) = titles {
        let mut titles_iter = titles.iter();
        for (mut style, mut text) in query_title.iter_mut() {
            if let Some(title) = titles_iter.next() {
                *text = Text::from_section(
                    title,
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                );
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (mut style, _) in &mut query_title {
            style.display = Display::None;
        }
    }
}

pub fn set_button_col_display(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_button: Query<&mut Style, (With<ListButtonColumn>, Without<ButtonText>)>,
    mut query_button_text: Query<
        (&mut Style, &mut Text),
        (With<ButtonText>, Without<ListButtonColumn>),
    >,
    asset_server: Res<AssetServer>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
    favored_class_asset: Res<Assets<FavoredClassAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let len = match selected_race_tab.0 {
        RaceTab::RaceDescription => None,
        RaceTab::StandardTraitTab => None,
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.alternate_traits.len())
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => {
            if let Some((_handle, asset)) = favored_class_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.favored_classes.len())
            } else {
                None
            }
        }
    };
    if let Some(len) = len {
        for (i, ((mut text_style, mut button_text), mut button_style)) in query_button_text
            .iter_mut()
            .zip(query_button.iter_mut())
            .enumerate()
        {
            if i < len {
                *button_text = Text::from_section(
                    "Select",
                    TextStyle {
                        font: font.clone(),
                        font_size: LIST_BUTTON_TEXT_SIZE,
                        color: TEXT_COLOR,
                    },
                );
                button_style.display = Display::Flex;
                text_style.display = Display::Flex;
            } else {
                button_style.display = Display::None;
                text_style.display = Display::None;
            }
        }
    } else {
        for mut button_style in &mut query_button {
            button_style.display = Display::None;
        }
    }
}

pub fn set_skill_replacement_text(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<(&mut Style, &mut Text), With<ReplacesText>>,
    asset_server: Res<AssetServer>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let len = match selected_race_tab.0 {
        RaceTab::RaceDescription => None,
        RaceTab::StandardTraitTab => None,
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.alternate_traits.len())
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => None,
    };
    if let Some(len) = len {
        for (i, (mut style, mut text)) in query_node.iter_mut().enumerate() {
            if i < len {
                *text = Text::from_section(
                    "Replaces",
                    TextStyle {
                        font: font.clone(),
                        font_size: LIST_BUTTON_TEXT_SIZE,
                        color: TEXT_COLOR,
                    },
                );
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (mut style, _) in &mut query_node {
            style.display = Display::None;
        }
    }
}

pub fn set_replace_display(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<&mut Style, With<ReplacesContent>>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
    let len = match selected_race_tab.0 {
        RaceTab::RaceDescription => Some(1_usize),
        RaceTab::StandardTraitTab => None,
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.alternate_traits.len())
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => None,
    };
    if let Some(len) = len {
        for (i, mut style) in query_node.iter_mut().enumerate() {
            if i < len {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for mut style in &mut query_node {
            style.display = Display::None;
        }
    }
}

pub fn set_replaced_content_display(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<(&mut Style, &mut Text, &mut AltTraitReplaces)>,
    asset_server: Res<AssetServer>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
    println!("running set_replaces_display");
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let traits = match selected_race_tab.0 {
        RaceTab::RaceDescription => None,
        RaceTab::StandardTraitTab => None,
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.alternate_traits.iter().map(|alt_trait| {
                    (
                        alt_trait.replaces_names.iter(),
                        alt_trait.replaces_strings.iter(),
                    )
                }))
                // Some(
                //     asset
                //         .alternate_traits
                //         .iter()
                //         .map(|alt_trait| (alt_trait.replaces_names, alt_trait.replaces_strings))
                //         .unzip(),
                // )
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => None,
    };
    if let Some(mut traits_iter) = traits {
        for (i, (mut style, mut text, mut displayed_replaces)) in query_node.iter_mut().enumerate()
        {
            println!("i: {}", i);
            if let Some((trait_names, replaces_strings)) = traits_iter.next() {
                *text = Text::from_sections(replaces_strings.map(|trait_name| {
                    TextSection::new(
                        trait_name,
                        TextStyle {
                            font: font.clone(),
                            font_size: LIST_BUTTON_TEXT_SIZE,
                            color: TEXT_COLOR,
                        },
                    )
                }));
                style.display = Display::Flex;
                displayed_replaces.0.extend(trait_names);
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (mut style, _, _) in &mut query_node {
            style.display = Display::None;
        }
    }
}

pub fn set_list_descr(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_descr: Query<(&mut Style, &mut Text), With<Description>>,
    asset_server: Res<AssetServer>,
    descr_asset: Res<Assets<RaceAsset>>,
    std_trait_asset: Res<Assets<DefaultTraitAsset>>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
    favored_class_asset: Res<Assets<FavoredClassAsset>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let descr: Option<Vec<&String>> = match selected_race_tab.0 {
        RaceTab::RaceDescription => Some(
            descr_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .map(|(_handle, descr_asset)| &descr_asset.text)
                .collect(),
        ),
        RaceTab::StandardTraitTab => {
            if let Some((_handle, asset)) = std_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(
                    asset
                        .default_traits
                        .iter()
                        .map(|traits| &traits.description)
                        .collect(),
                )
            } else {
                None
            }
        }
        RaceTab::AltTraitTab => {
            if let Some((_handle, asset)) = alt_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(
                    asset
                        .alternate_traits
                        .iter()
                        .map(|traits| &traits.description)
                        .collect(),
                )
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => {
            if let Some((_handle, asset)) = favored_class_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(
                    asset
                        .favored_classes
                        .iter()
                        .map(|traits| &traits.description)
                        .collect(),
                )
            } else {
                None
            }
        }
    };
    if let Some(descriptions) = descr {
        let mut descriptions_iter = descriptions.iter();
        for (mut style, mut text) in query_descr.iter_mut() {
            if let Some(text_descr) = descriptions_iter.next() {
                *text = Text::from_section(
                    *text_descr,
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                );
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (mut style, _) in &mut query_descr {
            style.display = Display::None;
        }
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
// Tooltip labels
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

#[derive(Component, Clone, Copy, Debug)]
pub struct DescriptionSection;

// #[derive(Component, Clone, Debug)]
// pub struct RacialChoicesButton;

#[derive(Component, Clone, Debug)]
pub struct RightPanel;

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
pub enum RaceTab {
    #[default]
    RaceDescription,
    StandardTraitTab,
    AltTraitTab,
    FavoredClassTab,
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

impl std::fmt::Display for RaceTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::RaceDescription => write!(f, "Race Description"),
            Self::StandardTraitTab => write!(f, "Standard Racial Traits"),
            Self::AltTraitTab => write!(f, "Alternate Racial Traits"),
            Self::FavoredClassTab => write!(f, "Favored Class Option"),
        }
    }
}

impl RaceTab {
    pub fn array() -> [RaceTab; 4] {
        [
            Self::RaceDescription,
            Self::StandardTraitTab,
            Self::AltTraitTab,
            Self::FavoredClassTab,
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
        for handle in found_assets {
            races_asset_struct.add_untyped(&handle);
        }
    }
    let finding_assets = asset_server.load_folder(RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER);
    if let Ok(found_assets) = finding_assets {
        for handle in found_assets {
            default_trait_struct.add_untyped(&handle);
        }
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

// Display the selected race tab.
use super::components::RaceDescriptionNode;

// Makes sure other race buttons are the default color.
pub fn cleanup_selected_race_description_button(
    query_change: Query<(&Interaction, &RaceTab)>,
    mut query_others: Query<(&Interaction, &mut BackgroundColor, &RaceTab)>,
    selected_race: Res<SelectedRaceTab>,
) {
    if !query_change.is_empty() {
        for (interaction, mut color, button_type) in query_others.iter_mut() {
            if *interaction == Interaction::None && *button_type != selected_race.inner() {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

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
pub struct SelectedRaceTab(pub RaceTab);
impl SelectedRaceTab {
    fn inner(&self) -> RaceTab {
        self.0
    }
}

// Changes the color of the selected racial tab button
pub fn selected_race_description_type(
    mut selected: ResMut<SelectedRaceTab>,
    mut interaction_query: Query<
        (&Interaction, &RaceTab, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    let selection_copy = selected.inner();
    for (interaction, interacted_button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *interacted_button {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected = SelectedRaceTab(*interacted_button);
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

#[derive(Resource, Copy, Clone, Debug, Default, PartialEq)]
pub struct SelectedClass(pub PlayableClass);
impl SelectedClass {
    pub fn inner(&self) -> PlayableClass {
        self.0
    }
}
// Changes the color of the selected left panel button
pub fn race_select_button_system(
    mut interaction_query: Query<
        (&Interaction, &LeftPanelEnum, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_race: ResMut<SelectedRaceButton>,
    mut selected_class: ResMut<SelectedClass>,
) {
    let selection_copy = (*selected_race).inner();
    for (interaction, left_enum, mut color) in &mut interaction_query {
        if let Some(player_race) = left_enum.get_race() {
            match *interaction {
                Interaction::Clicked => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected_race = SelectedRaceButton(player_race);
                    }
                }
                Interaction::Hovered => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if selection_copy != player_race {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        } else if let Some(player_class) = left_enum.get_class() {
            match *interaction {
                Interaction::Clicked => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected_class = SelectedClass(player_class);
                    }
                }
                Interaction::Hovered => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if player_class != selected_class.inner() {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        }
    }
}
use crate::systems::game::class::PlayableClass;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum LeftPanelEnum {
    Race(PlayableRace),
    Class(PlayableClass),
}
impl LeftPanelEnum {
    pub fn get_race(&self) -> Option<PlayableRace> {
        match self {
            Self::Race(playable_race) => Some(*playable_race),
            _ => None,
        }
    }
    pub fn get_class(&self) -> Option<PlayableClass> {
        match self {
            Self::Class(playable_class) => Some(*playable_class),
            _ => None,
        }
    }
}
impl Default for LeftPanelEnum {
    fn default() -> LeftPanelEnum {
        LeftPanelEnum::Race(PlayableRace::Human)
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct LeftPanelButton;
// Makes sure other race buttons are the default color.
pub fn cleanup_race_select_button(
    query_change: Query<&LeftPanelButton, Changed<Interaction>>,
    mut query_others: Query<(
        &LeftPanelEnum,
        &Interaction,
        &mut BackgroundColor,
        &LeftPanelButton,
    )>,
    selected_race: Res<SelectedRaceButton>,
) {
    if !query_change.is_empty() {
        for (player_race, interaction, mut color, _) in query_others.iter_mut() {
            if let Some(player_race) = player_race.get_race() {
                if *interaction == Interaction::None && player_race != selected_race.0 {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

pub fn cleanup_race_description_type_button(
    query_change: Query<&RaceTab, Changed<Interaction>>,
    mut query_others: Query<(&RaceTab, &Interaction, &mut BackgroundColor)>,
    selected_description_type: Res<SelectedRaceTab>,
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

// Changes the color of the selected race button
pub fn creation_tab(
    mut interaction_query: Query<
        (&Interaction, &CreationTab, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut selected_tab: ResMut<CreationTabSelected>,
) {
    let selection_copy = (*selected_tab).inner();
    for (interaction, tab, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected_tab = CreationTabSelected(*tab);
                }
            }
            Interaction::Hovered => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR_HOVERED.into();
                }
            }
            Interaction::None => {
                if selection_copy != *tab {
                    *color = RACE_BUTTON_COLOR.into();
                }
            }
        }
    }
}

// Makes sure other race buttons are the default color.
pub fn cleanup_creation_tab(
    query_change: Query<&CreationTab, Changed<Interaction>>,
    mut query_others: Query<(&CreationTab, &Interaction, &mut BackgroundColor)>,
    selected_tab: Res<CreationTabSelected>,
) {
    if !query_change.is_empty() {
        for (tab, interaction, mut color) in query_others.iter_mut() {
            if *interaction == Interaction::None && *tab != selected_tab.inner() {
                *color = RACE_BUTTON_COLOR.into();
            }
        }
    }
}

// #[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
// pub struct SubTabParent;
// #[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
// pub struct SubTabNode;
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct SubTabButton;
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct SubTabButtonText;

impl SubTabButton {
    pub fn display(
        mut query_button: Query<(&mut Style, &RaceTab), With<SubTabButton>>,
        selected_tab: Res<CreationTabSelected>,
    ) {
        match selected_tab.into_inner().inner() {
            CreationTab::Race => {
                let race_subtabs = RaceTab::array();
                for (i, (mut button_style, race_tab)) in query_button.iter_mut().enumerate() {
                    if i < race_subtabs.len() {
                        button_style.display = Display::Flex;
                    } else {
                        button_style.display = Display::None;
                    }
                }
            }
            _ => {
                for (i, (mut button_style, race_tab)) in query_button.iter_mut().enumerate() {
                    button_style.display = Display::None;
            }
            }
        }
    }
}
