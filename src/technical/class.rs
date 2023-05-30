//! Implements loader for a custom asset type.

use crate::systems::game::class::ClassFeature;
use crate::systems::game::class::PlayableClass;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid, Default)]
#[uuid = "26e3c774-fe4d-4ac7-bf5c-ee9f2337ad4d"]
pub struct ClassAsset {
    pub class_name: PlayableClass,
    pub title: String,
    pub description: String,
    pub class_features: Vec<ClassFeatureDescription>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ClassFeatureDescription {
    pub class_feature_name: ClassFeature,
    pub title: String,
    pub description: String,
}

#[derive(Default)]
pub struct ClassAssetLoader;

impl AssetLoader for ClassAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let race_asset = ron::de::from_bytes::<ClassAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(race_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["class.ron"]
    }
}

pub struct MyClassAssetPlugin;

impl Plugin for MyClassAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>()
            .add_asset::<ClassAsset>()
            .init_asset_loader::<ClassAssetLoader>();
        //             .add_startup_system(setup_asset_example)
        //             .add_system(print_on_load);
    }
}

#[derive(Resource, Default)]
pub struct State {
    handle: Handle<ClassAsset>,
    printed: bool,
}

pub fn setup_asset_example(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("text/descriptions/class/fighter.class.ron");
}

pub fn print_on_load(mut state: ResMut<State>, text_assets: ResMut<Assets<ClassAsset>>) {
    let text_asset = text_assets.get(&state.handle);
    if state.printed || text_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", text_asset.unwrap());
    state.printed = true;
}
