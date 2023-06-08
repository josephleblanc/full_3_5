use bevy::prelude::*;

pub const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";
pub const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/race/default_traits";
pub const CLASS_DESCRIPTIONS_FOLDER: &str = "text/descriptions/class";
pub const ARCHETYPE_DESCRIPTIONS_FOLDER: &str = "text/descriptions/class/archetypes";
pub const CLASS_DESCRIPTION_TITLE: &'static str = "Class Description";

// Central List Styles
pub const LIST_PARENT_NODE_STYLE: Style = Style {
    // padding: UiRect::all(Val::Px(5.)),
    margin: UiRect::all(Val::Px(10.)),
    flex_direction: FlexDirection::Column,
    gap: Size::height(Val::Px(10.)),
    ..Style::DEFAULT
};
pub const LIST_NODE_STYLE: Style = Style {
    padding: UiRect::all(Val::Px(5.)),
    // margin: UiRect::all(Val::Px(10.)),
    flex_direction: FlexDirection::Column,
    display: Display::None,
    ..Style::DEFAULT
};
pub const LIST_ITEM_TITLE_STYLE: Style = Style {
    max_size: Size::width(Val::Px(1200.)),
    margin: UiRect::all(Val::Px(10.)),
    ..Style::DEFAULT
};
pub const LIST_ROW_NODE_STYLE: Style = Style {
    // padding: UiRect::all(Val::Px(5.)),
    margin: UiRect::all(Val::Px(10.)),
    flex_direction: FlexDirection::Row,
    ..Style::DEFAULT
};
pub const LIST_COL_NODE_STYLE: Style = Style {
    margin: UiRect::all(Val::Px(10.)),
    flex_direction: FlexDirection::Column,
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};
pub const LIST_BUTTON_STYLE: Style = Style {
    size: Size::width(Val::Percent(100.)),
    // padding: UiRect::left(Val::Percent(7.)),
    ..Style::DEFAULT
};
pub const REPLACEMENT_ITEM_TEXT_STYLE: Style = Style {
    align_items: AlignItems::Center,
    ..Style::DEFAULT
};
pub const LIST_DESCRIPTION_TEXT_STYLE: Style = Style {
    max_size: Size::width(Val::Px(900.)),
    margin: UiRect::all(Val::Px(20.)),
    ..Style::DEFAULT
};

// SubTab Button Styles
pub const SUBTAB_BUTTON_FONT: f32 = 25.;
pub const SUBTAB_BUTTON_BUNDLE_COLOR: Color = Color::PURPLE;
pub const SUBTAB_BUTTON_TEXT_COLOR: Color = Color::VIOLET;
