use bevy::prelude::*;

use crate::{
    menu::character_creation::components::{LeftPanelEvent, ListNode, Status},
    systems::game::{archetype::ArchetypeName, character::PlayableRace, class::PlayableClass},
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
    let mut peekable_reader = event_reader.iter().peekable();
    println!("----> display_race");
    while let (Some(current), peeked) = (peekable_reader.next(), peekable_reader.peek()) {
        if peeked.is_some() && peeked.unwrap().race.is_some() && current.race.is_some() {
            for event in [current, peeked.unwrap()] {
                if let Some(status) = event.status {
                    println!("\tcurrent: {current:?}\n\tpeeked {peeked:?}\n");
                    for (mut style, _identifier) in
                        query_list_items.iter_mut().filter(|(_style, identifier)| {
                            if let Some(race) = event.race {
                                // Checks that the current and next events in the iterator are both of
                                // the same item, e.g. PlayableRace.
                                // This is to prevent selected items from changing
                                // when other items are selected in different tabs, for example
                                // PlayableRace in another Panel
                                race == **identifier
                            } else {
                                false
                            }
                        })
                    {
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
    }
}
pub fn display_class(
    mut query_list_items: Query<(&mut Style, &PlayableClass), With<ListNode>>,
    mut event_reader: EventReader<LeftPanelEvent>,
) {
    let mut peekable_reader = event_reader.iter().peekable();
    println!("----> display_class");
    while let (Some(current), peeked) = (peekable_reader.next(), peekable_reader.peek()) {
        // peek exists and has class field, current has class field
        if peeked.is_some() && peeked.unwrap().class.is_some() && current.class.is_some() {
            // this changes the targets of both peeked and current, which is fine because the last
            // iterator item consumed will not pass the peek.is_some() test
            for event in [current, peeked.unwrap()] {
                if let Some(status) = event.status {
                    println!("\tcurrent: {current:?}\n\tpeeked {peeked:?}\n");
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
    }
}
pub fn display_archetype(
    mut query_list_items: Query<(&mut Style, &ArchetypeName), With<ListNode>>,
    mut event_reader: EventReader<LeftPanelEvent>,
) {
    let mut peekable_reader = event_reader.iter().peekable();
    println!("----> display_archetype");
    while let (Some(current), peeked) = (peekable_reader.next(), peekable_reader.peek()) {
        // peek exists and has archetype field, current has archetype field
        if peeked.is_some() && peeked.unwrap().archetype.is_some() && current.archetype.is_some() {
            // this changes the targets of both peeked and current, which is fine because the last
            // iterator item consumed will not pass the peek.is_some() test
            for event in [current, peeked.unwrap()] {
                if let Some(status) = event.status {
                    println!("\tcurrent: {current:?}\n\tpeeked {peeked:?}\n");
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
    }
}
