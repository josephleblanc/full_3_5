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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub enum CreationTab {
    #[default]
    Race,
    Class,
}

//// Alt Trait description screen labels
// Node containing a whole row
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltTraitNode;

// Title of alt trait
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltRacialTraitTitle;

// Button to select the alt racial trait
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltTraitSelectButton;
// Text in button to select the alt racial trait,
// should be the title of the racial trait
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltTraitSelectButtonText;
// A vec of the standard traits this alternate trait replaces_names.
#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AltTraitReplaces(pub Vec<RacialTraitName>);

// Text with the description content of the alt trait
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltTraitDescription;

// Labels for shared components of trait lists
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct AltTraitParent;

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

pub fn display_alt_traits(
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_parent_style: Query<(&mut Style), With<ListParent>>,
) {
    let mut parent_style = query_parent_style.get_single_mut().unwrap();
    if selected_race_tab.0 == RaceTab::AltTraitTab {
        parent_style.display = Display::Flex;
    } else {
        parent_style.display = Display::None;
    }
}

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

use crate::technical::alternate_traits::{AltTraitAsset, AltTraitDisplay};
pub fn fill_alt_traits(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut set: ParamSet<(
        // query parent node
        Query<Entity, /* &mut Style),*/ With<ListParent>>,
        // query title
        Query<&mut Text, With<ListTitle>>,
        // query replaces_names text & list
        Query<&mut AltTraitReplaces, With<AltTraitReplaces>>,
        // query description text
        Query<&mut Text, With<Description>>,
        // query row node
        Query<&mut Style, With<ListNode>>,
        // query_children
        Query<&Children>,
    )>,
    asset_server: Res<AssetServer>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
    println!("-------------------------starting alt_trait_visibility-----------------------");
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let mut alt_traits: Vec<AltTraitDisplay> = Vec::new();

    let mut children: Vec<Entity> = Vec::new();
    let mut parent_entity: Option<Entity> = None;
    if let Ok(entity /*mut node_style)*/) = set.p0().get_single_mut() {
        println!("inside parent node display");
        // get parent_entity copied out of this context
        parent_entity = Some(entity);
    }
    let parent_entity = parent_entity.unwrap();
    let children: Vec<Entity> = set
        .p5()
        .iter_descendants(parent_entity)
        .collect::<Vec<Entity>>();

    // If no asset is loaded, set display to Display::None for all alt trait nodes
    // This prevents the previously selected race's alt traits from being displayed
    // when no alt traits are found.
    if !alt_trait_asset
        .iter()
        .any(|(_handle, list_asset)| list_asset.race == selected_race.0)
    {
        if let mut iter = set.p4().iter_many_mut(&children) {
            while let Some(mut node_style) = iter.fetch_next() {
                node_style.display = Display::None;
            }
        }
    } else {
        // If a match for the selected race is found in the loaded alt_trait_asset, then
        // iterate over all the alt trait nodes and set the titles, replacement traits,
        // and description text.
        for (_handle, list_asset) in alt_trait_asset.iter() {
            println!(
                "is alt_trait a match? {}",
                list_asset.race == selected_race.inner()
            );

            if list_asset.race == selected_race.inner() {
                alt_traits = list_asset.alternate_traits.clone();

                let mut traits_len = alt_traits.len();
                println!("traits_len = {}", traits_len);
                let titles: Vec<String>;
                let replacements: Vec<Vec<RacialTraitName>>;
                let descriptions: Vec<String>;
                (titles, (replacements, descriptions)) = alt_traits
                    .into_iter()
                    .map(|alt_display| {
                        (
                            alt_display.title,
                            (alt_display.replaces_names, alt_display.description),
                        )
                    })
                    .unzip();

                let mut titles_iter = titles.into_iter();
                if let mut iter = set.p1().iter_many_mut(&children) {
                    println!("--> inside first param if let");
                    while let Some(mut title) = iter.fetch_next() {
                        println!("----> inside first param loop");
                        // change title text
                        if let Some(trait_title) = titles_iter.next() {
                            println!(
                                "------> inside first alt_trait if let, title = {}",
                                trait_title.clone()
                            );
                            *title = Text::from_section(
                                trait_title,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 25.,
                                    color: Color::WHITE,
                                },
                            );
                        }
                    }
                }

                let mut replacements_iter = replacements.into_iter();
                if let mut iter = set.p2().iter_many_mut(&children) {
                    while let Some(mut replaces_names_list) = iter.fetch_next() {
                        if let Some(trait_replaces_names) = replacements_iter.next() {
                            // change replacement list
                            *replaces_names_list = AltTraitReplaces(trait_replaces_names);
                        }
                    }
                }

                let mut descriptions_iter = descriptions.into_iter();
                if let mut iter = set.p3().iter_many_mut(&children) {
                    while let Some(mut text) = iter.fetch_next() {
                        if let Some(trait_description) = descriptions_iter.next() {
                            *text = Text::from_section(
                                trait_description,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 25.,
                                    color: Color::WHITE,
                                },
                            );
                        }
                        // change text description
                    }
                }

                if let mut iter = set.p4().iter_many_mut(&children) {
                    while let Some(mut node_style) = iter.fetch_next() {
                        if traits_len > 0 {
                            traits_len -= 1;
                            node_style.display = Display::Flex;
                        } else {
                            node_style.display = Display::None;
                        }
                    }
                }
            }
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
        (&mut Text, &mut Style),
        (
            With<RacialTraitButtonText>,
            Without<DefaultTraitDescriptionText>,
        ),
    >,
    mut query_text: Query<
        (&mut Text, &mut Style),
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
            let traits_len = trait_asset.default_traits.len();

            for (i, ((mut button_text, mut button_style), (mut descr_text, mut descr_style))) in
                query_button
                    .iter_mut()
                    .zip(query_text.iter_mut())
                    .enumerate()
            {
                if i < traits_len {
                    let race_trait = &trait_asset.default_traits[i];
                    *descr_text = Text::from_section(
                        race_trait.description.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    );
                    *button_text = Text::from_section(
                        race_trait.title.clone(),
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.,
                            color: Color::WHITE,
                        },
                    );
                } else {
                    descr_style.display = Display::None;
                    button_style.display = Display::None;
                }
            }
        }
    }
}

pub fn cleanup_standard_trait_nodes(
    selected_race: Res<SelectedRaceButton>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<DefaultTraitAsset>>,
    node_style: Query<&mut Style, With<RaceDescriptionNode>>,
) {
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
    selected: Res<SelectedRaceTab>,
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
