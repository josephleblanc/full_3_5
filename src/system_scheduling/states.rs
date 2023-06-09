use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Battle,
    CharacterCreation,
}
