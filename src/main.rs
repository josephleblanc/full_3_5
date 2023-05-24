#![allow(dead_code)]

mod menu;
mod my_camera;
mod my_debug;
mod my_test_plugins;
mod system_scheduling;
mod systems;
mod technical;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use menu::main_menu;
use systems::{grid_systems::flex_grid, layout::plugin::CharacterCreationPlugin};
use technical::alternate_traits::MyAltTraitAssetPlugin;
use technical::archetype::MyArchetypeAssetPlugin;
use technical::class::MyClassAssetPlugin;
use technical::default_race_traits::MyDefaultTraitAssetPlugin;
use technical::favored_class::MyFavoredClassAssetPlugin;
use technical::race_load::MyRaceAssetPlugin;
// #[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use system_scheduling::states::AppState;

pub const GRID_SQUARE_SIZE: usize = 50;
pub const GRID_LINE_THICKNESS: f32 = 5.;
pub const GRID_LINE_COLOR: Color = Color::BLUE;
pub const GRID_Z_POS: f32 = 0.;

fn main() {
    ////
    let mut app = App::new();
    app
        // Bevy default plugins, includes window setup
        .add_plugins(DefaultPlugins.set(
            // Using ImagePlugin::default_nearest() here because it is supposed
            // to help pixel art render without being blurred by linear filtering.
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1800., 1200.).into(),
                    ..default()
                }),
                ..default()
            },
        ))
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MyRaceAssetPlugin)
        .add_plugin(MyDefaultTraitAssetPlugin)
        .add_plugin(MyAltTraitAssetPlugin)
        .add_plugin(MyFavoredClassAssetPlugin)
        .add_plugin(MyClassAssetPlugin)
        .add_plugin(MyArchetypeAssetPlugin)
        .add_system(load_ascii.in_base_set(StartupSet::PreStartup))
        .add_state::<AppState>()
        .add_plugin(CharacterCreationPlugin)
        .add_system(my_camera::my_camera_systems::setup.on_startup())
        // .add_system(my_camera::my_camera_systems::setup.in_schedule(OnEnter(AppState::Battle)))
        .add_system(main_menu::setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(main_menu::button_system.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(main_menu::main_menu_cleanup.in_schedule(OnExit(AppState::MainMenu)))
        .add_system(flex_grid::setup_flex_grid.in_schedule(OnEnter(AppState::Battle)));
    // .add_system(systems::interface::mouse::mouse_scroll);
    // .add_startup_system(new_setup_asset_example)
    // .add_system(new_print_on_load);
    //
    // For testing character.rs
    // use crate::systems::game::race::*;
    // use crate::systems::game::character::PlayableRace;
    // use RacialTraitName::*;
    // app.insert_resource(RaceBuilder(
    // RacialTraitName::default_traits(&PlayableRace::Gnome),
    // RacialTraitName::default_traits(&PlayableRace::Elf),
    // vec![
    // Humanoid,
    // Human,
    // SizeMedium,
    // NormalVision,
    // SpeedNormal,
    // ChooseOneASM,
    // BaseLanguagesCommonAny,
    // BaseHumanBonusFeat,
    // BaseHumanSkilled,
    // ElvenImmunities,
    // BaseElfElvenMagic,
    // KeenSenses,
    // BaseElfWeaponFamiliarity,
    // ]
    // ))
    //.add_system(spawn_player.on_startup())
    //.add_systems(
    //    (
    //        print_armor_class_bonuses.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_floating_bonus_feats
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        build_race.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_ability_score_bonuses
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_saving_throw_bonuses
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_caster_level_bonuses
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_floating_ability_bonuses
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_skill_bonuses.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_builder.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_weapon_proficiencies
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_spell_like_abilities
    //            .run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_spell_dc_bonuses.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //        print_attack_roll_bonuses.run_if(input_just_pressed::<MouseButton>(MouseButton::Right)),
    //    )
    //        .chain(),
    //);

    app.run();
}
fn check_state(state: Res<State<AppState>>) {
    println!("State: {:?}", state.0);
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(50.));

    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 900.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Player"));
}

#[derive(Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("df_tileset_Markvii.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(12.0), 16, 16, None, None);

    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

#[derive(Resource)]
pub struct EntitiesTimer(Timer);

#[derive(Component)]
pub enum TestParentSet {
    Alpha,
    Beta,
    Gamma,
}

pub fn test_multiple_commands(mut commands: Commands, query: Query<(Entity, &TestParentSet)>) {
    if let Ok((entity, _)) = query.get_single() {
        let parent_id = commands.spawn(TestParentSet::Gamma).id();
        commands.spawn(TestParentSet::Alpha).set_parent(entity);
        commands.spawn(TestParentSet::Beta).set_parent(entity);

        commands.spawn(TestParentSet::Alpha).set_parent(parent_id);
    }
}

pub fn test_set_parent(mut commands: Commands) {
    commands.spawn(TestParentSet::Gamma);
}
