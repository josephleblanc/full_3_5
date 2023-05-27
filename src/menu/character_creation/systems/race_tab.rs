use crate::menu::character_creation::components::*;
use crate::menu::styles::*;
use crate::systems::game::character::{CharacterSize, CreatureSubtypes, CreatureType, GroundSpeed};
use crate::systems::game::race::{CharacterBuilder, RaceBuilder};
use crate::systems::layout::character_creation::COMMON_TRAIT_FONT_SIZE;
use crate::technical::alternate_traits::AltTraitAsset;
use crate::{
    systems::game::race::RacialTraitName,
    technical::{
        default_race_traits::DefaultTraitAsset, favored_class::FavoredClassAsset,
        race_load::RaceAsset,
    },
};
use bevy::prelude::*;

pub fn list_node(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<&mut Style, (With<ListNode>, With<RaceItem>)>,
    std_trait_asset: Res<Assets<DefaultTraitAsset>>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
    favored_class_asset: Res<Assets<FavoredClassAsset>>,
) {
    let len = match selected_race_tab.0 {
        RaceTab::RaceDescription => Some(1_usize),
        RaceTab::StandardTraitTab => {
            if let Some((_handle, asset)) = std_trait_asset
                .iter()
                .filter(|(_handle, asset)| asset.race == selected_race.inner())
                .next()
            {
                Some(asset.default_traits.len())
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

// Set title text for elements of the list in the central display while in the
// Race tab of character creation.
pub fn set_list_title(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_title: Query<(&mut Style, &mut Text), (With<ListTitle>, With<RaceItem>)>,
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

// Set the content of the column in a list element, located on the left, and
// containing the replacement text for alternate traits in the race tab of
// character creation.
pub fn button_col(
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
// The static value of replacement text for choosing alternate traits in the
// alternate traits subtab of race selection in character creation.
pub fn replacement_text(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<(&mut Style, &mut Text), (With<ReplacesText>, With<RaceItem>)>,
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
// Sets the display of the replaces node in the left side of the row for an alternate
// trait. Should not be displayed for other race subtabs.
pub fn replace_node(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<&mut Style, (With<ReplacesContent>, With<RaceItem>)>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
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
// Sets the text of the replaced traits in the left column of the alternate traits
// list element of the race tab in character creation.
pub fn replace_text(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_node: Query<(&mut Style, &mut Text, &mut AltTraitReplaces)>,
    asset_server: Res<AssetServer>,
    alt_trait_asset: Res<Assets<AltTraitAsset>>,
) {
    // println!("running set_replaces_display");
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
            } else {
                None
            }
        }
        RaceTab::FavoredClassTab => None,
    };
    if let Some(mut traits_iter) = traits {
        for (mut style, mut text, mut displayed_replaces) in query_node.iter_mut() {
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
// Handle the description text display in the race tab.
pub fn description(
    selected_race: Res<SelectedRaceButton>,
    selected_race_tab: Res<SelectedRaceTab>,
    mut query_descr: Query<(&mut Style, &mut Text), (With<Description>, With<RaceItem>)>,
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

// Race subtab buttons - manages color if hovered or clicked.
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
            } // _ => (),
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
pub fn update_race_builder(
    mut race_builder: ResMut<RaceBuilder>,
    selected_race: Res<SelectedRaceButton>,
) {
    race_builder.inner_mut().clear();
    race_builder
        .inner_mut()
        .append(&mut RacialTraitName::default_traits(&selected_race.inner()));
}
