use bevy::prelude::*;

pub const MAIN_MENU_TITLE_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    position: UiRect {
        left: Val::Px(50.),
        top: Val::Px(550.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const NAV_BUTTON_TOP_STYLE: Style = Style {
    size: Size::new(Val::Px(200.), Val::Px(50.)),
    margin: UiRect {
        top: Val::Auto,
        bottom: Val::Px(5.),
        ..UiRect::DEFAULT
    },
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const NAV_BUTTON_MIDDLE_STYLE: Style = Style {
    size: Size::new(Val::Px(200.), Val::Px(50.)),
    margin: UiRect {
        top: Val::Px(5.),
        bottom: Val::Px(5.),
        ..UiRect::DEFAULT
    },
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const NAV_BUTTON_BOTTOM_STYLE: Style = Style {
    size: Size::new(Val::Px(200.), Val::Px(50.)),
    margin: UiRect {
        top: Val::Px(5.),
        bottom: Val::Auto,
        ..UiRect::DEFAULT
    },
    align_items: AlignItems::Center,
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const NAV_BAR_STYLE: Style = Style {
    size: Size::new(Val::Px(200.), Val::Percent(100.)),
    display: Display::Flex,
    align_items: AlignItems::Center,
    flex_direction: FlexDirection::Column,
    margin: UiRect {
        right: Val::Percent(8.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const CHARACTER_CREATION_TITLE_STYLE: Style = Style {
    size: Size::new(Val::Auto, Val::Px(50.)),
    display: Display::Flex,
    align_items: AlignItems::Center,
    margin: UiRect {
        bottom: Val::Px(15.),
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const CHARACTER_SHEET_STYLE_H2: Style = Style {
    display: Display::Flex,
    ..Style::DEFAULT
};

pub const STAGES_OF_CREATION_BUTTON: Style = Style {
    display: Display::Flex,
    padding: UiRect {
        left: Val::Px(10.),
        right: Val::Px(10.),
        top: Val::Px(10.),
        bottom: Val::Px(10.),
    },
    margin: UiRect {
        left: Val::Px(8.),
        right: Val::Px(8.),
        top: Val::Px(8.),
        bottom: Val::Px(8.),
    },
    ..Style::DEFAULT
};
pub const TEXT_COLOR: Color = Color::WHITE;

pub const STAGES_OF_CREATION_BUTTON_COLOR: Color = Color::PURPLE;
pub const RACE_BUTTON_COLOR_HOVERED: Color = Color::GRAY;
pub const RACE_BUTTON_COLOR_SELECTED: Color = Color::SEA_GREEN;
pub const RACE_BUTTON_COLOR: Color = Color::DARK_GRAY;

// List button for central area, used for things like Alt traits select
pub const LIST_BUTTON_COLOR: Color = Color::DARK_GREEN;
pub const LIST_BUTTON_TEXT_SIZE: f32 = 30.;

pub const STAGES_OF_CREATION_FONT_SIZE: f32 = 30.;
pub const STAGES_OF_CREATION_TEXT_COLOR: Color = Color::WHITE;
pub const STAGES_OF_CREATION_TEXT_STYLE: Style = Style { ..Style::DEFAULT };

pub const RACIAL_CHOICES_NODE_COLOR: BackgroundColor = BackgroundColor(Color::rgb(34., 58., 77.));
pub const RACIAL_CHOICES_PANEL_COLOR: BackgroundColor = BackgroundColor(Color::rgb(34., 58., 77.));
pub const RACIAL_CHOICES_BUTTON_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(71., 101., 101.));
pub const RACIAL_CHOICES_TEXT_BG_COLOR: BackgroundColor =
    BackgroundColor(Color::rgb(48., 102., 105.));
