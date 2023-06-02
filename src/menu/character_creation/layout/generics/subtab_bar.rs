use crate::constants::PATH_SIMPLE_FONT;
use crate::menu::character_creation::generics::SubTab;
use crate::menu::{
    character_creation::components::{SubTabButton, SubTabButtonText},
    styles::TEXT_COLOR,
};
use crate::systems::layout::character_creation::ButtonBar;
use bevy::prelude::*;

use super::list_traits::AsVec;

// pub fn build_button_bar<B, S, V>(
//     subtab_button: SubTabButton<S>,
// ) -> impl FnMut(Commands, Query<Entity, (With<B>, With<Node>)>, Res<AssetServer>)
// where
//     B: ButtonBar + Component + Copy + Default,
//     // This is the subtab identifier specified when the function is called,
//     // e.g. RaceTab::AlternateTraits, ClassTab::ClassFeatures
//     S: Component + Clone + Copy + SubTab + std::fmt::Display + AsVec,
// {
//     move |mut commands: Commands,
//           query_parent: Query<Entity, (With<B>, With<Node>)>,
//           asset_server: Res<AssetServer>| {
//         let button_bar = query_parent.get_single().unwrap();
//         let shared_font = asset_server.load(PATH_SIMPLE_FONT);
//         commands
//             .spawn((
//                 NodeBundle {
//                     style: Style {
//                         flex_direction: FlexDirection::Column,
//                         size: Size::new(Val::Percent(100.), Val::Percent(100.)),
//                         align_self: AlignSelf::Center,
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     background_color: Color::BEIGE.into(), // RACIAL_CHOICES_NODE_COLOR,
//                     ..default()
//                 },
//                 Name::from("Choose Description Content Container"),
//             ))
//             .with_children(|button_container| {
//                 button_container
//                     .spawn((
//                         NodeBundle {
//                             background_color: Color::YELLOW_GREEN.into(), // RACIAL_CHOICES_PANEL_COLOR,
//                             ..default()
//                         },
//                         Name::from("Button Container - Choose Description Content"),
//                     ))
//                     .with_children(|list| {
//                         for subtab_enum in S::vec() {
//                             list.spawn((
//                                 ButtonBundle {
//                                     style: Style {
//                                         padding: UiRect::all(Val::Px(5.)),
//                                         margin: UiRect::all(Val::Px(5.)),
//                                         ..default()
//                                     },
//                                     background_color: Color::PURPLE.into(),
//                                     ..default()
//                                 },
//                                 subtab_enum,
//                                 SubTabButton::<S>::new(subtab_enum),
//                                 Name::from("Button: Choose Description Content"),
//                             ))
//                             .with_children(|list_button| {
//                                 list_button.spawn((
//                                     TextBundle {
//                                         text: Text::from_section(
//                                             subtab_enum.to_string(),
//                                             TextStyle {
//                                                 font: shared_font.clone(),
//                                                 font_size: 25.,
//                                                 color: TEXT_COLOR,
//                                             },
//                                         ),
//                                         background_color: Color::VIOLET.into(), // RACIAL_CHOICES_TEXT_BG_COLOR,
//                                         ..default()
//                                     },
//                                     SubTabButtonText,
//                                 ));
//                             });
//                         }
//                     });
//             })
//             .set_parent(button_bar);
//     }
// }
