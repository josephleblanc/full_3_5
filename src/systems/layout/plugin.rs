use crate::{
    menu::{
        character_creation::{
            components::*,
            layout::{
                generics::{
                    build_subtab_buttons::{self, BuiltSubTabButtons, CharacterCreationSubTabs},
                    build_tab_buttons::{self, BuiltTabButtons, CharacterTabs},
                    description, recur_description,
                    select_item::{build_button_desc_list, BuiltLists},
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
            archetype::{
                ArchTableBuilt, ArchTableSpawned, ArchTablesMap, ArchetypeMap, ArchetypeName,
            },
            character::PlayableRace,
            class::{ClassFeature, PlayableClass},
            race::{build_race, RaceBuilder, RacialTraitName},
            resources::{
                archetype_resource,
                class_resource::{self, ClassTablesBuilt, ClassTablesMap, ClassTablesSpawned},
            },
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
            .init_resource::<ClassTablesMap>()
            .init_resource::<ClassTablesBuilt>()
            .init_resource::<ClassTablesSpawned>()
            .init_resource::<ArchTableSpawned>()
            .init_resource::<ArchTablesMap>()
            .init_resource::<ArchetypeMap>()
            .init_resource::<ArchTableBuilt>()
            .insert_resource::<TooltipTimer>(TooltipTimer(Timer::from_seconds(
                0.5,
                TimerMode::Once,
            )))
            .add_systems(
                OnEnter(AppState::CharacterCreation),
                (
                    setup_assets,
                    build_layout,
                    CentralListBundles::init,
                    class_resource::setup_classes,
                    archetype_resource::setup_archetypes,
                    apply_deferred,
                    build_tab_buttons::build_tab_buttons::<CharacterTabs, Tab>(),
                    build_subtab_buttons::build_subtab_buttons::<
                        CharacterCreationSubTabs,
                        SubTabButton,
                    >(),
                )
                    .chain(),
            )
            .configure_set(
                Update,
                // Ensure custom assets loaded, only run in character creation
                SuperSet::Super
                    .run_if(is_custom_asset_loaded::<RaceAsset>())
                    .run_if(is_custom_asset_loaded::<ClassAsset>())
                    .run_if(is_custom_asset_loaded::<DefaultTraitAsset>())
                    .run_if(is_custom_asset_loaded::<AltTraitAsset>())
                    .run_if(is_custom_asset_loaded::<FavoredClassAsset>())
                    .run_if(in_state(AppState::CharacterCreation)),
            )
            .configure_sets(
                Update,
                (
                    EventSet::Sending,
                    EventSet::Receiving.after(EventSet::Sending),
                )
                    .in_set(SuperSet::Super),
            )
            .configure_sets(
                Update,
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
            .add_systems(Update, mouse_scroll.in_set(SuperSet::Super))
            // Tab select button management (Race, Class, etc.)
            .add_systems(
                Update,
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
                    class_resource::progression_table_resource
                        .run_if(resource_equals(ClassTablesBuilt(false))),
                    class_resource::spawn_tables
                        .run_if(resource_equals(ClassTablesSpawned(false)))
                        .run_if(resource_equals(ClassTablesBuilt(true))),
                    // Archetype Tab
                    description::build_description_list::<ArchetypeAsset, ArchetypeName>(
                        Tab::Archetype,
                        SubTab::Description,
                    )
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Archetype,
                        subtab: SubTab::Description,
                    }))),
                    recur_description::build_item_desc_list::<
                        ArchetypeAsset,
                        ArchetypeName,
                        ClassFeature,
                    >(Tab::Archetype, SubTab::Features)
                    .run_if(not(BuiltLists::is_built(SubTabListParent {
                        tab: Tab::Archetype,
                        subtab: SubTab::Features,
                    }))),
                    archetype_resource::modify_class_map
                        .run_if(resource_equals(ClassTablesBuilt(true)))
                        .run_if(resource_equals(ArchTableBuilt(false)))
                        .run_if(on_event::<LeftPanelEvent>()),
                    archetype_resource::spawn_tables
                        .run_if(resource_equals(ClassTablesBuilt(true)))
                        .run_if(resource_equals(ArchTableBuilt(true)))
                        .run_if(resource_equals(ArchTableSpawned(false))),
                )
                    .in_set(SuperSet::Super),
            )
            .add_systems(
                Update,
                (
                    select_tab::tab_button_select,
                    select_tab::subtab_button_select,
                )
                    .in_set(EventSet::Sending),
            )
            .add_systems(
                Update,
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
                Update,
                (
                    left_panel::button_event::<PlayableRace, SelectedRace>,
                    left_panel::button_event::<PlayableClass, SelectedClass>,
                    left_panel::button_event::<ArchetypeName, SelectedArchetype>,
                )
                    .in_set(EventSet::Sending),
            )
            .add_systems(
                Update,
                (
                    left_panel::panel_recv_tab_display,
                    left_panel::select_race,
                    left_panel::select_class,
                    left_panel::select_archetype,
                )
                    .in_set(EventSet::Receiving),
            )
            .add_systems(
                Update,
                (
                    display_central::display_race.run_if(on_event::<LeftPanelEvent>()),
                    display_central::display_class.run_if(on_event::<LeftPanelEvent>()),
                    display_central::display_archetype.run_if(on_event::<LeftPanelEvent>()),
                )
                    .in_set(EventSet::Receiving),
            )
            .add_systems(
                Update,
                (left_panel::button_color, left_panel::cleanup_buttons).in_set(SuperSet::Super),
            )
            .add_systems(Update, update_race_builder.in_set(Build::PreBuild))
            .add_systems(
                Update,
                (
                    reset_race,
                    apply_deferred,
                    build_race,
                    apply_deferred,
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
            .add_systems(Update, tooltip::display_on_hover.in_set(SuperSet::Super));
    }
}
