use crate::menu::character_creation::layout::generics::description::{
    self, ClassItemDescription, RaceItemDescription,
};
use crate::menu::character_creation::layout::generics::recur_description::build_item_desc_list;
use crate::menu::character_creation::layout::generics::select_item::{
    build_button_desc_list, BuiltLists, ListName, RaceItemAltTrait, RaceItemDefaultTrait,
};
use crate::menu::character_creation::layout::resource::CentralListBundles;
use crate::systems::game::character::PlayableRace;
use crate::systems::game::class::PlayableClass;
use crate::systems::game::race::RacialTraitName;
use crate::technical::class::ClassAsset;
use crate::{
    menu::character_creation::{
        components::*,
        generics,
        systems::*,
        systems::{race_tab::*, setup::*, tooltip::*},
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
    RaceTabAny,
    Class,
    ClassTab,
    ClassOrTab,
    ClassOrTabAny,
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
            .init_resource::<SelectedRace>()
            .init_resource::<SelectedClassTab>()
            .init_resource::<SelectedClass>()
            .init_resource::<SelectedArchetype>()
            .init_resource::<FlavorTextSetup>()
            .init_resource::<SelectedCreationTab>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .init_resource::<CustomAssetLoadState<AltTraitAsset>>()
            .init_resource::<CustomAssetLoadState<FavoredClassAsset>>()
            .init_resource::<CustomAssetLoadState<ArchetypeAsset>>()
            .init_resource::<RaceBuilder>()
            .init_resource::<BuiltLists>()
            .insert_resource::<TooltipTimer>(TooltipTimer(Timer::from_seconds(
                0.5,
                TimerMode::Once,
            )))
            .add_systems(
                (
                    setup_assets,
                    build_layout,
                    CentralListBundles::init,
                    apply_system_buffers,
                )
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
                        .run_if(resource_equals(SelectedCreationTab(CreationTab::Race))),
                    CreationTabSet::Class
                        .run_if(resource_equals(SelectedCreationTab(CreationTab::Class))),
                )
                    .in_set(SuperSet::Super),
            )
            .configure_sets(
                (
                    Build::Super.run_if(resource_changed::<SelectedRace>()),
                    Build::PreBuild
                        .before(Build::Build)
                        .run_if(resource_changed::<SelectedRace>()),
                    Build::Build.run_if(resource_changed::<RaceBuilder>()),
                    Changed::Race.run_if(resource_changed::<SelectedRace>()),
                    Changed::RaceOrTab.run_if(
                        resource_changed::<SelectedRace>()
                            .or_else(resource_changed::<SelectedRaceTab>()),
                    ),
                    Changed::ClassOrTab.run_if(
                        resource_changed::<SelectedClass>()
                            .or_else(resource_changed::<SelectedClassTab>()),
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
            .add_systems(
                (
                    description::build_description_list::<
                        CreationTab,
                        RaceTab,
                        RaceAsset,
                        RaceItemDescription,
                        PlayableRace,
                    >(
                        CreationTab::Race,
                        RaceTab::RaceDescription,
                        ListName::DescriptionRace,
                    )
                    .run_if(not(BuiltLists::is_built(ListName::DescriptionRace))),
                    build_item_desc_list::<
                        RaceItemDefaultTrait,
                        CreationTab,
                        RaceTab,
                        DefaultTraitAsset,
                        PlayableRace,
                        RacialTraitName,
                    >(
                        CreationTab::Race,
                        RaceTab::StandardTraitTab,
                        ListName::DefaultTraitsRace,
                    )
                    .run_if(not(BuiltLists::is_built(ListName::DefaultTraitsRace))),
                    build_button_desc_list::<
                        RaceItemAltTrait,
                        CreationTab,
                        RaceTab,
                        AltTraitAsset,
                        PlayableRace,
                        RacialTraitName,
                    >(
                        CreationTab::Race,
                        RaceTab::AltTraitTab,
                        ListName::AltTraitsRace,
                        true,
                    )
                    .run_if(not(BuiltLists::is_built(ListName::AltTraitsRace))),
                    // Class Tab
                    description::build_description_list::<
                        CreationTab,
                        ClassTab,
                        ClassAsset,
                        ClassItemDescription,
                        PlayableClass,
                    >(
                        CreationTab::Class,
                        ClassTab::Description,
                        ListName::ClassDescription,
                    )
                    .run_if(not(BuiltLists::is_built(ListName::ClassDescription))),
                    generics::new_selected_tab::<SelectedCreationTab, CreationTab>(),
                    generics::cleanup_tab_button::<SelectedCreationTab, CreationTab>(),
                    generics::new_selected_tab::<SelectedRaceTab, RaceTab>(),
                    generics::cleanup_tab_button::<SelectedRaceTab, RaceTab>(),
                    generics::new_selected_tab::<SelectedClassTab, ClassTab>(),
                    generics::cleanup_tab_button::<SelectedClassTab, ClassTab>(),
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
            .add_systems(
                (
                    // RaceTab
                    description::display_node::<SelectedRace, PlayableRace, RaceItemDescription>
                        .run_if(
                            resource_changed::<SelectedCreationTab>().or_else(
                                resource_changed::<SelectedRaceTab>()
                                    .or_else(resource_changed::<SelectedRace>()),
                            ),
                        )
                        .run_if(resource_equals(SelectedCreationTab(CreationTab::Race))),
                    description::display_node::<SelectedRace, PlayableRace, RaceItemDefaultTrait>
                        .run_if(
                            resource_changed::<SelectedCreationTab>().or_else(
                                resource_changed::<SelectedRaceTab>()
                                    .or_else(resource_changed::<SelectedRace>()),
                            ),
                        )
                        .run_if(resource_equals(SelectedCreationTab(CreationTab::Race))),
                    // ClassTab
                    description::display_node::<SelectedClass, PlayableClass, RaceItemDescription>
                        .run_if(
                            resource_changed::<SelectedCreationTab>().or_else(
                                resource_changed::<SelectedClassTab>()
                                    .or_else(resource_changed::<SelectedClass>()),
                            ),
                        )
                        .run_if(resource_equals(SelectedCreationTab(CreationTab::Class))),
                    // display_subtab::<CreationTab, SelectedCreationTab, RaceTab, SelectedRaceTab>
                    //     .run_if(
                    //         resource_changed::<SelectedCreationTab>().or_else(
                    //             resource_changed::<SelectedRaceTab>()
                    //                 .or_else(resource_changed::<SelectedRace>()),
                    //         ),
                    //     )
                    //     .run_if(resource_equals(SelectedCreationTab(CreationTab::Race))),
                )
                    .in_set(SuperSet::Super),
            )
            // .add_system(fill_alt_traits.in_set(Changed::Race));
            // .add_systems(
            //     (
            //         // race_tab::list_node,
            //         // race_tab::set_list_title,
            //         // race_tab::button_col,
            //         // race_tab::replacement_text,
            //         // race_tab::replace_node,
            //         // race_tab::replace_text,
            //         // race_tab::description,
            //     )
            //         .chain()
            //         .in_set(Changed::RaceOrTab),
            // )
            .add_systems(
                (
                    // Manage the display of left panels.
                    left_panel::race_panel.run_if(resource_changed::<SelectedCreationTab>()),
                    left_panel::class_panel.run_if(resource_changed::<SelectedCreationTab>()),
                    left_panel::archetype_panel.run_if(
                        resource_changed::<SelectedCreationTab>()
                            .or_else(resource_changed::<SelectedClassTab>()),
                    ),
                )
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                (
                    subtab_button::display.run_if(resource_changed::<SelectedCreationTab>()),
                    subtab_button::text.run_if(resource_changed::<SelectedCreationTab>()),
                    // I'm afraid I deleted this in the re-organization. Maybe grab it from a
                    // roll-back
                    // SubTabButtonText::display.run_if(resource_changed::<SelectedCreationTab>()),
                )
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                (left_panel::cleanup_buttons, left_panel::button_system).in_set(SuperSet::Super),
            )
            .add_systems(
                (
                    archetype::archetype_panel_display.run_if(
                        resource_changed::<SelectedCreationTab>()
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
                    .run_if(
                        resource_changed::<SelectedCreationTab>()
                            .or_else(resource_changed::<SelectedClassTab>()),
                    )
                    .in_set(SuperSet::Super),
            );
    }
}
