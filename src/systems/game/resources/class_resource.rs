#![allow(unused_mut, unused_variables)]
use crate::constants::PATH_SIMPLE_FONT;
use crate::menu::character_creation::components::{
    SubTab, SubTabListParent, Tab, TabListParent, TooltipText,
};
use crate::menu::character_creation::constants::PROGRESSION_TABLE_HEADERS;
use crate::menu::character_creation::layout::generics::select_item::BuiltLists;
use crate::menu::character_creation::layout::resource::CentralListBundles;
use crate::menu::character_creation::systems::tooltip;
use crate::menu::components::{CellPosition, MyTable};
use crate::menu::styles::TEXT_COLOR;
use crate::systems::game::character::*;
use crate::systems::game::{class::*, skills::SkillName::*};
use crate::technical::class::ClassAsset;
use bevy::prelude::*;
pub fn setup_classes(mut commands: Commands, class_asset: Res<Assets<ClassAsset>>) {
    use BABProgression::*;
    use ClassFeature::*;
    use FighterFeature::*;
    use SavingThrowName::*;
    let mut class_map = ClassMap::new();

    let mut fighter_info = ClassInfo {
        name_str: String::new(),
        description: String::new(),
        class_name: PlayableClass::Fighter,
        class_skills: vec![
            Climb,
            Craft,
            HandleAnimal,
            Intimidate,
            KnowledgeDungeoneering,
            KnowledgeEngineering,
            Profession,
            Ride,
            Survival,
            Swim,
        ],
        skill_ranks_per_level: 2,
        hit_die: Dice::D10,
        alignment_restriction: Option::None,
        bab_progression: Full,
        save_progression: [Fort, SavingThrowName::None, SavingThrowName::None],
        class_features_list: vec![
            Fighter(BonusFeat(Option::None)),
            Fighter(Bravery(Option::None)),
            Fighter(ArmorTraining(Option::None)),
            Fighter(WeaponTraining(Option::None)),
            Fighter(ArmorMastery),
            Fighter(WeaponMastery),
        ],
        class_features: vec![
            vec![Fighter(BonusFeat(Some(1)))],                             // 1
            vec![Fighter(Bravery(Some(1))), Fighter(BonusFeat(Some(2)))],  // 2
            vec![Fighter(ArmorTraining(Some(1)))],                         // 3
            vec![Fighter(BonusFeat(Some(3)))],                             // 4
            vec![Fighter(WeaponTraining(Some(1)))],                        // 5
            vec![Fighter(BonusFeat(Some(4))), Fighter(Bravery(Some(2)))],  // 6
            vec![Fighter(ArmorTraining(Some(2)))],                         // 7
            vec![Fighter(BonusFeat(Some(5)))],                             // 8
            vec![Fighter(WeaponTraining(Some(2)))],                        // 9
            vec![Fighter(BonusFeat(Some(6))), Fighter(Bravery(Some(3)))],  // 10
            vec![Fighter(ArmorTraining(Some(3)))],                         // 11
            vec![Fighter(BonusFeat(Some(7)))],                             // 12
            vec![Fighter(WeaponTraining(Some(3)))],                        // 13
            vec![Fighter(BonusFeat(Some(8))), Fighter(Bravery(Some(4)))],  // 14
            vec![Fighter(ArmorTraining(Some(4)))],                         // 15
            vec![Fighter(BonusFeat(Some(9)))],                             // 16
            vec![Fighter(WeaponTraining(Some(4)))],                        // 17
            vec![Fighter(BonusFeat(Some(10))), Fighter(Bravery(Some(5)))], // 18
            vec![Fighter(ArmorMastery)],                                   // 19
            vec![Fighter(BonusFeat(Some(11))), Fighter(WeaponMastery)],    // 20
        ],
    };
    class_map
        .inner_ref_mut()
        .insert(PlayableClass::Fighter, fighter_info);

    commands.insert_resource(class_map);
}

// TODO: Fix display so the table is only shown for the correct class
pub fn progression_table(
    mut commands: Commands,
    query_parent: Query<(Entity, &TabListParent)>,
    custom_asset: Res<Assets<ClassAsset>>,
    asset_server: Res<AssetServer>,
    list_resource: Res<CentralListBundles>,
    mut res_built: ResMut<BuiltLists>,
    class_map: Res<ClassMap>,
    // try to remove this later
) {
    let tab = Tab::Class;
    let subtab = SubTab::Progression;
    let subtab_list_parent = SubTabListParent { tab, subtab };
    // Exit early if already built
    if res_built.inner_ref().contains(&subtab_list_parent) {
        return ();
    }
    let table_headers = PROGRESSION_TABLE_HEADERS;
    let table_2d = MyTable::build_2d::<6, 21>();
    let cols_widths = [60.0, 180.0, 60.0, 60.0, 60.0, 300.0];

    let shared_font = asset_server.load(PATH_SIMPLE_FONT);
    let text_style = TextStyle {
        font: shared_font.clone(),
        font_size: 18.,
        color: TEXT_COLOR,
    };
    let header_style = TextStyle {
        font: shared_font.clone(),
        font_size: 24.,
        color: TEXT_COLOR,
    };

    let classes = PlayableClass::array();

    let (parent_entity, _list_parent) = query_parent
        .iter()
        .filter(|(_, &list_parent)| list_parent == (Tab::Class).into())
        .next()
        .unwrap();

    let node_container = commands
        .spawn((
            list_resource.subtab_list_parent.clone(),
            Name::from(format!("{tab} {subtab} table node parent")),
            subtab_list_parent,
        ))
        .set_parent(parent_entity)
        .id();

    for (class, features) in custom_asset
        .iter()
        .filter(|(_handle, asset)| classes.contains(&asset.class_name))
        .map(|(_handle, asset)| (asset.class_name, &asset.class_features))
    {
        if let Some(class_info) = class_map.inner_ref().get(&class) {
            for (row_i, row) in table_2d.iter().enumerate() {
                commands
                    .spawn((
                        NodeBundle {
                            style: Style {
                                padding: UiRect::left(Val::Px(10.)),
                                ..default()
                            },
                            background_color: Color::rgba(0.2, 0.2, 0.2, 0.5).into(),
                            ..default()
                        },
                        class,
                        MyTable::Row(row_i),
                    ))
                    .set_parent(node_container)
                    .with_children(|row_node| {
                        let level = row_i;
                        let fort_save =
                            class_info.saving_throw_at_level(&SavingThrowName::Fort, level);
                        let will_save =
                            class_info.saving_throw_at_level(&SavingThrowName::Will, level);
                        let reflex_save =
                            class_info.saving_throw_at_level(&SavingThrowName::Reflex, level);
                        for (col_i, _col) in row.iter().enumerate() {
                            if col_i == 5 && row_i > 0 {
                                row_node
                                    .spawn((
                                        list_resource.list_row_node.clone(),
                                        MyTable::Col(col_i),
                                        CellPosition {
                                            row: row_i,
                                            col: col_i,
                                        },
                                    ))
                                    .with_children(|features_cell| {
                                        for (i, class_feature) in class_info.class_features[row_i - 1].iter().enumerate() {
                                            let feature = &features
                                                .iter()
                                                .filter(|feature_desc| {
                                                    feature_desc.class_feature_name
                                                        == *class_feature
                                                })
                                                .next()
                                                .expect("class feature in class_info not found in class_asset");
                                            features_cell.spawn((
                                                TextBundle {
                                                    text: Text::from_section(
                                                        {
                                                        if class_info.class_features[row_i - 1].len() > i + 1 {
                                                            let feature_title = feature.title.clone();
                                                            let mut title_string = feature_title.to_string();
                                                            title_string.push_str(", ");
                                                            title_string
                                                        } else {feature.title.clone()}},
                                                        text_style.clone(),
                                                    ),
                                                    ..default()
                                                },
                                                *class_feature,
                                                Interaction::default(),
                                                TooltipText( Text::from_sections([
                                                    TextSection::new(
                                                        &feature.title,
                                                        text_style.clone()
                                                        ),
                                                    TextSection::new(
                                                        "\n",
                                                        text_style.clone()
                                                        ),
                                                    TextSection::new(
                                                        tooltip::first_99_words(feature.description.clone()),
                                                        text_style.clone()
                                                        )
                                                    ])
                                                    )
                                            ));
                                        }
                                    });
                            } else {
                                row_node.spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            // Headers for first row
                                            if row_i == 0 {
                                                String::from(table_headers[col_i].clone())
                                            } else if col_i == 0 {
                                                level.to_string()
                                            } else if col_i == 1 {
                                                BaseAttack::from_progression(
                                                    &class_info.bab_progression,
                                                    level,
                                                )
                                                .to_string()
                                            } else if col_i == 2 {
                                                format!("+{}", fort_save)
                                            } else if col_i == 3 {
                                                format!("+{}", will_save)
                                            } else if col_i == 4 {
                                                format!("+{}", reflex_save)
                                            } else {
                                                format!("|{row_i}, {col_i}|")
                                            },
                                            {
                                                if row_i == 0 {
                                                    header_style.clone()
                                                } else {
                                                    text_style.clone()
                                                }
                                            },
                                        ),
                                        style: Style {
                                            padding: UiRect::all(Val::Px(5.)),
                                            margin: UiRect::all(Val::Px(5.)),
                                            size: Size::width(Val::Px(cols_widths[col_i])),
                                            ..default()
                                        },
                                        background_color: Color::rgba(0.2, 0.2, 0.2, 0.9).into(),
                                        ..default()
                                    },
                                    MyTable::Col(col_i),
                                    CellPosition {
                                        row: row_i,
                                        col: col_i,
                                    },
                                ));
                            }
                        }
                    });
            }
        }
    }
    res_built.inner_mut().push(subtab_list_parent);
}
