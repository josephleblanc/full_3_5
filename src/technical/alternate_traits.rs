//! Implements loader for a custom asset type.

use crate::systems::game::character::PlayableRace;
use crate::systems::game::race::RacialTraitName;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

// A sub-struct for AltTraitAsset with the info needed to display
// it in the character creation screen.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct AltTraitDisplay {
    pub my_trait_name: RacialTraitName,
    pub title: String,
    pub description: String,
    pub replaces: Vec<RacialTraitName>,
    pub source: String,
}

#[derive(Debug, Deserialize, TypeUuid, Default)]
#[uuid = "06c14941-a310-4f2d-bbf0-4abe3dec0847"]
pub struct AltTraitAsset {
    pub race: PlayableRace,
    pub alternate_traits: Vec<AltTraitDisplay>,
}

#[derive(Default)]
pub struct AltTraitAssetLoader;

impl AssetLoader for AltTraitAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<AltTraitAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["alt_trait.ron"]
    }
}

pub struct MyAltTraitAssetPlugin;

impl Plugin for MyAltTraitAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultTraitState>()
            .add_asset::<AltTraitAsset>()
            .init_asset_loader::<AltTraitAssetLoader>();
        // .add_startup_system(setup)
        // .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct DefaultTraitState {
    handle: Handle<AltTraitAsset>,
    printed: bool,
}

pub fn new_setup_asset_example(
    mut state: ResMut<DefaultTraitState>,
    asset_server: Res<AssetServer>,
) {
    state.handle = asset_server.load("text/descriptions/races/alternate_traits");
}

pub fn new_print_on_load(
    mut state: ResMut<DefaultTraitState>,
    text_assets: ResMut<Assets<AltTraitAsset>>,
) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
