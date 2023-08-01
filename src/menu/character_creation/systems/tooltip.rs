use crate::menu::character_creation::components::*;
use bevy::prelude::*;

pub fn display_on_hover(
    query_trait: Query<(&Interaction, &TooltipText)>,
    query_change: Query<&Interaction, (Changed<Interaction>, With<TooltipText>)>,
    mut timer: ResMut<TooltipTimer>,
    time: Res<Time>,
    mut event_reader: EventReader<CursorMoved>,
    mut query_tooltip: Query<(&mut Style, &mut Text), With<Tooltip>>,
) {
    for (interaction, tooltip_text) in &query_trait {
        if *interaction == Interaction::Hovered && timer.inner_mut().tick(time.delta()).finished() {
            if let Some(cursor_event) = event_reader.iter().last() {
                let (mut tooltip_style, mut tooltip) = query_tooltip.get_single_mut().unwrap();
                tooltip_style.display = Display::Flex;
                let mut calculated_tooltip_left = Val::Px(cursor_event.position.x - 20.);
                calculated_tooltip_left
                    .try_sub_assign(tooltip_style.width)
                    .unwrap();
                // tooltip_style.position = UiRect {
                //     left: calculated_tooltip_left,
                //     bottom: Val::Px(cursor_event.position.y),
                //     ..default()
                // };
                tooltip_style.left = calculated_tooltip_left;
                tooltip_style.bottom = Val::Px(cursor_event.position.y);
                *tooltip = tooltip_text.0.clone();
                println!(
                    "tooltip left and bottom values: (left: {:#?}) (bottom: {:#?})",
                    tooltip_style.left, tooltip_style.bottom
                );
                println!("tooltip position: {:#?}", cursor_event.position);
            }
        }
    }
    for changed_interaction in &query_change {
        if *changed_interaction == Interaction::None {
            let (mut tooltip_style, mut tooltip) = query_tooltip.get_single_mut().unwrap();
            tooltip_style.display = Display::None;
            *tooltip = Text::default();
            timer.inner_mut().reset();
        }
    }
}

pub fn first_99_words(string: String) -> String {
    let tooltip_text = string
        .split(' ')
        .enumerate()
        .filter(|(i, _)| *i < 100)
        .map(|(i, word)| if i != 99 { word } else { "..." })
        .collect::<Vec<&str>>()
        .join(" ");
    tooltip_text
}
