use bevy::prelude::*;

use crate::{
    menu::character_creation::components::{LeftPanelEvent, ListNode, Status},
    systems::game::{archetype::MyArchetypeName, character::PlayableRace, class::PlayableClass},
};

// The display systems for the central area of character creation.
//
// The central list items built by, e.g. generics::description, should be displayed when the
// identifying enum in the left panel is clicked. When the leftpanel item is clicked, an event is
// sent with the item being selected like so:
//
//  LeftPanelEvent {
//      race: Option<PlayableRace>,
//      class: Option<PlayableClass>,
//      /* more as needed */
//      status: Option</* either Status::Entering or Status::Exiting */>
//      }
//
// When the LeftPanelEvent is sent, the matching race or class in the central area should be
// displayed if `status` is `Entering`, and hidden if `status` is `Exiting`. Because the items in
// the central list are set to Display::None by default, this will be sufficient to display the
// selected item and hide all others.
//
// This approach allows for the selections from the other tabs to persist when changing tabs. If we
// chose "Elf" under the Race tab, then switched to the Class tab and selected "Cleric", then
// switched back the "Race" tab, then "Elf" would still be selected.

// TODO: See if there is some way to get a bool out of comparing the types of two things, and use
// that to turn the following three display_<something> into a generic like display<T> instead
pub fn display_race(
    mut query_list_items: Query<(&mut Style, &PlayableRace), With<ListNode>>,
    mut event_reader: EventReader<LeftPanelEvent>,
) {
    info!("running display_race");
    let mut peekable_reader = event_reader.iter().peekable();
    while let (Some(current), Some(peeked)) = (peekable_reader.next(), peekable_reader.peek()) {
        for event in [current, peeked] {
            info!("-> event received");
            if let Some(status) = event.status {
                info!("{}", format!("--> event status: {:?}", status));
                for (mut style, identifier) in
                    query_list_items.iter_mut().filter(|(_, identifier)| {
                        if let Some(race) = event.race {
                            race == **identifier
                        } else {
                            false
                        }
                    })
                {
                    info!(
                        "---> identifier match found for event {} and race {}",
                        event.race.unwrap(),
                        identifier
                    );
                    match status {
                        Status::Entering => {
                            info!("----> setting display to Flex");
                            style.display = Display::Flex;
                        }
                        Status::Exiting => {
                            info!("----> setting display to None");
                            style.display = Display::None;
                        }
                    }
                }
            }
        }
    }
}
pub fn display_class(
    mut query_list_items: Query<(&mut Style, &PlayableClass), With<ListNode>>,
    mut event_reader: EventReader<LeftPanelEvent>,
) {
    for event in event_reader.iter() {
        if let Some(status) = event.status {
            for (mut style, _) in query_list_items.iter_mut().filter(|(_, identifier)| {
                if let Some(class) = event.class {
                    class == **identifier
                } else {
                    false
                }
            }) {
                match status {
                    Status::Entering => {
                        style.display = Display::Flex;
                    }
                    Status::Exiting => {
                        style.display = Display::None;
                    }
                }
            }
        }
    }
}
pub fn display_archetype(
    mut query_list_items: Query<(&mut Style, &MyArchetypeName), With<ListNode>>,
    mut event_reader: EventReader<LeftPanelEvent>,
) {
    for event in event_reader.iter() {
        if let Some(status) = event.status {
            for (mut style, _) in query_list_items.iter_mut().filter(|(_, identifier)| {
                if let Some(archetype) = event.archetype {
                    archetype == **identifier
                } else {
                    false
                }
            }) {
                match status {
                    Status::Entering => {
                        style.display = Display::Flex;
                    }
                    Status::Exiting => {
                        style.display = Display::None;
                    }
                }
            }
        }
    }
}
