use crate::{
    menu::{
        character_creation::{
            components::{SelectedSubTab, *},
            layout::{
                generics::{
                    build_subtab_buttons::{self, BuiltSubTabButtons, CharacterCreationSubTabs},
                    build_tab_buttons::{self, BuiltTabButtons, CharacterTabs},
                    description, recur_description,
                    select_item::{build_button_desc_list, BuiltLists},
                },
                resource::CentralListBundles,
            },
            systems::{setup::*, *},
        },
        mouse::mouse_scroll,
    },
    system_scheduling::states::AppState,
    systems::{
        game::{
            character::PlayableRace,
            class::PlayableClass,
            race::{RaceBuilder, RacialTraitName},
        },
        layout::character_creation::build_layout,
    },
    technical::{
        alternate_traits::AltTraitAsset,
        archetype::ArchetypeAsset,
        class::ClassAsset,
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum EventSet {
    Sending,
    Receiving,
}

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Events
            .add_event::<SelectTabEvent>()
            .add_event::<SelectSubTabEvent>()
            //// init resources, load custom assets, & build layout
            .init_resource::<SelectedRace>()
            .init_resource::<SelectedClass>()
            .init_resource::<SelectedArchetype>()
            .init_resource::<SelectedTab>()
            .init_resource::<SelectedSubTab>()
            .init_resource::<SelectedSubTabsMap>()
            .init_resource::<FlavorTextSetup>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<DefaultTraitAsset>>()
            .init_resource::<CustomAssetLoadState<AltTraitAsset>>()
            .init_resource::<CustomAssetLoadState<FavoredClassAsset>>()
            .init_resource::<CustomAssetLoadState<ArchetypeAsset>>()
            .init_resource::<RaceBuilder>()
            .init_resource::<BuiltLists>()
            .init_resource::<BuiltTabButtons>()
            .init_resource::<BuiltSubTabButtons>()
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
                    build_tab_buttons::build_tab_buttons::<CharacterTabs, Tab>(),
                    build_subtab_buttons::build_subtab_buttons::<
                        CharacterCreationSubTabs,
                        SubTabButton,
                    >(),
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
                    EventSet::Sending,
                    EventSet::Receiving.after(EventSet::Sending),
                )
                    .in_set(SuperSet::Super),
            )
            // .configure_sets(
            //     (
            //         Build::Super.run_if(resource_changed::<SelectedRace>()),
            //         Build::PreBuild
            //             .before(Build::Build)
            //             .run_if(resource_changed::<SelectedRace>()),
            //         Build::Build.run_if(resource_changed::<RaceBuilder>()),
            //         Changed::Race.run_if(resource_changed::<SelectedRace>()),
            //     )
            //         .in_set(SuperSet::Super),
            // )
            // Mouse Scroll systems
            .add_system(mouse_scroll.in_set(SuperSet::Super))
            // Tab select button management (Race, Class, etc.)
            .add_systems(
                (
                    // Race Tab
                    description::build_description_list::<RaceAsset, PlayableRace>(
                        Tab::Race,
                        SubTab::Description,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Race,
                        subtab: SubTab::Description,
                    }))),
                    recur_description::build_item_desc_list::<
                        DefaultTraitAsset,
                        PlayableRace,
                        RacialTraitName,
                    >(Tab::Race, SubTab::DefaultTraits)
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Race,
                        subtab: SubTab::DefaultTraits,
                    }))),
                    build_button_desc_list::<AltTraitAsset, PlayableRace, RacialTraitName>(
                        Tab::Race,
                        SubTab::AltTraits,
                        true,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Race,
                        subtab: SubTab::AltTraits,
                    }))),
                    // Class Tab
                    description::build_description_list::<ClassAsset, PlayableClass>(
                        Tab::Class,
                        SubTab::Description,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Class,
                        subtab: SubTab::Description,
                    }))),
                    // generics::new_selected_tab::<SelectedTab, Tab>(),
                    // generics::cleanup_tab_button::<SelectedTab, Tab>(),
                    // generics::new_selected_tab::<SelectedRaceTab, RaceTab>(),
                    // generics::cleanup_tab_button::<SelectedRaceTab, RaceTab>(),
                    // generics::new_selected_tab::<SelectedClassTab, ClassTab>(),
                    // generics::cleanup_tab_button::<SelectedClassTab, ClassTab>(),
                )
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                (
                    select_tab::tab_button_select,
                    select_tab::subtab_button_select,
                )
                    .in_set(EventSet::Sending),
            )
            .add_systems(
                (
                    select_tab::new_display_tab_list,
                    select_tab::debug_new_display_tab_list,
                    select_tab::new_display_subtab_list,
                    select_tab::debug_new_display_subtab_list,
                    select_tab::tab_button_color,
                    select_tab::subtab_button_color,
                    select_tab::display_subtab_buttons,
                )
                    .in_set(EventSet::Receiving),
            );
        // Manages displayed racial descriptions in the central area
        // .add_systems((scroll_snap_top,).in_set(Changed::Race))
        // .add_system(update_race_builder.in_set(Build::PreBuild))
        // .add_systems(
        //     (
        //         reset_race,
        //         apply_system_buffers,
        //         build_race,
        //         apply_system_buffers,
        //         update_common_traits_display,
        //     )
        //         .chain()
        //         .in_set(Build::Build),
        // )
        // .add_system(chosen_trait_tooltip.in_set(SuperSet::Super))
        // .add_systems(
        //     (
        //         // Manage the display of left panels.
        //         left_panel::race_panel.run_if(resource_changed::<SelectedTab>()),
        //         left_panel::class_panel.run_if(resource_changed::<SelectedTab>()),
        //         left_panel::archetype_panel.run_if(
        //             resource_changed::<SelectedTab>()
        //                 .or_else(resource_changed::<SelectedClassTab>()),
        //         ),
        //     )
        //         .in_set(SuperSet::Super),
        // )
        // .add_systems(
        //     (left_panel::cleanup_buttons, left_panel::button_system).in_set(SuperSet::Super),
        // );
    }
}
