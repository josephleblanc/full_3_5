//! Implements loader for a custom asset type.
use crate::{systems::game::race::RacialTraitName, technical::race_load::PlayableRace};
use bevy::reflect::TypePath;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

// A sub-struct for DefaultTraitAsset with the info needed to display
// it in the character creation screen.
#[derive(Debug, Deserialize, Default)]
pub struct RacialTraitDisplay {
    pub my_trait_name: RacialTraitName,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, TypeUuid, Default, TypePath)]
#[uuid = "34359287-319c-490c-adfd-7e8cccf8d45a"]
#[type_path = "crate::technical::default_race_traits"]
pub struct DefaultTraitAsset {
    pub race: PlayableRace,
    pub default_traits: Vec<RacialTraitDisplay>,
}

#[derive(Default)]
pub struct DefaultTraitAssetLoader;

impl AssetLoader for DefaultTraitAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<DefaultTraitAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["default_traits.ron"]
    }
}

pub struct MyDefaultTraitAssetPlugin;

impl Plugin for MyDefaultTraitAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefaultTraitState>()
            .add_asset::<DefaultTraitAsset>()
            .init_asset_loader::<DefaultTraitAssetLoader>();
        // .add_startup_system(setup)
        // .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct DefaultTraitState {
    handle: Handle<DefaultTraitAsset>,
    printed: bool,
}

pub fn new_setup_asset_example(
    mut state: ResMut<DefaultTraitState>,
    asset_server: Res<AssetServer>,
) {
    state.handle = asset_server.load("text/descriptions/races/default_traits");
}

pub fn new_print_on_load(
    mut state: ResMut<DefaultTraitState>,
    text_assets: ResMut<Assets<DefaultTraitAsset>>,
) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
