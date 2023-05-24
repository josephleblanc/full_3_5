use crate::menu::character_creation::components::*;
use bevy::prelude::*;

pub fn chosen_trait_tooltip(
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
                    .try_sub_assign(tooltip_style.size.width)
                    .unwrap();
                tooltip_style.position = UiRect {
                    left: calculated_tooltip_left,
                    bottom: Val::Px(cursor_event.position.y),
                    ..default()
                };
                *tooltip = tooltip_text.0.clone();
                println!("tooltip position: {:#?}", tooltip_style.position);
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
