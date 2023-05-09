use bevy::prelude::*;

use bevy::asset::Asset;
use bevy::asset::LoadState;

#[derive(Resource, Default)]
pub struct CustomAssetLoadState<T: Asset> {
    pub handles: Vec<Handle<T>>,
    loaded: bool,
}

impl<T: Asset> CustomAssetLoadState<T> {
    pub fn add_untyped(&mut self, handle: &HandleUntyped) {
        self.handles.push(handle.clone().typed())
    }

    pub fn handles(&self) -> &[Handle<T>] {
        &self.handles
    }
}
// Check if a vec of UntypedHandle are loaded,
// good to use as a conditional on systems that need a custom type
// to be loaded.
// Example:
// app.init_resource::<CustomAssetLoadState<SomeCustomAsset>>())
//      .add_system(some_system_using_custom_asset
//          .run_if(is_custom_asset_loaded::<SomeCustomAsset>())
//          );
pub fn is_custom_asset_loaded<T>(
) -> impl FnMut(Res<CustomAssetLoadState<T>>, Res<AssetServer>) -> bool
where
    T: Asset,
{
    move |state: Res<CustomAssetLoadState<T>>, asset_server: Res<AssetServer>| {
        // Use `asset_server` to check load state of each handle
        let handles = state.handles.iter().map(|handle| handle.id());
        let load_state = asset_server.get_group_load_state(handles);
        load_state == LoadState::Loaded
    }
}
