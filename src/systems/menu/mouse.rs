use crate::systems::menu::components::ScrollingList;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

// Scroll list if the node or its direct children are being hovered.
pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<
        (
            Entity,
            &mut ScrollingList,
            &mut Style,
            &Parent,
            &Node,
            &Interaction,
        ),
        With<Children>,
    >,
    query_node: Query<&Node>,
    query_buttons: Query<(&Interaction, &Parent), With<Button>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (entity, mut scrolling_list, mut style, parent, list_node, interaction) in
            &mut query_list
        {
            let child_hovered = query_buttons.iter().any(|(interaction, b_parent)| {
                b_parent.get() == entity && *interaction == Interaction::Hovered
            });
            if *interaction == Interaction::Hovered || child_hovered {
                let items_height = list_node.size().y;
                let container_height = query_node.get(parent.get()).unwrap().size().y;

                let max_scroll = (items_height - container_height).max(0.);

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

pub fn scroll_snap_top(
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Node, &Parent)>,
    query_node: Query<&Node>,
) {
    for (mut scrolling_list, mut style, list_node, parent) in &mut query_list {
        let items_height = list_node.size().y;
        let container_height = query_node.get(parent.get()).unwrap().size().y;

        let max_scroll = (items_height - container_height).max(0.);

        // snap to top
        scrolling_list.position = 0.;
        style.position.top = Val::Px(scrolling_list.position);
    }
}
