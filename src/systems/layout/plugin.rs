use crate::{
    menu::character_creation::{
        components::*,
        systems::*,
        systems::{creation_tab::*, race_tab::*, setup::*, tooltip::*},
    },
    menu::mouse::{mouse_scroll, scroll_snap_top},
    system_scheduling::states::AppState,
    systems::{
        game::race::{build_race, RaceBuilder},
        layout::character_creation::build_layout,
    },
    technical::{
        alternate_traits::AltTraitAsset,
        archetype::ArchetypeAsset,
        default_race_traits::DefaultTraitAsset,
        favored_class::FavoredClassAsset,
        is_custom_asset_loaded::{is_custom_asset_loaded, CustomAssetLoadState},
        race_load::RaceAsset,
    },
};
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
    RaceOrTab,
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

#[derive(SystemSet, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum CreationTabSet {
    #[default]
    Race,
    AbilityScores,
    Class,
}

use bevy::input::common_conditions::input_just_pressed;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app
            //// init resources, load custom assets, & build layout
            .init_resource::<SelectedRaceTab>()
            .init_resource::<SelectedRaceButton>()
            .init_resource::<SelectedClassTab>()
            .init_resource::<SelectedClass>()
            .init_resource::<SelectedArchetype>()
            .init_resource::<FlavorTextSetup>()
            .init_resource::<CreationTabSelected>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .init_resource::<CustomAssetLoadState<AltTraitAsset>>()
            .init_resource::<CustomAssetLoadState<FavoredClassAsset>>()
            .init_resource::<CustomAssetLoadState<ArchetypeAsset>>()
            .init_resource::<RaceBuilder>()
            .insert_resource::<TooltipTimer>(TooltipTimer(Timer::from_seconds(
                0.5,
                TimerMode::Once,
            )))
            .add_systems(
                (setup_assets, build_layout, apply_system_buffers)
                    .chain()
                    .in_schedule(OnEnter(AppState::CharacterCreation)),
            )
            .configure_set(
                // Ensure custom assets loaded, only run in character creation
                SuperSet::Super
                    .run_if(is_custom_asset_loaded::<RaceAsset>())
                    .run_if(is_custom_asset_loaded::<DefaultTraitAsset>())
                    .run_if(is_custom_asset_loaded::<AltTraitAsset>())
                    .run_if(is_custom_asset_loaded::<FavoredClassAsset>())
                    .in_set(OnUpdate(AppState::CharacterCreation)),
            )
            .configure_sets(
                (
                    CreationTabSet::Race
                        .run_if(resource_equals(CreationTabSelected(CreationTab::Race))),
                    CreationTabSet::Class
                        .run_if(resource_equals(CreationTabSelected(CreationTab::Class))),
                )
                    .in_set(SuperSet::Super),
            )
            .configure_sets(
                (
                    Build::Super.run_if(resource_changed::<SelectedRaceButton>()),
                    Build::PreBuild
                        .before(Build::Build)
                        .run_if(resource_changed::<SelectedRaceButton>()),
                    Build::Build.run_if(resource_changed::<RaceBuilder>()),
                    Changed::Race.run_if(resource_changed::<SelectedRaceButton>()),
                    Changed::RaceOrTab.run_if(
                        resource_changed::<SelectedRaceButton>()
                            .or_else(resource_changed::<SelectedRaceTab>()),
                    ),
                )
                    .in_set(SuperSet::Super),
            )
            .configure_set(
                Changed::RaceTab
                    .run_if(resource_changed::<SelectedRaceTab>())
                    .in_set(CreationTabSet::Race),
            )
            .configure_set(
                ButtonSet::LeftClicked
                    .in_set(SuperSet::Super)
                    .run_if(input_just_pressed(bevy::input::mouse::MouseButton::Left)),
            )
            // Add default flavor text
            // .add_system(setup_flavor_text.in_set(SuperSet::Super))
            // Mouse Scroll systems
            .add_system(mouse_scroll.in_set(SuperSet::Super))
            // Tab select button management (Race, Class, etc.)
            .add_systems((creation_tab, cleanup_creation_tab).in_set(SuperSet::Super))
            // Race select button management
            .add_systems(
                (
                    selected_race_description_type,
                    cleanup_race_description_type_button,
                )
                    .in_set(SuperSet::Super),
            )
            // Manages displayed racial descriptions in the central area
            .add_systems((scroll_snap_top,).in_set(Changed::Race))
            .add_system(update_race_builder.in_set(Build::PreBuild))
            .add_systems(
                (
                    reset_race,
                    apply_system_buffers,
                    build_race,
                    apply_system_buffers,
                    // only for testing, remove later
                    // --------------------------
                    // print_builder,
                    // print_floating_ability_bonuses,
                    // print_floating_bonus_feats,
                    // print_floating_skill_bonuses,
                    // print_saving_throw_bonuses,
                    // print_caster_level_bonuses,
                    // print_armor_class_bonuses,
                    // print_spell_like_abilities,
                    // print_spell_dc_bonuses,
                    // print_attack_roll_bonuses,
                    // --------------------------
                    update_common_traits_display,
                )
                    .chain()
                    .in_set(Build::Build),
            )
            .add_system(chosen_trait_tooltip.in_set(SuperSet::Super))
            // .add_system(fill_alt_traits.in_set(Changed::Race));
            .add_systems(
                (
                    race_tab::list_node,
                    race_tab::set_list_title,
                    race_tab::button_col,
                    race_tab::replacement_text,
                    race_tab::replace_node,
                    race_tab::replace_text,
                    race_tab::description,
                )
                    .chain()
                    .in_set(Changed::RaceOrTab),
            )
            .add_system(
                left_panel::left_panel
                    .run_if(resource_changed::<CreationTabSelected>())
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                (
                    subtab_button::display.run_if(resource_changed::<CreationTabSelected>()),
                    // I'm afraid I deleted this in the re-organization. Maybe grab it from a
                    // roll-back
                    // SubTabButtonText::display.run_if(resource_changed::<CreationTabSelected>()),
                )
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                (left_panel::cleanup_buttons, left_panel::button_system).in_set(SuperSet::Super),
            )
            .add_systems(
                (
                    class_tab::display_list_node.run_if(
                        resource_changed::<CreationTabSelected>().or_else(
                            resource_changed::<SelectedClassTab>()
                                .or_else(resource_changed::<SelectedClass>()),
                        ),
                    ),
                    class_tab::display_list_title.run_if(
                        resource_changed::<CreationTabSelected>().or_else(
                            resource_changed::<SelectedClassTab>()
                                .or_else(resource_changed::<SelectedClass>()),
                        ),
                    ),
                    class_tab::selected_tab,
                )
                    .in_set(CreationTabSet::Class),
            )
            .add_system(
                left_panel::left_panel
                    .run_if(resource_changed::<CreationTabSelected>())
                    .in_set(CreationTabSet::Class),
            )
            .add_systems(
                (
                    archetype::archetype_panel_display.run_if(
                        resource_changed::<CreationTabSelected>()
                            .or_else(resource_changed::<SelectedClassTab>()),
                    ),
                    archetype::archetype_panel_text
                        .run_if(
                            resource_changed::<SelectedClass>()
                                .and_then(resource_equals(SelectedClassTab(ClassTab::Archetypes))),
                        )
                        .after(archetype::archetype_panel_display),
                    archetype::archetype_panel_title
                        .after(archetype::archetype_panel_display)
                        .run_if(
                            resource_changed::<SelectedClass>()
                                .and_then(resource_equals(SelectedClassTab(ClassTab::Archetypes))),
                        ),
                )
                    .in_set(SuperSet::Super),
            )
            .add_system(
                ListParent::display
                    .run_if(resource_changed::<CreationTabSelected>())
                    .in_set(SuperSet::Super),
            )
            .add_system(
                class_tab::display_list_title
                    .run_if(
                        resource_changed::<SelectedClass>()
                            .or_else(resource_changed::<CreationTabSelected>().and_then(
                                resource_equals(CreationTabSelected(CreationTab::Class)).and_then(
                                    resource_equals(SelectedClassTab(ClassTab::Archetypes)),
                                ),
                            ))
                            .or_else(resource_changed::<SelectedClassTab>()),
                    )
                    .in_set(SuperSet::Super),
            )
            .add_system(
                class_tab::display_list_text
                    .run_if(
                        resource_changed::<SelectedClass>()
                            .or_else(resource_changed::<CreationTabSelected>().and_then(
                                resource_equals(CreationTabSelected(CreationTab::Class)).and_then(
                                    resource_equals(SelectedClassTab(ClassTab::Archetypes)),
                                ),
                            ))
                            .or_else(resource_changed::<SelectedClassTab>()),
                    )
                    .in_set(SuperSet::Super),
            );
    }
}
