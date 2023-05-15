//! Implements loader for a custom asset type.

use crate::systems::game::character::PlayableRace;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, Default)]
#[uuid = "c9b00296-4394-478c-a140-1bcb3d78c517"]
pub struct RaceAsset {
    pub race: PlayableRace,
    pub text: String,
}

#[derive(Default)]
pub struct RaceAssetLoader;

impl AssetLoader for RaceAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<RaceAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["race.ron"]
    }
}

pub struct MyRaceAssetPlugin;

impl Plugin for MyRaceAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_asset::<RaceAsset>()
            .init_asset_loader::<RaceAssetLoader>()
            .add_startup_system(setup_asset_example)
            .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct State {
    handle: Handle<RaceAsset>,
    printed: bool,
}

pub fn setup_asset_example(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("text/descriptions/races/human.race.ron");
}

pub fn print_on_load(mut state: ResMut<State>, text_assets: ResMut<Assets<RaceAsset>>) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
