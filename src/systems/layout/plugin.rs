use crate::systems::interface::mouse::mouse_left_clicked;
use crate::{
    my_debug::print::*,
    system_scheduling::states::AppState,
    systems::{
        game::race::{build_race, RaceBuilder},
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
    LeftClicked,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Changed {
    Race,
    RaceTab,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SuperSet {
    Super,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum Build {
    Super,
    PreBuild,
    Build,
    PostBuild,
}

use bevy::input::common_conditions::input_just_pressed;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app
            //// init resources, load custom assets, & build layout
            .init_resource::<SelectedRaceButton>()
            .init_resource::<SelectedRacialDescriptionType>()
            .init_resource::<FlavorTextSetup>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .init_resource::<RaceBuilder>()
            .add_systems(
                (setup_assets, build_layout, apply_system_buffers)
                    .chain()
                    .in_schedule(OnEnter(AppState::CharacterCreation)),
            )
            // .add_system(setup_assets.in_schedule(OnEnter(AppState::CharacterCreation)))
            // .add_system(build_layout.in_schedule(OnEnter(AppState::CharacterCreation)))
            // .add_system(
            //     setup_flavor_text
            //         .after(build_layout)
            //         .run_if(is_custom_asset_loaded::<RaceAsset>())
            //         .in_schedule(OnEnter(AppState::CharacterCreation)),
            // )
            .configure_set(
                // Ensure custom assets loaded, only run in character creation
                SuperSet::Super
                    .run_if(is_custom_asset_loaded::<RaceAsset>())
                    .run_if(is_custom_asset_loaded::<DefaultTraitAsset>())
                    .in_set(OnUpdate(AppState::CharacterCreation)),
            )
            .configure_sets(
                (
                    Build::Super.run_if(resource_changed::<SelectedRaceButton>()),
                    Build::PreBuild
                        .run_if(resource_changed::<SelectedRaceButton>())
                        .before(Build::Build),
                    Build::Build.run_if(resource_changed::<RaceBuilder>()),
                    Changed::Race.run_if(resource_changed::<SelectedRaceButton>()),
                    Changed::RaceTab.run_if(resource_changed::<SelectedRacialDescriptionType>()),
                )
                    .in_set(SuperSet::Super),
            )
            .configure_set(
                ButtonSet::LeftClicked
                    .in_set(SuperSet::Super)
                    .run_if(input_just_pressed(bevy::input::mouse::MouseButton::Left)),
            )
            // Add default flavor text
            .add_system(setup_flavor_text.in_set(SuperSet::Super))
            // Mouse Scroll systems
            .add_system(mouse_scroll.in_set(SuperSet::Super))
            // Race select button management
            .add_systems(
                (
                    race_select_button_system,
                    cleanup_race_select_button,
                    selected_race_description_type,
                    cleanup_race_description_type_button,
                )
                    .in_set(SuperSet::Super),
            )
            // Manages displayed racial descriptions in the central area
            .add_systems(
                (
                    selected_race_visibility,
                    scroll_snap_top,
                    selected_default_traits_visibility,
                )
                    .in_set(Changed::Race),
            )
            // Changes central area based on which tab is selected
            .add_system(display_racial_tab.in_set(Changed::RaceTab))
            .add_system(update_race_builder.in_set(Build::PreBuild))
            .add_systems(
                (
                    reset_race,
                    apply_system_buffers,
                    build_race,
                    apply_system_buffers,
                    // only for testing, remove later
                    // --------------------------
                    print_builder,
                    print_floating_ability_bonuses,
                    print_floating_bonus_feats,
                    print_floating_skill_bonuses,
                    print_saving_throw_bonuses,
                    print_caster_level_bonuses,
                    print_armor_class_bonuses,
                    print_spell_like_abilities,
                    print_spell_dc_bonuses,
                    print_attack_roll_bonuses,
                    // --------------------------
                    update_common_traits_display,
                )
                    .chain()
                    .in_set(Build::Build),
            )
            .add_system(standard_traits_visibility.in_set(Changed::Race))
            .add_system(track_trait.in_set(SuperSet::Super));
        // .add_systems(
        //     (
        //     )
        //         .chain()
        //         .in_set(ButtonSet::LeftClicked),
        // );
    }
}
