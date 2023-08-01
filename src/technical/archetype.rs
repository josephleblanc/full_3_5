//! Implements loader for a custom asset type.

use crate::systems::game::archetype::ArchetypeName;
use crate::systems::game::class::ClassFeature;
use crate::systems::game::class::PlayableClass;
use bevy::reflect::TypePath;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, Default, TypePath)]
#[uuid = "7eb2e325-a3a2-4a0c-a6b0-e2387ca6bd7d"]
#[type_path = "crate::technical::archetype"]
pub struct ArchetypeAsset {
    pub class_name: PlayableClass,
    pub archetype_name: ArchetypeName,
    pub title: String,
    pub description: String,
    pub class_features: Vec<ArchFeatureDescr>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ArchFeatureDescr {
    pub feature: ClassFeature,
    pub title: String,
    pub description: String,
}

#[derive(Default)]
pub struct ArchetypeAssetLoader;

impl AssetLoader for ArchetypeAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<ArchetypeAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["archetype.ron"]
    }
}

pub struct MyArchetypeAssetPlugin;

impl Plugin for MyArchetypeAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_asset::<ArchetypeAsset>()
            .init_asset_loader::<ArchetypeAssetLoader>();
        // .add_startup_system(setup_asset_example)
        // .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct State {
    handle: Handle<ArchetypeAsset>,
    printed: bool,
}

pub fn setup_asset_example(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle =
        asset_server.load("text/descriptions/class/archetypes/fighter_brawler.archetype.ron");
}

pub fn print_on_load(mut state: ResMut<State>, text_assets: ResMut<Assets<ArchetypeAsset>>) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
