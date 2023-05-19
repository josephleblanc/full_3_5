use crate::systems::game::character::PlayableRace;
use bevy::prelude::*;
use serde::Deserialize;
use std::fmt;

#[derive(Default, Deserialize, Clone, Component, Debug)]
pub struct FavoredClass {
    pub class: PlayableClass,
    pub race: PlayableRace,
    pub description: String,
    pub source: String,
}

#[derive(Default, Deserialize, Copy, Clone, Debug)]
pub enum PlayableClass {
    Alchemist,
    Arcanist,
    Barbarian,
    Bard,
    Bloodrager,
    Brawler,
    Cavalier,
    Cleric,
    Druid,
    Fighter,
    Gunslinger,
    Hunter,
    Inquisitor,
    Investigator,
    Kineticist,
    Magus,
    Medium,
    Mesmerist,
    Monk,
    Occultist,
    Oracle,
    Paladin,
    Psychic,
    Ranger,
    Rogue,
    Shaman,
    Skald,
    Slayer,
    Sorcerer,
    Spiritualist,
    Summoner,
    Swashbuckler,
    Vigilante,
    Warpriest,
    Witch,
    Wizard,
    #[default]
    None,
}

impl fmt::Display for PlayableClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Alchemist => write!(f, "Alchemist"),
            Self::Arcanist => write!(f, "Arcanist"),
            Self::Barbarian => write!(f, "Barbarian"),
            Self::Bard => write!(f, "Bard"),
            Self::Bloodrager => write!(f, "Bloodrager"),
            Self::Brawler => write!(f, "Brawler"),
            Self::Cavalier => write!(f, "Cavalier"),
            Self::Cleric => write!(f, "Cleric"),
            Self::Druid => write!(f, "Druid"),
            Self::Fighter => write!(f, "Fighter"),
            Self::Gunslinger => write!(f, "Gunslinger"),
            Self::Hunter => write!(f, "Hunter"),
            Self::Inquisitor => write!(f, "Inquisitor"),
            Self::Investigator => write!(f, "Investigator"),
            Self::Kineticist => write!(f, "Kineticist"),
            Self::Magus => write!(f, "Magus"),
            Self::Medium => write!(f, "Medium"),
            Self::Mesmerist => write!(f, "Mesmerist"),
            Self::Monk => write!(f, "Monk"),
            Self::Occultist => write!(f, "Occultist"),
            Self::Oracle => write!(f, "Oracle"),
            Self::Paladin => write!(f, "Paladin"),
            Self::Psychic => write!(f, "Psychic"),
            Self::Ranger => write!(f, "Ranger"),
            Self::Rogue => write!(f, "Rogue"),
            Self::Shaman => write!(f, "Shaman"),
            Self::Skald => write!(f, "Skald"),
            Self::Slayer => write!(f, "Slayer"),
            Self::Sorcerer => write!(f, "Sorcerer"),
            Self::Spiritualist => write!(f, "Spiritualist"),
            Self::Summoner => write!(f, "Summoner"),
            Self::Swashbuckler => write!(f, "Swashbuckler"),
            Self::Vigilante => write!(f, "Vigilante"),
            Self::Warpriest => write!(f, "Warpriest"),
            Self::Witch => write!(f, "Witch"),
            Self::Wizard => write!(f, "Wizard"),
            Self::None => write!(f, "None"),
        }
    }
}
