use bevy::{prelude::*, utils::HashMap};

use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::{
                LeftPanelEvent, ListNode, SelectedClass, Status, SubTab, SubTabListParent, Tab,
                TabListParent, TooltipText,
            },
            layout::resource::CentralListBundles,
            systems::tooltip,
        },
        components::{CellPosition, MyTable},
        styles::TEXT_COLOR,
    },
    systems::game::{
        archetype::{
            ArchTableBuilt, ArchTableSpawned, ArchTablesMap, ArchetypeFeature, ArchetypeInfo,
            ArchetypeMap, ArchetypeName,
        },
        class::{BaseAttack, ClassFeature, ClassMap, FighterFeature, PlayableClass},
    },
    technical::{archetype::ArchetypeAsset, class::ClassAsset},
};

use super::class_resource::{ClassTablesMap, FeatureItem, FeaturesCell};

pub fn setup_archetypes(mut commands: Commands) {
    let mut archetype_map = ArchetypeMap::new();

    //// brawler features
    let close_control = ArchetypeFeature {
        features: vec![
            (
                2,
                ClassFeature::Fighter(FighterFeature::CloseControl(Some(1))),
            ),
            (
                6,
                ClassFeature::Fighter(FighterFeature::CloseControl(Some(2))),
            ),
            (
                10,
                ClassFeature::Fighter(FighterFeature::CloseControl(Some(3))),
            ),
            (
                14,
                ClassFeature::Fighter(FighterFeature::CloseControl(Some(4))),
            ),
            (
                18,
                ClassFeature::Fighter(FighterFeature::CloseControl(Some(5))),
            ),
        ],
        replaces: vec![ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(
            1,
        )))],
    };
    let close_combatant = ArchetypeFeature {
        features: vec![
            (
                3,
                ClassFeature::Fighter(FighterFeature::CloseCombatant(Some(1))),
            ),
            (
                7,
                ClassFeature::Fighter(FighterFeature::CloseCombatant(Some(2))),
            ),
            (
                11,
                ClassFeature::Fighter(FighterFeature::CloseCombatant(Some(3))),
            ),
            (
                15,
                ClassFeature::Fighter(FighterFeature::CloseCombatant(Some(4))),
            ),
            (
                19,
                ClassFeature::Fighter(FighterFeature::CloseCombatant(Some(5))),
            ),
        ],
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(1))),
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(2))),
        ],
    };
    let menacing_stance = ArchetypeFeature {
        features: vec![
            (
                7,
                ClassFeature::Fighter(FighterFeature::MenacingStance(Some(1))),
            ),
            (
                11,
                ClassFeature::Fighter(FighterFeature::MenacingStance(Some(2))),
            ),
            (
                15,
                ClassFeature::Fighter(FighterFeature::MenacingStance(Some(3))),
            ),
            (
                19,
                ClassFeature::Fighter(FighterFeature::MenacingStance(Some(4))),
            ),
        ],
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(2))),
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(3))),
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(4))),
            ClassFeature::Fighter(FighterFeature::ArmorMastery),
        ],
    };
    let no_escape = ArchetypeFeature {
        features: vec![(9, ClassFeature::Fighter(FighterFeature::NoEscape))],
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(3))),
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(4))),
        ],
    };
    let stand_still = ArchetypeFeature {
        features: vec![(13, ClassFeature::Fighter(FighterFeature::StandStill))],
        replaces: vec![],
    };
    let weapon_mastery_brawler = ArchetypeFeature {
        features: vec![(
            20,
            ClassFeature::Fighter(FighterFeature::WeaponMasteryBrawler),
        )],
        replaces: vec![ClassFeature::Fighter(FighterFeature::WeaponMastery)],
    };
    let brawler = ArchetypeInfo {
        name: ArchetypeName::Brawler,
        class: PlayableClass::Fighter,
        restrictions: None,
        archetype_features: Some(vec![
            close_control,
            close_combatant,
            menacing_stance,
            no_escape,
            stand_still,
            weapon_mastery_brawler,
        ]),
        skills: None,
        skill_ranks: None,
        gains_proficiency: None,
        loses_proficiency: None,
    };

    archetype_map
        .inner_ref_mut()
        .insert(ArchetypeName::Brawler, brawler);

    commands.insert_resource(archetype_map);
}
#[allow(unused_variables)]
pub fn modify_class_map(
    archetype_map: Res<ArchetypeMap>,
    class_map: Res<ClassMap>,
    class_table_map: Res<ClassTablesMap>,
    selected_class: Res<SelectedClass>,
    mut archetype_table_map: ResMut<ArchTablesMap>,
    arch_asset: Res<Assets<ArchetypeAsset>>,
    class_asset: Res<Assets<ClassAsset>>,
    mut event_reader: EventReader<LeftPanelEvent>,
    asset_server: Res<AssetServer>,
    list_resource: Res<CentralListBundles>,
    mut arch_tables_built: ResMut<ArchTableBuilt>,
    query_parent: Query<(Entity, &TabListParent)>,
) {
    if let Some((class_info, class_table, archetype_info)) = event_reader
        .iter()
        .filter(|event| event.status.is_some() && event.archetype.is_some())
        .filter(|event| event.status.unwrap() == Status::Entering)
        .filter(|event| event.archetype.unwrap().class() == selected_class.inner())
        .map(|event| event.archetype.unwrap())
        .filter(|archetype| {
            class_map.inner_ref().contains_key(&archetype.class())
                && class_table_map.inner_ref().contains_key(&archetype.class())
                && archetype_map.inner_ref().contains_key(&archetype)
        })
        .map(|archetype| {
            (
                class_map.inner_ref().get(&archetype.class()).unwrap(),
                class_table_map.inner_ref().get(&archetype.class()).unwrap(),
                archetype_map.inner_ref().get(&archetype).unwrap(),
            )
        })
        // .filter(|(class_info, archetype_info)| {
        //     archetype_info
        //         .replaces_features()
        //         .iter()
        //         .all(|req_feature| class_info.class_features_list.contains(req_feature))
        // })
        .last()
    {
        let (parent_entity, _list_parent) = query_parent
            .iter()
            .filter(|(_, &list_parent)| list_parent == (Tab::Archetype).into())
            .next()
            .unwrap();
        let (class, archetype, arch_features) = arch_asset
            .iter()
            .filter(|(_handle, asset)| asset.archetype_name == ArchetypeName::Brawler)
            .map(|(_handle, asset)| {
                (
                    asset.class_name,
                    asset.archetype_name,
                    &asset.class_features,
                )
            })
            .next()
            .unwrap();
        let class_features = class_asset
            .iter()
            .filter(|(_handle, asset)| asset.class_name == PlayableClass::Fighter)
            .map(|(_handle, asset)| &asset.class_features)
            .next()
            .unwrap();

        let mut new_table = class_table.clone();

        new_table.subtab_list_parent.set_tab(Tab::Archetype);
        new_table.subtab_list_parent.set_subtab(SubTab::Progression);

        // first make a Vec<Vec<ClassFeature>> for lvls 1-20 for the modified class
        let mut prog_vec: Vec<Vec<ClassFeature>> = Vec::new();

        // second go over the class and insert any features that haven't been replaced.
        let replaced_features = archetype_info.replaces_features();
        for (i, class_level) in class_info.class_features.iter().enumerate() {
            let level = i + 1;
            let mut new_class_level: Vec<ClassFeature> = Vec::new();

            class_level
                .iter()
                .filter(|base_feature| !replaced_features.contains(base_feature))
                .for_each(|base_feature| new_class_level.push(*base_feature));

            let arch_ftr_iter = &archetype_info
                .archetype_features
                .iter()
                .flatten()
                .map(|archetype_feature| &archetype_feature.features)
                .flatten()
                .for_each(|(arch_level, feature)| {
                    if *arch_level == level {
                        new_class_level.push(feature.clone());
                    }
                });

            prog_vec.push(new_class_level);
        }

        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let text_style = TextStyle {
            font: shared_font.clone(),
            font_size: 18.,
            color: TEXT_COLOR,
        };
        let mut features_cells: Vec<FeaturesCell> = Vec::new();
        for row_i in 1..=20 {
            for col_i in 5..=5 {
                let mut feature_items: Vec<FeatureItem> = Vec::new();
                if col_i == 5 && row_i > 0 {
                    for (i, class_feature) in prog_vec[row_i - 1].iter().enumerate() {
                        let (feature_title, feature_desc, class_feature) = &arch_features
                            .iter()
                            .filter(|feature_desc| {
                                feature_desc.feature.as_default() == class_feature.as_default()
                            })
                            .map(|feature| (&feature.title, &feature.description, *class_feature))
                            .next()
                            .or_else(|| {
                                class_features
                                    .iter()
                                    .filter(|feature_desc| {
                                        feature_desc.class_feature_name.as_default()
                                            == class_feature.as_default()
                                    })
                                    .map(|feature| {
                                        (&feature.title, &feature.description, *class_feature)
                                    })
                                    .next()
                            })
                            .expect("feature not found in arch_asset or class_asset")
                            .clone();

                        let feature_title = feature_title.clone();
                        feature_items.push(FeatureItem {
                            title_string: {
                                if class_info.class_features[row_i - 1].len() > i + 1 {
                                    let mut title_string = feature_title.to_string();
                                    title_string.push_str(", ");
                                    title_string
                                } else {
                                    feature_title.clone()
                                }
                            },
                            text_style: text_style.clone(),
                            class_feature: *class_feature,
                            interaction: Interaction::default(),
                            tooltip: TooltipText(Text::from_sections([
                                TextSection::new(feature_title.clone(), text_style.clone()),
                                TextSection::new("\n", text_style.clone()),
                                TextSection::new(
                                    tooltip::first_99_words(String::from(feature_title)),
                                    text_style.clone(),
                                ),
                            ])),
                        });
                    }
                    features_cells.push(FeaturesCell {
                        cell_node: list_resource.list_row_node.clone(),
                        feature_items,
                        column: MyTable::Col(col_i),
                        cell: CellPosition {
                            row: row_i,
                            col: col_i,
                        },
                    });
                }
            }
        }

        // for replaced in archetype_info.replaces_features().iter() {
        //     for feature_item in class_table
        // }
        new_table.parent = Some(parent_entity);
        new_table.features_cells = features_cells;
        archetype_table_map
            .inner_ref_mut()
            .insert(archetype, new_table);
        arch_tables_built.set_true();
    }
}
// TODO: integrate this with class_resource::spawn_tables instead of copying the whole function
pub fn spawn_tables(
    class_tables: Res<ArchTablesMap>,
    class_map: Res<ArchetypeMap>,
    list_resource: Res<CentralListBundles>,
    mut class_tables_spawned: ResMut<ArchTableSpawned>,
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

    let tab = Tab::Archetype;
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
    for class in ArchetypeName::array() {
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
                            .spawn(header.clone().text_bundle)
                            .set_parent(*row_entity);
                    }
                }
                for text_cell in &class_table.text_cells {
                    if let Some(row_entity) = row_ids.get(&MyTable::Row(text_cell.cell.row)) {
                        commands
                            .spawn(text_cell.clone().text_bundle)
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
