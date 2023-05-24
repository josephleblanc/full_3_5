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
    mut query_node: Query<(&mut Text, &mut Style), (With<ListTitle>, With<ClassItem>)>,
    class_asset: Res<Assets<ClassAsset>>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    if let Some(titles) = {
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
                            .map(|class_features| {
                                println!("class_features.title = {}", class_features.title);

                                class_features.title.clone()
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
                            .map(|class_feature| class_feature.title.clone())
                            .collect(),
                    )
                } else {
                    None
                }
            }
            ClassTab::Progression => None,
        }
    } {
        println!(
            "---------------> class_asset.class_features has len = {}",
            titles.len()
        );
        let mut titles_iter = titles.iter();
        for (mut list_title, mut style) in &mut query_node {
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
    }
}
pub fn display_list_text(
    selected_tab: Res<SelectedClassTab>,
    selected_archetype: Res<SelectedArchetype>,
    selected_class: Res<SelectedClass>,
    mut query_node: Query<(&mut Text, &mut Style), (With<Description>, With<ClassItem>)>,
    class_asset: Res<Assets<ClassAsset>>,
    archetype_asset: Res<Assets<ArchetypeAsset>>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/simple_font.TTF");
    if let Some(description) = {
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
                            .map(|class_features| {
                                println!("class_features.title = {}", class_features.description);

                                class_features.description.clone()
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
                            .map(|class_feature| class_feature.description.clone())
                            .collect(),
                    )
                } else {
                    None
                }
            }
            ClassTab::Progression => None,
        }
    } {
        let mut titles_iter = description.iter();
        for (mut list_description, mut style) in &mut query_node {
            if let Some(description) = titles_iter.next() {
                style.display = Display::Flex;
                *list_description = Text::from_section(
                    description,
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
    }
}
pub fn selected_tab(
    mut selected: ResMut<SelectedClassTab>,
    mut interaction_query: Query<
        (&Interaction, &SubTabButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    let selection_copy = selected.inner();
    for (interaction, interacted_subtab, mut color) in &mut interaction_query {
        if let SubTabButton::Class(class_subtab) = interacted_subtab {
            match *interaction {
                Interaction::Clicked => {
                    if selection_copy != *class_subtab {
                        *color = RACE_BUTTON_COLOR_SELECTED.into();
                        *selected = SelectedClassTab(*class_subtab);
                        println!("SelectedClassTab: {:#?}", selected);
                    }
                }
                Interaction::Hovered => {
                    if selection_copy != *class_subtab {
                        *color = RACE_BUTTON_COLOR_HOVERED.into();
                    }
                }
                Interaction::None => {
                    if selection_copy != *class_subtab {
                        *color = RACE_BUTTON_COLOR.into();
                    }
                }
            }
        }
    }
    // pub fn button_col(
    //     query: Query<&mut Style, (With<ListButtonColumn>, With<ClassItem>)>,
    //     selected_class_tab: Res<SelectedClassTab>,
    // ) {
    // }
}
