use crate::systems::game::{character::*, class::*, skills::*};
use bevy::prelude::*;
use serde::Deserialize;
use std::fmt;

#[derive(Default, Debug, Deserialize, Clone, Component)]
pub enum ClassArchetype {
    Fighter(FighterArchetype),
    #[default]
    None,
}

impl fmt::Display for ClassArchetype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Fighter(_) => write!(f, "Archer"),
            Self::None => write!(f, "None"),
        }
    }
}

#[derive(Default, Debug, Deserialize, Clone, Hash)]
pub struct FighterArchetype {
    name: ArchetypeName,
    class: PlayableClass,
    restrictions: Option<Vec<Restriction>>,
    archetype_features: Option<Vec<ArchetypeFeature>>,
    skills: Option<Vec<SkillName>>,
    skill_ranks: Option<usize>,
    gains_proficiency: Option<usize>,
    loses_proficiency: Option<usize>,
}

#[derive(Component, Default, Debug, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum ArchetypeName {
    #[default]
    Archer,
}

impl ArchetypeName {
    pub fn array() -> [ArchetypeName; 1] {
        [Self::Archer]
    }

    pub fn iterator() -> impl Iterator<Item = ArchetypeName> {
        [Self::Archer].iter().copied()
    }
    pub fn class(&self) -> PlayableClass {
        match self {
            Self::Archer => PlayableClass::Fighter,
        }
    }
}

impl fmt::Display for ArchetypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Archer => write!(f, "Archer"),
        }
    }
}

#[derive(Default, Debug, Deserialize, Clone, Component, Hash)]
pub enum Restriction {
    Race(PlayableRace),
    #[default]
    None,
}

#[derive(Default, Debug, Deserialize, Clone, Component, Hash)]
pub struct ArchetypeFeature {
    feature: ClassFeature,
    replaces: Vec<ClassFeature>,
    level: usize,
}
