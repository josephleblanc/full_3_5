use bevy::prelude::*;

pub const MAIN_MENU_TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.left = Val::Px(50.);
    style.top = Val::Px(550.);
    style
};

pub const NAV_BUTTON_TOP_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.);
    style.height = Val::Px(50.);
    style.margin = UiRect {
        top: Val::Auto,
        bottom: Val::Px(5.),
        ..UiRect::DEFAULT
    };
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const NAV_BUTTON_MIDDLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.);
    style.height = Val::Px(50.);
    style.margin = UiRect {
        top: Val::Px(5.),
        bottom: Val::Px(5.),
        ..UiRect::DEFAULT
    };
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const NAV_BUTTON_BOTTOM_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.);
    style.height = Val::Px(50.);
    style.margin = UiRect {
        top: Val::Px(5.),
        bottom: Val::Auto,
        ..UiRect::DEFAULT
    };
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub const NAV_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.);
    style.height = Val::Percent(100.);
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
    style.flex_direction = FlexDirection::Column;
    style.margin = UiRect {
        right: Val::Percent(8.),
        ..UiRect::DEFAULT
    };
    style
};

pub const CHARACTER_CREATION_TITLE_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Auto;
    style.height = Val::Px(50.);
    style.display = Display::Flex;
    style.align_items = AlignItems::Center;
    style.margin = UiRect {
        bottom: Val::Px(15.),
        ..UiRect::DEFAULT
    };
    style
};

pub const CHARACTER_SHEET_STYLE_H2: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style
};

pub const STAGES_OF_CREATION_BUTTON: Style = {
    let mut style = Style::DEFAULT;
    style.display = Display::Flex;
    style.padding = UiRect {
        left: Val::Px(10.),
        right: Val::Px(10.),
        top: Val::Px(10.),
        bottom: Val::Px(10.),
    };
    style.margin = UiRect {
        left: Val::Px(8.),
        right: Val::Px(8.),
        top: Val::Px(8.),
        bottom: Val::Px(8.),
    };
    style
};
pub const TEXT_COLOR: Color = Color::WHITE;

pub const STAGES_OF_CREATION_BUTTON_COLOR: Color = Color::PURPLE;
pub const RACE_BUTTON_COLOR_HOVERED: Color = Color::GRAY;
pub const RACE_BUTTON_COLOR_SELECTED: Color = Color::SEA_GREEN;
pub const RACE_BUTTON_COLOR: Color = Color::DARK_GRAY;

// List button for central area, used for things like Alt traits select
pub const LIST_BUTTON_COLOR: Color = Color::DARK_GREEN;
pub const LIST_BUTTON_TEXT_SIZE: f32 = 30.;
pub const LIST_TITLE_TEXT_SIZE: f32 = 30.;

pub const STAGES_OF_CREATION_FONT_SIZE: f32 = 30.;
pub const STAGES_OF_CREATION_TEXT_COLOR: Color = Color::WHITE;
pub const STAGES_OF_CREATION_TEXT_STYLE: Style = {
    let style = Style::DEFAULT;
    style
};

pub const RACIAL_CHOICES_NODE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(34., 58., 77.));
pub const RACIAL_CHOICES_PANEL_COLOR: BackgroundColor = BackgroundColor(Color::rgb(34., 58., 77.));
pub const RACIAL_CHOICES_BUTTON_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(71., 101., 101.));
pub const RACIAL_CHOICES_TEXT_BG_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(48., 102., 105.));

pub const SUBTAB_BUTTON_FONT: f32 = 25.;

pub const LEFT_PANEL_FONT_SIZE: f32 = 30.;
pub const PANEL_TITLE_COLOR: Color = Color::INDIGO;
pub const DESCRIPTION_FONT_SIZE: f32 = 25.;

pub const DESCRIPTION_MAX_WIDTH: f32 = 900.;
