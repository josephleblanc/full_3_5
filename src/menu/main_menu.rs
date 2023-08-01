use crate::{
    menu::{
        components::{Container, NavBarButtonType},
        styles::{
            MAIN_MENU_TITLE_STYLE, NAV_BAR_STYLE, NAV_BUTTON_BOTTOM_STYLE, NAV_BUTTON_MIDDLE_STYLE,
            NAV_BUTTON_TOP_STYLE,
        },
    },
    my_camera::my_camera_systems,
    system_scheduling::states::AppState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonSize {
    width: f32,
    height: f32,
}

#[derive(Component)]
pub struct MenuTitle;

#[derive(Component)]
pub struct MainMenuContainer;

use bevy::ui::Display;
// Setup the main menu, checking first that the labeled camera already exists.
// The check for the camera works because this system is run on entering
// AppState::MainMenu, which is after camera setup in .on_startup()
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Camera, With<my_camera_systems::MainCamera>>,
) {
    // Check for a camera
    if camera_query.iter().next().is_none() {
        panic!("--> my debug: setup_main_menu did not find a camera with the PrimaryWindow marker");
    }
    let shared_font = asset_server.load("fonts/simple_font.TTF");

    // create a simple black background
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    align_items: AlignItems::End,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Container::TopLevel,
            MainMenuContainer,
        ))
        .with_children(|parent| {
            // create menu title
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Main Menu",
                        TextStyle {
                            font: shared_font.clone(),
                            font_size: 60.,
                            color: Color::WHITE,
                        },
                    ),
                    style: MAIN_MENU_TITLE_STYLE,
                    ..default()
                },
                MenuTitle,
            ));
        })
        .with_children(|parent| {
            // Central container for other items from other menus.
            parent.spawn((
                Container::Central,
                NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Auto,
                        ..default()
                    },
                    background_color: Color::PURPLE.into(),
                    ..default()
                },
            ));
        })
        .with_children(|parent| {
            // Container node for navigation bar buttons
            parent
                .spawn((
                    Container::NavBar,
                    NodeBundle {
                        style: NAV_BAR_STYLE,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // create Battle button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: NAV_BUTTON_TOP_STYLE,
                                background_color: Color::rgb(0.2, 0.2, 0.8).into(),
                                ..Default::default()
                            },
                            NavBarButtonType::Battle,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Battle",
                                TextStyle {
                                    font: shared_font.clone(),
                                    font_size: 30.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                })
                .with_children(|parent| {
                    // create Battle button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: NAV_BUTTON_MIDDLE_STYLE,
                                background_color: Color::rgb(0.2, 0.2, 0.8).into(),
                                ..Default::default()
                            },
                            NavBarButtonType::CharacterCreation,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Character Creation",
                                TextStyle {
                                    font: shared_font.clone(),
                                    font_size: 30.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                })
                .with_children(|parent| {
                    // create Exit button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: NAV_BUTTON_BOTTOM_STYLE,
                                background_color: Color::rgb(0.8, 0.2, 0.2).into(),
                                ..default()
                            },
                            NavBarButtonType::Exit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    font: shared_font.clone(),
                                    font_size: 30.,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
}

pub fn main_menu_cleanup(
    mut commands: Commands,
    mut query: Query<Entity, With<MainMenuContainer>>,
) {
    if let Ok(menu_container_entity) = query.get_single_mut() {
        println!("\n--> main_menu_cleanup found Entity with MainMenuContainer");
        commands.entity(menu_container_entity).despawn_recursive();
    } else {
        println!("--> main_menu_cleanup not working");
    }
}

use bevy::app::AppExit;
pub fn button_system(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(&NavBarButtonType, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for (button_type, interaction) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button_type {
                NavBarButtonType::Battle => {
                    next_state.set(AppState::Battle);
                    println!(
                        "--> set new app state to Battle crate::systems::main_menu::button_system"
                    );
                    return;
                }
                NavBarButtonType::CharacterCreation => {
                    next_state.set(AppState::CharacterCreation);
                    println!("--> set new app state to Character creation in crate::systems::main_menu::button_system");
                    return;
                }
                NavBarButtonType::Exit => {
                    println!("--> sending exit event in crate::systems::main_menu::button_system");
                    app_exit_event_writer.send(AppExit);
                    return;
                }
                NavBarButtonType::Empty => {
                    println!("yeah, this doesn't do anything");
                }
            };
        }
    }
}
