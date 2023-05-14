use crate::systems::interface::mouse::mouse_left_clicked;
use crate::{
    system_scheduling::states::AppState,
    systems::{
        layout::character_creation::build_layout,
        menu::{
            character_creation::*,
            mouse::{mouse_scroll, scroll_snap_top},
        },
    },
    technical::{
        default_race_traits::DefaultTraitAsset,
        is_custom_asset_loaded::{is_custom_asset_loaded, CustomAssetLoadState},
        race_load::RaceAsset,
    },
};
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

pub struct CharacterCreationPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ButtonSet {
    Super,
    Clicked,
    AnyInteraction,
    RacialTab,
}

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app
            //// init resources, load custom assets, & build layout
            .init_resource::<SelectedRaceButton>()
            .init_resource::<SelectedRacialDescriptionType>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .add_system(setup_assets.in_schedule(OnEnter(AppState::CharacterCreation)))
            .add_system(build_layout.in_schedule(OnEnter(AppState::CharacterCreation)))
            //// Configure Sets
            .configure_sets((
                // Super set contains other sets, requires assets to be loaded so the other
                // functions don't break.
                ButtonSet::Super.run_if(
                    in_state(AppState::CharacterCreation).and_then(
                        is_custom_asset_loaded::<RaceAsset>()
                            .and_then(is_custom_asset_loaded::<DefaultTraitAsset>()),
                    ),
                ),
                // LeftClicked for systems that only need to run when left mouse button
                // is clicked.
                ButtonSet::Clicked
                    .in_set(ButtonSet::Super)
                    .run_if(on_event::<MouseButtonInput>().and_then(mouse_left_clicked)),
                // RacialTab set for systems that manage what content is displayed when
                // navigation buttons are left-clicked.
                // e.g. changing displayed text when clicking a different race,
                //      same when clicking the 'racial traits' tab
                ButtonSet::RacialTab
                    .run_if(
                        resource_changed::<SelectedRaceButton>()
                            .or_else(resource_changed::<SelectedRacialDescriptionType>()),
                    )
                    .in_set(ButtonSet::Clicked),
            ))
            .configure_set(ButtonSet::AnyInteraction.in_set(ButtonSet::Super))
            //// add systems
            .add_systems(
                (race_select_button_system, cleanup_race_select_button)
                    .chain()
                    .in_set(ButtonSet::Clicked),
            )
            .add_systems(
                (
                    selected_race_description_type,
                    cleanup_selected_race_description_button,
                )
                    .chain()
                    .in_set(ButtonSet::Clicked),
            )
            .add_system(mouse_scroll.in_set(OnUpdate(AppState::CharacterCreation)))
            .add_systems(
                (
                    selected_default_traits_visibility,
                    selected_race_visibility,
                    display_racial_description_type,
                    // hide all non-selected nodes
                    hide_racial_trait_text,
                    hide_racial_trait_button,
                )
                    .in_set(ButtonSet::RacialTab),
            )
            // snap scrolling content to top on changing selected race or tab
            .add_system(
                scroll_snap_top.run_if(
                    resource_changed::<SelectedRaceButton>()
                        .or_else(resource_changed::<SelectedRacialDescriptionType>()),
                ),
            );
    }
}
