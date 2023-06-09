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
                    table,
                },
                resource::CentralListBundles,
            },
            systems::{
                race_tab::{reset_race, update_common_traits_display, update_race_builder},
                setup::*,
                *,
            },
        },
        mouse::mouse_scroll,
    },
    system_scheduling::states::AppState,
    systems::{
        game::{
            archetype::MyArchetypeName,
            character::PlayableRace,
            class::{ClassFeature, PlayableClass},
            race::{build_race, RaceBuilder, RacialTraitName},
            resources::class_resource,
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
enum Changed {
    Race,
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
            .add_event::<LeftPanelEvent>()
            //// init resources, load custom assets, & build layout
            .init_resource::<SelectedRace>()
            .init_resource::<SelectedClass>()
            .init_resource::<SelectedArchetype>()
            .init_resource::<SelectedTab>()
            .init_resource::<SelectedSubTabsMap>()
            .init_resource::<FlavorTextSetup>()
            .init_resource::<CustomAssetLoadState<RaceAsset>>()
            .init_resource::<CustomAssetLoadState<ClassAsset>>()
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
                    class_resource::setup_classes,
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
                    .run_if(is_custom_asset_loaded::<ClassAsset>())
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
            .configure_sets(
                (
                    Build::Super.run_if(resource_changed::<SelectedRace>()),
                    Build::PreBuild
                        .before(Build::Build)
                        .run_if(resource_changed::<SelectedRace>()),
                    Build::Build.run_if(resource_changed::<RaceBuilder>()),
                    Changed::Race.run_if(resource_changed::<SelectedRace>()),
                )
                    .in_set(SuperSet::Super),
            )
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
                    recur_description::build_item_desc_list::<
                        ClassAsset,
                        PlayableClass,
                        ClassFeature,
                    >(Tab::Class, SubTab::Features)
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Class,
                        subtab: SubTab::Features,
                    }))),
                    table::build_progression::<ClassAsset, PlayableClass, ClassFeature>(
                        Tab::Class,
                        SubTab::Progression,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Class,
                        subtab: SubTab::Progression,
                    }))),
                    // Archetype Tab
                    description::build_description_list::<ArchetypeAsset, MyArchetypeName>(
                        Tab::Archetype,
                        SubTab::Description,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Archetype,
                        subtab: SubTab::Description,
                    }))),
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
                    select_tab::new_display_subtab_list,
                    select_tab::tab_button_color,
                    select_tab::subtab_button_color,
                    select_tab::display_subtab_buttons,
                )
                    .in_set(EventSet::Receiving),
            )
            // left panel management
            .add_systems(
                (
                    left_panel::button_event::<PlayableRace, SelectedRace>,
                    left_panel::button_event::<PlayableClass, SelectedClass>,
                    left_panel::button_event::<MyArchetypeName, SelectedArchetype>,
                )
                    .in_set(EventSet::Sending),
            )
            .add_systems(
                (
                    left_panel::panel_recv_tab_display,
                    left_panel::select_race,
                    left_panel::select_class,
                    left_panel::select_archetype,
                )
                    .in_set(EventSet::Receiving),
            )
            .add_systems(
                (
                    display_central::display_race.run_if(on_event::<LeftPanelEvent>()),
                    display_central::display_class.run_if(on_event::<LeftPanelEvent>()),
                    display_central::display_archetype.run_if(on_event::<LeftPanelEvent>()),
                )
                    .in_set(EventSet::Receiving),
            )
            .add_systems(
                (left_panel::button_color, left_panel::cleanup_buttons).in_set(SuperSet::Super),
            )
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
            );
    }
}
