use bevy::prelude::*;

pub const RACE_DESCRIPTION_FOLDER: &str = "text/descriptions/races";
pub const RACIAL_DEFAULT_TRAITS_DESCRIPTION_FOLDER: &str = "text/descriptions/races/default_traits";
pub const RACIAL_ALT_TRAITS_FOLDER: &str = "text/descriptions/races/alternate_traits";
pub const CLASS_DESCRIPTIONS_FOLDER: &str = "text/descriptions/class";
pub const ARCHETYPE_DESCRIPTIONS_FOLDER: &str = "text/descriptions/class/archetypes";
pub const CLASS_DESCRIPTION_TITLE: &'static str = "Class Description";

pub const PROGRESSION_TABLE_HEADERS: [&'static str; 6] = [
    "Level",
    "Base Attack Bonus",
    "Fort Save",
    "Ref Save",
    "Will Save",
    "Special",
];

// Central List Styles
pub const LIST_PARENT_NODE_STYLE: Style = {
    // padding: UiRect::all(Val::Px(5.)),
    let mut style = Style::DEFAULT;
    style.margin = UiRect::all(Val::Px(10.));
    style.flex_direction = FlexDirection::Column;
    style.row_gap = Val::Px(10.);
    style
};
pub const LIST_NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.padding = UiRect::all(Val::Px(5.));
    // style.margin = UiRect::all(Val::Px(10.));
    style.flex_direction = FlexDirection::Column;
    style.display = Display::None;
    style
};
pub const LIST_ITEM_TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.max_width = Val::Px(1200.);
    style.margin = UiRect::all(Val::Px(10.));
    style
};
pub const LIST_ROW_NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    // style.padding = UiRect::all(Val::Px(5.));
    style.margin = UiRect::all(Val::Px(10.));
    style.flex_direction = FlexDirection::Row;
    style
};
pub const LIST_COL_NODE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.margin = UiRect::all(Val::Px(10.));
    style.flex_direction = FlexDirection::Column;
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};
pub const LIST_BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.);
    // style.padding = UiRect::left(Val::Percent(7.));
    style
};
pub const REPLACEMENT_ITEM_TEXT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.align_items = AlignItems::Center;
    style
};
pub const LIST_DESCRIPTION_TEXT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.max_width = Val::Px(900.);
    style.margin = UiRect::all(Val::Px(20.));
    style
};

// SubTab Button Styles
pub const SUBTAB_BUTTON_FONT: f32 = 25.;
pub const SUBTAB_BUTTON_BUNDLE_COLOR: Color = Color::PURPLE;
pub const SUBTAB_BUTTON_TEXT_COLOR: Color = Color::VIOLET;
