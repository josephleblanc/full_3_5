#![allow(unused_mut, unused_variables)]
use crate::constants::PATH_SIMPLE_FONT;
use crate::menu::character_creation::components::ListNode;
use crate::menu::{
    character_creation::{
        components::{SubTab, SubTabListParent, Tab, TabListParent, TooltipText},
        constants::PROGRESSION_TABLE_HEADERS,
        layout::resource::CentralListBundles,
        systems::tooltip,
    },
    components::{CellPosition, MyTable},
    styles::TEXT_COLOR,
};
use crate::systems::game::character::*;
use crate::systems::game::{class::*, skills::SkillName::*};
use crate::technical::class::ClassAsset;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::sync::Arc;
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

#[derive(Resource, Copy, Clone, Default, PartialEq)]
pub struct ClassTablesBuilt(pub bool);
impl ClassTablesBuilt {
    pub fn set_true(&mut self) {
        self.0 = true;
    }
}

#[derive(Resource, Copy, Clone, Default, PartialEq)]
pub struct ClassTablesSpawned(pub bool);
impl ClassTablesSpawned {
    pub fn set_true(&mut self) {
        self.0 = true;
    }
}

#[derive(Resource, Default)]
pub struct ClassTablesMap(pub HashMap<PlayableClass, ClassTable>);
impl ClassTablesMap {
    pub fn inner_ref(&self) -> &HashMap<PlayableClass, ClassTable> {
        &self.0
    }
    fn inner_ref_mut(&mut self) -> &mut HashMap<PlayableClass, ClassTable> {
        &mut self.0
    }
}

#[derive(Clone, Default)]
/// Holds everything needed to make a progression table for a class. This can then be modified with
/// an archetype to produce a changed table that reflects the progression of the class with that
/// archetype or archetypes.
///
/// All nodes are already constructed except those in the class features column. The nodes in the
/// class features column need to be constructed before the full table can be built.
pub struct ClassTable {
    pub subtab_list_parent: TableParent,
    pub row_nodes: Vec<ProgressionRowNode>,
    pub headers: Vec<ProgressionHeader>,
    pub text_cells: Vec<TextCell>,
    pub features_cells: Vec<FeaturesCell>,
    pub parent: Option<Entity>,
}

#[derive(Bundle, Clone, Default)]
pub struct TableParent {
    node_bundle: NodeBundle,
    tab: Tab,
    subtab: SubTab,
}
impl TableParent {
    pub fn set_tab(&mut self, other: Tab) {
        self.tab = other;
    }
    pub fn set_subtab(&mut self, other: SubTab) {
        self.subtab = other;
    }
}

#[derive(Bundle, Clone)]
pub struct ProgressionRowNode {
    pub node_bundle: NodeBundle,
    pub class: PlayableClass,
    pub row: MyTable,
}

#[derive(Component, Clone)]
pub struct ProgressionHeader {
    pub text_bundle: Arc<dyn Fn() -> TextBundle + Send + Sync>,
    pub column: MyTable,
    pub cell: CellPosition,
}

#[derive(Component, Clone)]
pub struct TextCell {
    pub text_bundle: Arc<dyn Fn() -> TextBundle + Send + Sync>,
    pub column: MyTable,
    pub cell: CellPosition,
}

#[derive(Clone)]
pub struct FeaturesCell {
    pub cell_node: NodeBundle,
    pub feature_items: Vec<FeatureItem>,
    pub column: MyTable,
    pub cell: CellPosition,
}

#[derive(Clone, Debug)]
pub struct FeatureItem {
    pub title_string: String,
    pub text_style: TextStyle,
    pub class_feature: ClassFeature,
    pub interaction: Interaction,
    pub tooltip: TooltipText,
}

pub fn progression_table_resource(
    query_parent: Query<(Entity, &TabListParent)>,
    custom_asset: Res<Assets<ClassAsset>>,
    asset_server: Res<AssetServer>,
    list_resource: Res<CentralListBundles>,
    class_map: Res<ClassMap>,
    mut class_tables: ResMut<ClassTablesMap>,
    mut class_tables_built: ResMut<ClassTablesBuilt>,
    // try to remove this later
) {
    class_tables_built.set_true();
    let classes = PlayableClass::array();

    let class_map = class_map.into_inner();

    for (class, features) in custom_asset
        .iter()
        .filter(|(_handle, asset)| classes.contains(&asset.class_name))
        .map(|(_handle, asset)| (asset.class_name, &asset.class_features))
    {
        if let Some(class_info) = class_map.inner_ref().get(&class) {
            // Exit early if already built
            if class_tables.inner_ref().contains_key(&class) {
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

            let (parent_entity, _list_parent) = query_parent
                .iter()
                .filter(|(_, &list_parent)| list_parent == (Tab::Class).into())
                .next()
                .unwrap();

            let table_subtab_list_parent = TableParent {
                node_bundle: list_resource.subtab_list_parent.clone(),
                tab: Tab::Class,
                subtab: SubTab::Progression,
            };
            let mut row_nodes: Vec<ProgressionRowNode> = Vec::new();
            let mut headers: Vec<ProgressionHeader> = Vec::new();
            let mut text_cells: Vec<TextCell> = Vec::new();
            let mut features_cells: Vec<FeaturesCell> = Vec::new();
            let parent = parent_entity;

            for (row_i, row) in table_2d.iter().enumerate() {
                let level = row_i;
                let fort_save = class_info.saving_throw_at_level(&SavingThrowName::Fort, level);
                let will_save = class_info.saving_throw_at_level(&SavingThrowName::Will, level);
                let reflex_save = class_info.saving_throw_at_level(&SavingThrowName::Reflex, level);
                row_nodes.push(ProgressionRowNode {
                    node_bundle: NodeBundle {
                        style: Style {
                            padding: UiRect::left(Val::Px(10.)),
                            ..default()
                        },
                        background_color: Color::rgba(0.2, 0.2, 0.2, 0.5).into(),
                        ..default()
                    },
                    class,
                    row: MyTable::Row(row_i),
                });
                for (col_i, _col) in row.iter().enumerate() {
                    let mut feature_items: Vec<FeatureItem> = Vec::new();
                    // Special section with class features
                    // TODO: add a +1, +2, etc. to class features like Bravery
                    if col_i == 5 && row_i > 0 {
                        class_table_common(
                            class_info,
                            row_i,
                            features,
                            &mut feature_items,
                            &text_style,
                        );
                        features_cells.push(FeaturesCell {
                            cell_node: list_resource.list_row_node.clone(),
                            feature_items,
                            column: MyTable::Col(col_i),
                            cell: CellPosition {
                                row: row_i,
                                col: col_i,
                            },
                        });
                    } else {
                        // All other sections, e.g. BAB, fort save, level
                        let regular_text_bundle = {
                            class_table_contents(
                                row_i,
                                col_i,
                                fort_save,
                                will_save,
                                reflex_save,
                                header_style.clone(),
                                text_style.clone(),
                                cols_widths[col_i],
                                class_info.bab_progression,
                            )
                        };
                        if row_i == 0 {
                            headers.push(ProgressionHeader {
                                text_bundle: regular_text_bundle,
                                column: MyTable::Col(col_i),
                                cell: CellPosition {
                                    row: row_i,
                                    col: col_i,
                                },
                            });
                        } else {
                            text_cells.push(TextCell {
                                text_bundle: regular_text_bundle,
                                column: MyTable::Col(col_i),
                                cell: CellPosition {
                                    row: row_i,
                                    col: col_i,
                                },
                            });
                        }
                    }
                }
            }
            class_tables.inner_ref_mut().insert(
                class,
                ClassTable {
                    subtab_list_parent: table_subtab_list_parent,
                    row_nodes,
                    headers,
                    text_cells,
                    features_cells,
                    parent: Some(parent_entity),
                },
            );
        }
    }
}

/// Spawns a table from `ClassMap` and `ClassTablesMap` resources.
/// The table must already have been built and inserted into the `ClassTablesMap` resource for this
/// function to work correctly.
pub fn spawn_tables(
    class_tables: Res<ClassTablesMap>,
    class_map: Res<ClassMap>,
    list_resource: Res<CentralListBundles>,
    mut class_tables_spawned: ResMut<ClassTablesSpawned>,
    mut commands: Commands,
) {
    class_tables_spawned.set_true();
    let list_parent = class_tables
        .inner_ref()
        .values()
        .next()
        .unwrap()
        .parent
        .unwrap();

    let tab = Tab::Class;
    let subtab = SubTab::Progression;

    // The subtab_container is used to manage whether to display/hide itself based on the events
    // sent by changing tab select_tab::new_display_subtab_list
    let subtab_container = commands
        .spawn((
            list_resource.subtab_list_parent.clone(),
            SubTabListParent { tab, subtab },
            Name::from(format!("{tab} {subtab} subtab container")),
        ))
        .set_parent(list_parent)
        .id();
    for class in PlayableClass::array() {
        // One table is made for every class with an entry in both ClassMap and ClassTablesMap
        let table_container = commands
            .spawn((
                list_resource.list_node.clone(),
                // ListNode is required for left_panel::display_class to manage the display of the
                // items.
                ListNode,
                class,
                Name::from(format!("{class} table container - listnode")),
            ))
            .set_parent(subtab_container)
            .id();
        if let Some(class_info) = class_map.inner_ref().get(&class) {
            let mut row_ids: HashMap<MyTable, Entity> = HashMap::new();
            if let Some(class_table) = class_tables.inner_ref().get(&class) {
                for (row_i, row_node) in class_table.row_nodes.iter().enumerate() {
                    let row_node_id = commands
                        .spawn(row_node.clone())
                        .set_parent(table_container)
                        .id();
                    row_ids.insert(row_node.row, row_node_id);
                    let level = row_i;
                }
                for header in &class_table.headers {
                    if let Some(row_entity) = row_ids.get(&MyTable::Row(header.cell.row)) {
                        commands
                            .spawn((header.text_bundle)())
                            .set_parent(*row_entity);
                    }
                }
                for text_cell in &class_table.text_cells {
                    if let Some(row_entity) = row_ids.get(&MyTable::Row(text_cell.cell.row)) {
                        commands
                            .spawn((text_cell.text_bundle)())
                            .set_parent(*row_entity);
                    }
                }
                for features_cell in &class_table.features_cells {
                    if let Some(row_entity) = row_ids.get(&MyTable::Row(features_cell.cell.row)) {
                        commands
                            .spawn(features_cell.clone().cell_node)
                            .set_parent(*row_entity)
                            .with_children(|feature_cell| {
                                for feature_item in features_cell.clone().feature_items {
                                    feature_cell.spawn((
                                        TextBundle {
                                            text: Text::from_section(
                                                feature_item.title_string,
                                                feature_item.text_style,
                                            ),
                                            ..default()
                                        },
                                        feature_item.class_feature,
                                        feature_item.interaction,
                                        feature_item.tooltip,
                                    ));
                                }
                            });
                    }
                }
            }
        }
    }
}

fn class_table_contents(
    row_i: usize,
    col_i: usize,
    fort_save: usize,
    will_save: usize,
    reflex_save: usize,
    header_style_clone: TextStyle,
    text_style_clone: TextStyle,
    cols_width: f32,
    bab: BABProgression,
) -> Arc<dyn Fn() -> TextBundle + Send + Sync> {
    let level = row_i;
    let table_headers = PROGRESSION_TABLE_HEADERS;
    Arc::new(move || TextBundle {
        text: Text::from_section(
            if row_i == 0 {
                String::from(table_headers[col_i].clone())
            } else if col_i == 0 {
                level.to_string()
            } else if col_i == 1 {
                BaseAttack::from_progression(&bab, level).to_string()
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
                    header_style_clone.clone()
                } else {
                    text_style_clone.clone()
                }
            },
        ),
        style: Style {
            padding: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            width: Val::Px(cols_width),
            ..default()
        },
        background_color: Color::rgba(0.2, 0.2, 0.2, 0.9).into(),
        ..default()
    })
}

use crate::technical::class::ClassFeatureDescription;
fn class_table_common(
    class_info: &ClassInfo,
    row_i: usize,
    features: &Vec<ClassFeatureDescription>,
    feature_items: &mut Vec<FeatureItem>,
    text_style: &TextStyle,
) {
    for (i, class_feature) in class_info.class_features[row_i - 1].iter().enumerate() {
        let feature = &features
            .iter()
            .filter(|feature_desc| feature_desc.class_feature_name == class_feature.as_default())
            .next()
            .expect("class feature in class_info not found in class_asset");
        feature_items.push(FeatureItem {
            title_string: {
                if class_info.class_features[row_i - 1].len() > i + 1 {
                    let feature_title = feature.title.clone();
                    let mut title_string = feature_title.to_string();
                    title_string.push_str(", ");
                    title_string
                } else {
                    feature.title.clone()
                }
            },
            text_style: text_style.clone(),
            class_feature: *class_feature,
            interaction: Interaction::default(),
            tooltip: TooltipText(Text::from_sections([
                TextSection::new(&feature.title, text_style.clone()),
                TextSection::new("\n", text_style.clone()),
                TextSection::new(
                    tooltip::first_99_words(feature.description.clone()),
                    text_style.clone(),
                ),
            ])),
        });
    }
}
