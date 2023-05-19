//! Implements loader for a custom asset type.

use crate::systems::game::character::PlayableRace;
use crate::systems::game::class::{FavoredClass, PlayableClass};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, Default)]
#[uuid = "455bd79e-ee91-40f8-b28a-d1ad3377e26e"]
pub struct FavoredClassAsset {
    pub race: PlayableRace,
    pub favored_classes: Vec<FavoredClass>,
}

#[derive(Default)]
pub struct FavoredClassAssetLoader;

impl AssetLoader for FavoredClassAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<FavoredClassAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["favored_class.ron"]
    }
}

pub struct MyFavoredClassAssetPlugin;

impl Plugin for MyFavoredClassAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_asset::<FavoredClassAsset>()
            .init_asset_loader::<FavoredClassAssetLoader>()
            .add_startup_system(setup_asset_example)
            .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct State {
    handle: Handle<FavoredClassAsset>,
    printed: bool,
}

pub fn setup_asset_example(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("text/descriptions/races/human.race.ron");
}

pub fn print_on_load(mut state: ResMut<State>, text_assets: ResMut<Assets<FavoredClassAsset>>) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
