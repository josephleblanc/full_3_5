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
const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/race/default_traits/";
// Stores the race selected by the player
// Also used to make sure only one selected button has its background changed.
#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct SelectedRaceButton(pub PlayableRace);

impl SelectedRaceButton {
    fn copy_inner(&self) -> PlayableRace {
        self.0
    }
}

#[derive(Component, Clone, Debug)]
pub struct ActiveRaceDescription(pub PlayableRace);

#[derive(Component, Clone, Copy, Debug)]
pub struct DescriptionSection;

#[derive(Component, Clone, Copy, Debug)]
pub struct RacialChoiceButton;

#[derive(Component, Clone, Debug)]
pub struct RacialChoicesButton;

#[derive(Component, Clone, Debug)]
pub struct RightPanel;

#[derive(Component, Clone, Debug)]
pub struct RacialTraitNameSelections {
    traits_chosen: Vec<RacialTraitName>,
    race: PlayableRace,
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum RacialChoicesButtonType {
    #[default]
    RaceDescription,
    StandardRacialTraitNames,
    AlternateRacialTraitNames,
    RacialSubtypes,
    FavoredClassOption,
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

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitListNumber(pub usize);

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButtonText;
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButton;
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitDescriptionText;
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct DefaultRacialTraitRace(pub PlayableRace);

// Keeping this here for debuggin purposes
// fn print_custom_assets(races_state: Res<RacesLoadState>, assets: Res<Assets<RaceAsset>>) {
//     for handle in races_state.handles.iter() {
//         let loaded_text = &assets
//             .get(&handle.clone().typed::<RaceAsset>())
//             .unwrap()
//             .text;
//         println!("{loaded_text}");
//     }
// }

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
    } else {
        println!(
            "trait asset not found at {}",
            RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER
        );
    }
}

pub fn selected_race_visibility(
    selected_race: Res<SelectedRaceButton>,

    mut query_text: Query<&mut Text, With<DescriptionSection>>,
    asset_server: Res<AssetServer>,
    //     asset_handles: Res<RacesLoadState>,
    assets: Res<Assets<RaceAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected_race.0);
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

pub fn selected_default_traits_visibility(
    selected_race: Res<SelectedRaceButton>,
    mut query_button: Query<
        (&mut Text, &RacialTraitListNumber),
        (
            With<RacialTraitButtonText>,
            Without<RacialTraitDescriptionText>,
        ),
    >,
    mut query_text: Query<
        (&mut Text, &RacialTraitListNumber),
        (
            With<RacialTraitDescriptionText>,
            Without<RacialTraitButtonText>,
        ),
    >,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<DefaultTraitAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected_race.0);
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    // let mut text = query_text.get_single_mut().unwrap();
    for (_handle, trait_asset) in assets.iter() {
        if trait_asset.race == selected_race.0 {
            for (race_trait, ((mut button, button_i), (mut text, text_i))) in trait_asset
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
        (With<RacialTraitDescriptionText>, Without<RacialTraitButton>),
    >,
) {
    for (mut style, race) in query_text.iter_mut() {
        if selected_race.copy_inner() == race.0 {
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
        (With<RacialTraitButton>, Without<RacialTraitDescriptionText>),
    >,
) {
    for (mut style, race) in query_button.iter_mut() {
        if selected_race.copy_inner() == race.0 {
            style.display = bevy::ui::Display::Flex;
        } else {
            style.display = bevy::ui::Display::None;
        }
    }
}

use super::components::RaceDescriptionNode;
pub fn display_racial_description_type(
    mut query: Query<(&mut Style, &RaceDescriptionNode)>,
    selected: Res<SelectedRacialDescriptionType>,
) {
    let active_button = selected.inner();
    for (mut style, description_node) in query.iter_mut() {
        if description_node.inner() == active_button {
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

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct SelectedRacialDescriptionType(RacialChoicesButtonType);
impl SelectedRacialDescriptionType {
    fn inner(&self) -> RacialChoicesButtonType {
        self.0
    }
}

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
                    println!("{selected:?}");
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
    let selection_copy = (*selected_race).copy_inner();
    for (interaction, player_race, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if selection_copy != *player_race {
                    *color = RACE_BUTTON_COLOR_SELECTED.into();
                    *selected_race = SelectedRaceButton(*player_race);
                    println!("{selected_race:?}");
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
    query_change: Query<(&Interaction, &PlayableRace, &RaceSelectButton)>,
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
