use crate::constants::*;
use crate::menu::character_creation::constants::*;
use crate::menu::styles::*;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CentralListBundles {
    // A bundle of bundles used to spawn a list with a unified style.
    pub subtab_list_parent: NodeBundle,
    pub list_node: NodeBundle,
    pub list_item_title: TextBundle,
    pub list_row_node: NodeBundle,
    pub list_col_node: NodeBundle,
    pub list_button: ButtonBundle,
    pub list_button_text: TextBundle,
    pub skill_replaces_text: TextBundle,
    pub skill_replacement_item_text: TextBundle,
    pub list_description_text: TextBundle,
}

impl CentralListBundles {
    pub fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
        // A function that may be turned into a system to initialize the resource before
        // it is referenced by systems which utilize the CentralListBundles.
        let shared_font = asset_server.load(PATH_SIMPLE_FONT);
        let subtab_list_parent = NodeBundle {
            style: LIST_PARENT_NODE_STYLE,
            // background_color: Color::rgb_linear(20., 20., 20.).into(),
            ..default()
        };
        let list_node = NodeBundle {
            style: LIST_NODE_STYLE,
            background_color: Color::BLACK.into(),
            ..default()
        };

        let list_item_title = TextBundle {
            text: Text::from_section(
                "Select Me!",
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: LIST_ITEM_TITLE_STYLE,
            ..default()
        };
        let list_row_node = NodeBundle {
            style: Style {
                // padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(10.)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        };
        let list_col_node = NodeBundle {
            style: Style {
                // padding: UiRect::all(Val::Px(5.)),
                margin: UiRect::all(Val::Px(10.)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        };
        let list_button = ButtonBundle {
            style: Style {
                size: Size::width(Val::Percent(100.)),
                // padding: UiRect::left(Val::Percent(7.)),
                ..default()
            },
            background_color: Color::DARK_GREEN.into(),
            ..default()
        };
        let list_button_text = TextBundle::from_section(
            "List Button Text".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: LIST_BUTTON_TEXT_SIZE,
                color: TEXT_COLOR,
            },
        );
        let skill_replaces_text = TextBundle::from_section(
            "Replaces".to_string(),
            TextStyle {
                font: shared_font.clone(),
                font_size: 30.,
                color: TEXT_COLOR,
            },
        );
        let skill_replacement_item_text = TextBundle {
            text: Text::from_section(
                "Alt Race Replaces:".to_string(),
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: Style {
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        };
        let list_description_text = TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: shared_font.clone(),
                    font_size: 30.,
                    color: TEXT_COLOR,
                },
            ),
            style: Style {
                max_size: Size::width(Val::Px(900.)),
                margin: UiRect::left(Val::Px(20.)),
                ..default()
            },
            ..default()
        };
        let central_list_bundles = CentralListBundles {
            subtab_list_parent,
            list_node,
            list_item_title,
            list_row_node,
            list_col_node,
            list_button,
            list_button_text,
            skill_replaces_text,
            skill_replacement_item_text,
            list_description_text,
        };
        commands.insert_resource(central_list_bundles);
    }
}
