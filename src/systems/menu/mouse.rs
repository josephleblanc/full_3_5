use crate::systems::menu::components::ScrollingList;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

// Scroll list if the node or any of its descendants are hovered.
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
    query_children: Query<&Children>,
    query_node: Query<&Node>,
    query_buttons: Query<(Entity, &Interaction)>,
) {
    let mut child_hovered = false;
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (entity, mut scrolling_list, mut style, parent, list_node, interaction) in
            &mut query_list
        {
            for hovered_ent in query_buttons
                .iter()
                .filter(|(_, ent_interaction)| **ent_interaction == Interaction::Hovered)
                .map(|(ent, _)| ent)
            {
                if query_children
                    .iter_descendants(entity)
                    .into_iter()
                    .any(|descendant| hovered_ent == descendant)
                {
                    child_hovered = true;
                } else {
                    child_hovered = false;
                }
            }
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

pub fn scroll_snap_top(mut query_list: Query<(&mut ScrollingList, &mut Style)>) {
    for (mut scrolling_list, mut style) in &mut query_list {
        // snap to top
        scrolling_list.position = 0.;
        style.position.top = Val::Px(scrolling_list.position);
    }
}
