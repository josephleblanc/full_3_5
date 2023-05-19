use bevy::prelude::*;

use crate::systems::menu::character_creation::*;

#[derive(Component, Debug)]
pub enum Container {
    Central,
    NavBar,
    Title,
    TopLevel,
}

#[derive(Default, Component)]
pub enum NavBarButtonType {
    Battle,
    CharacterCreation,
    Exit,
    #[default]
    Empty,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum StagesOfCreationButton {
    Race,
    AbilityScores,
    Class,
    Skills,
    Feats,
    BonusFeats,
    Optional,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RaceSelectButton;

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RaceDescriptionNode(pub RaceTab);
impl RaceDescriptionNode {
    pub fn inner(&self) -> RaceTab {
        self.0
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq)]
pub struct RaceDescriptionNodeParent;

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct ScrollingList {
    pub position: f32,
}
