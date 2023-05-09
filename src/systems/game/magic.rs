use crate::systems::game::character;
use bevy::prelude::*;

#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq, Ord)]
pub enum CasterLevelUse {
    Cast,
    Dispel,
    OvercomeSpellResistance,
}

impl character::Limitation for CasterLevelUse {}
