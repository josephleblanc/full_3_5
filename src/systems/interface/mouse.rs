use crate::systems::menu::components::ScrollingList;
use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node, &Interaction)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        println!("Mouse event detected");
        for (mut scrolling_list, mut style, parent, list_node, interaction) in &mut query_list {
            if *interaction == Interaction::Hovered {
                let items_height = list_node.size().y;
                let container_height = query_node.get(parent.get()).unwrap().size().y;

                let max_scroll = (items_height - container_height).max(0.);
                println!(
                    "items_height: {}, container_height: {}",
                    items_height, container_height
                );

                let dy = match mouse_wheel_event.unit {
                    MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                    MouseScrollUnit::Pixel => mouse_wheel_event.y,
                };

                scrolling_list.position += dy;
                scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
                style.position.top = Val::Px(scrolling_list.position);
                println!("moved padding by {}", scrolling_list.position);
            }
        }
    }
}

pub fn hovering_over_button(query: Query<&Interaction, With<Button>>) -> bool {
    if !query.is_empty() {
        return true;
    }
    false
}

pub fn mouse_left_clicked(mut event: EventReader<MouseButtonInput>) -> bool {
    for click in event.iter() {
        println!("{:?}", click);
        if click.button == MouseButton::Left {
            return true;
        }
    }
    false
}
