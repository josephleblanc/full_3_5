use crate::{
    menu::character_creation::layout::generics::list_traits::AsVec,
    systems::game::{character::*, class::*, skills::*},
};
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;
use std::fmt;

use super::resources::class_resource::ClassTable;

#[derive(Resource, Copy, Clone, Default, PartialEq)]
pub struct ArchTableBuilt(pub bool);
impl ArchTableBuilt {
    pub fn set_true(&mut self) {
        self.0 = true;
    }
}
#[derive(Resource, Copy, Clone, Default, PartialEq)]
pub struct ArchTableSpawned(pub bool);
impl ArchTableSpawned {
    pub fn set_true(&mut self) {
        self.0 = true;
    }
}

#[derive(Resource, Default)]
pub struct ArchTablesMap(pub HashMap<ArchetypeName, ClassTable>);
impl ArchTablesMap {
    pub fn inner_ref(&self) -> &HashMap<ArchetypeName, ClassTable> {
        &self.0
    }
    pub fn inner_ref_mut(&mut self) -> &mut HashMap<ArchetypeName, ClassTable> {
        &mut self.0
    }
}

#[derive(Resource, Default, Deserialize, Clone, Debug)]
pub struct ArchetypeMap(HashMap<ArchetypeName, ArchetypeInfo>);
impl ArchetypeMap {
    pub fn new() -> Self {
        let map: HashMap<ArchetypeName, ArchetypeInfo> = HashMap::new();
        Self(map)
    }
    pub fn inner_ref(&self) -> &HashMap<ArchetypeName, ArchetypeInfo> {
        &self.0
    }
    pub fn inner_ref_mut(&mut self) -> &mut HashMap<ArchetypeName, ArchetypeInfo> {
        &mut self.0
    }
}

#[derive(Default, Deserialize, Clone, Debug)]
pub struct ArchetypeInfo {
    pub name: ArchetypeName,
    pub class: PlayableClass,
    pub restrictions: Option<Vec<Restriction>>,
    pub archetype_features: Option<Vec<ArchetypeFeature>>,
    pub skills: Option<Vec<SkillName>>,
    pub skill_ranks: Option<usize>,
    pub gains_proficiency: Option<usize>,
    pub loses_proficiency: Option<usize>,
}

/// Gets all the class features this archetype replaces in a vec
impl ArchetypeInfo {
    pub fn replaces_features(&self) -> Vec<ClassFeature> {
        let mut out = vec![];

        if let Some(archetype_features) = &self.archetype_features {
            for features in archetype_features {
                out.extend_from_slice(features.replaces.as_slice());
            }
        }
        out
    }
}

#[derive(Component, Default, Debug, Deserialize, Copy, Clone, Eq, PartialEq, PartialOrd, Hash)]
pub enum ArchetypeName {
    #[default]
    Archer,
    Brawler,
}

impl ArchetypeName {
    pub fn array() -> [ArchetypeName; 2] {
        [Self::Archer, Self::Brawler]
    }

    pub fn iterator() -> impl Iterator<Item = ArchetypeName> {
        [Self::Archer, Self::Brawler].iter().copied()
    }
    pub fn class(&self) -> PlayableClass {
        match self {
            Self::Archer => PlayableClass::Fighter,
            Self::Brawler => PlayableClass::Fighter,
        }
    }
}

impl AsVec for ArchetypeName {
    fn vec() -> Vec<Self> {
        Vec::from(ArchetypeName::array())
    }
}
impl fmt::Display for ArchetypeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Archer => write!(f, "Archer"),
            Self::Brawler => write!(f, "Brawler"),
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
    pub features: Vec<(usize, ClassFeature)>,
    pub replaces: Vec<ClassFeature>,
}
