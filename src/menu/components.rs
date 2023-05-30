use bevy::prelude::*;

use crate::menu::character_creation::components::RaceTab;

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

pub trait SelectedWrapper<V>
where
    V: Component + Eq + PartialEq + Copy,
{
    fn selected(&self) -> V;
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

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct CellPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Component, Debug, Copy, Clone, Default)]
pub enum MyTable {
    Row(usize),
    Col(usize),
    Cell([usize; 2]),
    #[default]
    None,
}

impl MyTable {
    pub fn build_2d<const C: usize, const R: usize>() -> [[[usize; 2]; C]; R] {
        let mut table_2d = [[[0_usize; 2]; C]; R];

        for (y, row) in table_2d.iter_mut().enumerate() {
            for (x, col) in row.iter_mut().enumerate() {
                *col = [x, y];
            }
        }
        table_2d
    }

    pub fn spawn_empty_from_styles<const R: usize, const C: usize>(
        mut commands: Commands,
        row_style: Style,
        col_style: Style,
    ) {
        let table_2d = Self::build_2d::<R, C>();
        for (row_i, row) in table_2d.iter().enumerate() {
            commands
                .spawn((
                    NodeBundle {
                        style: row_style.clone(),
                        ..default()
                    },
                    MyTable::Row(row_i),
                ))
                .with_children(|row_node| {
                    for (col_i, _col) in row.iter().enumerate() {
                        row_node.spawn((
                            NodeBundle {
                                style: col_style.clone(),
                                ..default()
                            },
                            MyTable::Col(col_i),
                        ));
                    }
                });
        }
    }
}
