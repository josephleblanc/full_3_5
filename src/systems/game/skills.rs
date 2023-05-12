use crate::systems::game::character;
use bevy::prelude::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq)]
pub enum SpellcraftUses {
    IdentifyCast,
    LearnSpell,
    PrepareSpell,
    IdentifyItem,
    DecipherScroll,
    CraftItem,
}

// SkillName
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq)]
pub enum SkillName {
    Acrobatics,
    Appraise,
    Bluff,
    Climb,
    Craft,
    Diplomacy,
    DisableDevice,
    Disguise,
    Escape,
    Fly,
    HandleAnimal,
    Heal,
    Intimidate,
    KnowledgeArcana,
    KnowledgeDungeoneering,
    KnowledgeEngineering,
    KnowledgeGeography,
    KnowledgeHistory,
    KnowledgeLocal,
    KnowledgeNature,
    KnowledgeNobility,
    KnowledgePlanes,
    KnowledgeReligion,
    Linguistics,
    Perception,
    Perform,
    Profession,
    Ride,
    SenseMotive,
    SleightOfHand,
    Spellcraft,
    Stealth,
    Survival,
    Swim,
    UseMagicDevice,
}

impl SkillName {
    pub fn iterator(&self) -> impl Iterator<Item = SkillName> {
        use SkillName::*;
        [
            Acrobatics,
            Appraise,
            Bluff,
            Climb,
            Craft,
            Diplomacy,
            DisableDevice,
            Disguise,
            Escape,
            Fly,
            HandleAnimal,
            Heal,
            Intimidate,
            KnowledgeArcana,
            KnowledgeDungeoneering,
            KnowledgeEngineering,
            KnowledgeGeography,
            KnowledgeHistory,
            KnowledgeLocal,
            KnowledgeNature,
            KnowledgeNobility,
            KnowledgePlanes,
            KnowledgeReligion,
            Linguistics,
            Perception,
            Perform,
            Profession,
            Ride,
            SenseMotive,
            SleightOfHand,
            Spellcraft,
            Stealth,
            Survival,
            Swim,
            UseMagicDevice,
        ]
        .iter()
        .copied()
    }
}

impl Display for SkillName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use SkillName::*;
        match self {
            Acrobatics => write!(f, "Acrobatics"),
            Appraise => write!(f, "Appraise"),
            Bluff => write!(f, "Bluff"),
            Climb => write!(f, "Climb"),
            Craft => write!(f, "Craft"),
            Diplomacy => write!(f, "Diplomacy"),
            DisableDevice => write!(f, "DisableDevice"),
            Disguise => write!(f, "Disguise"),
            Escape => write!(f, "Escape"),
            Fly => write!(f, "Fly"),
            HandleAnimal => write!(f, "HandleAnimal"),
            Heal => write!(f, "Heal"),
            Intimidate => write!(f, "Intimidate"),
            KnowledgeArcana => write!(f, "KnowledgeArcana"),
            KnowledgeDungeoneering => write!(f, "KnowledgeDungeoneering"),
            KnowledgeEngineering => write!(f, "KnowledgeEngineering"),
            KnowledgeGeography => write!(f, "KnowledgeGeography"),
            KnowledgeHistory => write!(f, "KnowledgeHistory"),
            KnowledgeLocal => write!(f, "KnowledgeLocal"),
            KnowledgeNature => write!(f, "KnowledgeNature"),
            KnowledgeNobility => write!(f, "KnowledgeNobility"),
            KnowledgePlanes => write!(f, "KnowledgePlanes"),
            KnowledgeReligion => write!(f, "KnowledgeReligion"),
            Linguistics => write!(f, "Linguistics"),
            Perception => write!(f, "Perception"),
            Perform => write!(f, "Perform"),
            Profession => write!(f, "Profession"),
            Ride => write!(f, "Ride"),
            SenseMotive => write!(f, "SenseMotive"),
            SleightOfHand => write!(f, "SleightOfHand"),
            Spellcraft => write!(f, "Spellcraft"),
            Stealth => write!(f, "Stealth"),
            Survival => write!(f, "Survival"),
            Swim => write!(f, "Swim"),
            UseMagicDevice => write!(f, "UseMagicDevice"),
        }
    }
}
