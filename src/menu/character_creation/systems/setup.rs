use crate::{
    menu::character_creation::{components::*, constants::*},
    systems::game::character::PlayableRace,
    technical::{
        alternate_traits::AltTraitAsset, class::ClassAsset, default_race_traits::DefaultTraitAsset,
        is_custom_asset_loaded::CustomAssetLoadState, race_load::RaceAsset,
    },
};
use bevy::prelude::*;

pub fn setup_assets(
    mut races_asset_struct: ResMut<CustomAssetLoadState<RaceAsset>>,
    mut default_trait_struct: ResMut<CustomAssetLoadState<DefaultTraitAsset>>,
    mut alt_trait_struct: ResMut<CustomAssetLoadState<AltTraitAsset>>,
    mut class_asset_struct: ResMut<CustomAssetLoadState<ClassAsset>>,
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
    let finding_assets = asset_server.load_folder(RACIAL_ALT_TRAITS_FOLDER);
    if let Ok(found_assets) = finding_assets {
        for handle in found_assets {
            alt_trait_struct.add_untyped(&handle);
        }
    }
    let finding_assets = asset_server.load_folder(CLASS_DESCRIPTIONS_FOLDER);
    if let Ok(found_assets) = finding_assets {
        for handle in found_assets {
            class_asset_struct.add_untyped(&handle);
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
        // println!("{:-<40}", "Starting flavor text setup");
        let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
        let mut text = query_text.get_single_mut().unwrap();
        // println!("--- Assets {:#?}", assets.iter().next());
        for (_handle, race_asset) in assets.iter() {
            // println!("inside loop");
            if race_asset.race == PlayableRace::Human {
                // println!("{:-<50}", "Setting startup flavor text");
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
