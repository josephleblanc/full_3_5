use crate::{
    constants::PATH_SIMPLE_FONT,
    menu::{
        character_creation::{
            components::{SubTab, SubTabListParent, Tab, TabListParent},
            constants::PROGRESSION_TABLE_HEADERS,
            layout::resource::CentralListBundles,
        },
        components::{CellPosition, MyTable},
        styles::TEXT_COLOR,
    },
    systems::game::class::ClassMap,
};

use bevy::{prelude::*, reflect::TypeUuid};

use super::{list_traits, select_item::BuiltLists};

pub fn build_progression<T, V, Q>(
    tab: Tab,
    subtab: SubTab,
) -> impl FnMut(
    Commands,
    Query<(Entity, &TabListParent)>,
    Res<Assets<T>>,
    Res<AssetServer>,
    Res<CentralListBundles>,
    ResMut<BuiltLists>,
    Res<ClassMap>,
)
where
    // This is the CustomAsset
    // e.g. RaceAsset, ClassAsset
    T: TypeUuid + Send + Sync + 'static + list_traits::HasKey<V> + list_traits::HasItemVec<Q>,
    // This is the identifying enum
    // e.g. PlayableRace, PlayableClass
    V: Component + list_traits::AsVec + Eq + PartialEq + std::fmt::Display + Copy,
    // The defining enum of the vector of items inside the asset, which shall be listed with this
    // function.
    Q: Component + Copy + Clone + std::fmt::Debug,
{
    move |mut commands: Commands,
          query_parent: Query<(Entity, &TabListParent)>,
          custom_asset: Res<Assets<T>>,
          asset_server: Res<AssetServer>,
          list_resource: Res<CentralListBundles>,
          mut res_built: ResMut<BuiltLists>,
          class_map: Res<ClassMap>,
          // try to remove this later
          | {
        let subtab_list_parent = SubTabListParent {
            tab,
            subtab,
        };

        if !res_built.inner_ref().contains(&subtab_list_parent) {
            let table_headers = PROGRESSION_TABLE_HEADERS;
            let table_2d = MyTable::build_2d::<6, 21>();

            let shared_font = asset_server.load(PATH_SIMPLE_FONT);
            let text_style = TextStyle {
                font: shared_font.clone(),
                font_size: 10.,
                color: TEXT_COLOR,
            };

            let key_vec = V::vec();
            let key_array = key_vec.as_slice();
            if let Some((parent_entity, _list_parent)) = query_parent.iter().filter(|(_, &list_parent)| list_parent == tab.into()).next() {
                let list_id = commands
                    .spawn((
                        list_resource.subtab_list_parent.clone(),
                        Name::from(format!("{tab} {subtab} select description node parent")),
                        subtab_list_parent,
                    ))
                    .set_parent(parent_entity)
                    .id();
                let cols_widths = [
                    60.0,
                    60.0,
                    60.0,
                    60.0,
                    60.0,
                    60.0,
                ];
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
                        MyTable::Row(row_i),
                    )).set_parent(list_id)
                    .with_children(|row_node| {
                        for (col_i, _col) in row.iter().enumerate() {
                                row_node.spawn((
                                    TextBundle {
                                    text: Text::from_section(
                                        // Headers for first row
                                        if row_i == 0 {
                                            String::from(table_headers[col_i].clone())
                                        } else {
                                            format!("|{row_i}, {col_i}|")
                                        },
                                        text_style.clone(),
                                    ),
                                    style: Style {
                                        padding: UiRect::top(Val::Px(5.)),
                                        size: Size::width(Val::Px(cols_widths[col_i])),
                                        ..default()
                                    },
                                    background_color: Color::rgba(0.2, 0.2, 0.2, 0.9).into(),
                                    ..default()
                                    },
                                    MyTable::Col(col_i),
                                    CellPosition {row: row_i, col: col_i}
                                ));
                        }
                    });
            }
                for (asset_key, asset_items_vec) in custom_asset.iter().map(|(_handle, asset)| {
                if tab == Tab::Class && subtab == SubTab::Progression {
                    println!("----> asset key: {}\t asset_items_vec len: {}", asset.key(), asset.vec().len());
                }
                    (asset.key(), asset.vec())
                }) {



                }
            }
            res_built.inner_mut().push(subtab_list_parent);
        }
    }
}
