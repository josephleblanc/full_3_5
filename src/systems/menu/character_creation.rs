use crate::{
    systems::{
        game::character::PlayableRace,
        game::race::RacialTraitName,
        menu::{
            components::RaceSelectButton,
            styles::{RACE_BUTTON_COLOR, RACE_BUTTON_COLOR_HOVERED, RACE_BUTTON_COLOR_SELECTED},
        },
    },
    technical::is_custom_asset_loaded::CustomAssetLoadState,
    technical::race_load::RaceAsset,
};
use bevy::prelude::*;
const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";

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

#[derive(Component, Clone, Copy, Debug, Default)]
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
    asset_server: Res<AssetServer>,
) {
    let finding_assets = asset_server.load_folder(RACE_DESCRIPTION_FOLDER);
    if let Ok(found_assets) = finding_assets {
        for handle in found_assets {
            races_asset_struct.add_untyped(&handle);
        }
    }
}

pub fn selected_race_visibility(
    selected: Res<SelectedRaceButton>,
    mut query_text: Query<&mut Text, With<DescriptionSection>>,
    asset_server: Res<AssetServer>,
    //     asset_handles: Res<RacesLoadState>,
    assets: Res<Assets<RaceAsset>>,
) {
    println!("selected_race_visibility: {:?}", selected.0);
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let mut text = query_text.get_single_mut().unwrap();
    for (_handle, race_asset) in assets.iter() {
        if race_asset.race == selected.0 {
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
