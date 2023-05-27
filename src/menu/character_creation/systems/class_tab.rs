use crate::menu::character_creation::components::*;
use crate::menu::character_creation::constants::*;
use crate::menu::styles::*;
use crate::technical::archetype::ArchetypeAsset;
use crate::technical::class::ClassAsset;
use bevy::prelude::*;

pub fn display_list_node(
    selected_tab: Res<SelectedClassTab>,
    selected_class: Res<SelectedClass>,
    selected_archetype: Res<SelectedArchetype>,
    class_asset: Res<Assets<ClassAsset>>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
    mut query_node: Query<&mut Style, (With<ListNode>, With<ClassItem>)>,
) {
    if let Some(len) = {
        match selected_tab.inner() {
            ClassTab::Description => Some(1_usize),
            ClassTab::ClassFeatures => {
                if let Some((_handle, class_asset)) = class_asset
                    .iter()
                    .filter(|(_handle, class_asset)| {
                        class_asset.class_name == selected_class.inner()
                    })
                    .next()
                {
                    Some(class_asset.class_features.len())
                } else {
                    None
                }
            }
            ClassTab::Archetypes => {
                if let Some((_handle, archetype_asset)) = archetype_asset
                    .iter()
                    .filter(|(_handle, archetype)| {
                        archetype.archetype_name == selected_archetype.inner()
                    })
                    .next()
                {
                    Some(archetype_asset.class_features.len())
                } else {
                    None
                }
            }
            ClassTab::Progression => None,
        }
    } {
        for (i, mut node_style) in query_node.iter_mut().enumerate() {
            if i < len {
                println!("making class feature visible");
                node_style.display = Display::Flex;
            } else {
                node_style.display = Display::None;
            }
        }
    }
}
pub fn display_list_title(
    selected_tab: Res<SelectedClassTab>,
    selected_archetype: Res<SelectedArchetype>,
    selected_class: Res<SelectedClass>,
    mut query_title: Query<(&mut Text, &mut Style), (With<ListTitle>, With<ClassItem>)>,
    class_asset: Res<Assets<ClassAsset>>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let titles = {
        match selected_tab.inner() {
            ClassTab::Description => Some(vec![CLASS_DESCRIPTION_TITLE.to_string()]),
            ClassTab::ClassFeatures => {
                if let Some((_handle, class_asset)) = class_asset
                    .iter()
                    .filter(|(_handle, class_asset)| {
                        class_asset.class_name == selected_class.inner()
                    })
                    .next()
                {
                    Some(
                        class_asset
                            .class_features
                            .iter()
                            .map(|class_features| class_features.title.clone())
                            .collect(),
                    )
                } else {
                    None
                }
            }

            ClassTab::Archetypes => {
                if let Some((_handle, archetype_asset)) = archetype_asset
                    .iter()
                    .filter(|(_handle, archetype_asset)| {
                        archetype_asset.archetype_name == selected_archetype.inner()
                    })
                    .next()
                {
                    Some(
                        archetype_asset
                            .class_features
                            .iter()
                            .map(|class_feature| class_feature.title.clone())
                            .collect(),
                    )
                } else {
                    None
                }
            }
            ClassTab::Progression => None,
        }
    };
    if let Some(titles) = titles {
        let mut titles_iter = titles.iter();
        for (mut list_title, mut style) in &mut query_title {
            if let Some(title) = titles_iter.next() {
                println!("class_feature title: {}", title);
                *list_title = Text::from_section(
                    title,
                    TextStyle {
                        font: font.clone(),
                        font_size: LIST_TITLE_TEXT_SIZE,
                        color: TEXT_COLOR,
                    },
                );
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (_, mut style) in &mut query_title {
            style.display = Display::None;
        }
    }
}
pub fn display_list_text(
    selected_tab: Res<SelectedClassTab>,
    selected_archetype: Res<SelectedArchetype>,
    selected_class: Res<SelectedClass>,
    mut query_descr: Query<(&mut Style, &mut Text), (With<Description>, With<ClassItem>)>,
    class_asset: Res<Assets<ClassAsset>>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    let descr: Option<Vec<&String>> = match selected_tab.inner() {
        ClassTab::Description => Some(
            class_asset
                .iter()
                .filter(|(_handle, class_asset)| class_asset.class_name == selected_class.inner())
                .map(|(_handle, class)| &class.description)
                .collect(),
        ),
        ClassTab::ClassFeatures => {
            if let Some((_handle, class_asset)) = class_asset
                .iter()
                .filter(|(_handle, class_asset)| class_asset.class_name == selected_class.inner())
                .next()
            {
                Some(
                    class_asset
                        .class_features
                        .iter()
                        .map(|class_features| {
                            println!("class_features.title = {}", class_features.description);

                            &class_features.description
                        })
                        .collect(),
                )
            } else {
                None
            }
        }

        ClassTab::Archetypes => {
            if let Some((_handle, archetype_asset)) = archetype_asset
                .iter()
                .filter(|(_handle, archetype_asset)| {
                    archetype_asset.archetype_name == selected_archetype.inner()
                })
                .next()
            {
                Some(
                    archetype_asset
                        .class_features
                        .iter()
                        .map(|class_feature| &class_feature.description)
                        .collect(),
                )
            } else {
                None
            }
        }
        ClassTab::Progression => None,
    };
    if let Some(descriptions) = descr {
        let mut descriptions_iter = descriptions.iter();
        for (mut style, mut text) in query_descr.iter_mut() {
            if let Some(text_descr) = descriptions_iter.next() {
                *text = Text::from_section(
                    *text_descr,
                    TextStyle {
                        font: font.clone(),
                        font_size: 25.,
                        color: Color::WHITE,
                    },
                );
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    } else {
        for (mut style, _) in &mut query_descr {
            style.display = Display::None;
        }
    }
}
