use crate::systems::game::archetype::ArchetypeName;
use crate::systems::{
    game::character::{AbilityScore, PlayableRace},
    game::race::RacialTraitName,
};
use bevy::prelude::*;

use crate::systems::game::class::PlayableClass;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum LeftPanelEnum {
    Race(PlayableRace),
    Class(PlayableClass),
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct LeftPanelList;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct LeftPanelTitle;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct LeftPanelText;

impl LeftPanelEnum {
    pub fn get_race(&self) -> Option<PlayableRace> {
        match self {
            Self::Race(playable_race) => Some(*playable_race),
            _ => None,
        }
    }
    pub fn get_class(&self) -> Option<PlayableClass> {
        match self {
            Self::Class(playable_class) => Some(*playable_class),
            _ => None,
        }
    }
}
impl std::fmt::Display for LeftPanelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Race(race) => write!(f, "{}", race),
            Self::Class(class) => write!(f, "{}", class),
        }
    }
}

impl Default for LeftPanelEnum {
    fn default() -> LeftPanelEnum {
        LeftPanelEnum::Race(PlayableRace::Human)
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct LeftPanelButton;

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub struct CreationTabSelected(pub CreationTab);

impl Into<CreationTab> for ListParent {
    fn into(self) -> CreationTab {
        match self {
            Self::Race => CreationTab::Race,
            Self::Class => CreationTab::Class,
            // _ => CreationTab::Optional,
        }
    }
}
impl ListParent {
    pub fn display(
        mut query: Query<(&mut Style, &ListParent)>,
        selected_tab: Res<CreationTabSelected>,
    ) {
        for (mut style, list_parent) in &mut query {
            let creation_tab = selected_tab.inner();
            if creation_tab == (*list_parent).into() {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
pub enum CreationTab {
    #[default]
    Race,
    AbilityScores,
    Class,
    Skills,
    Feats,
    BonusFeats,
    Optional,
}
impl CreationTabSelected {
    pub fn inner(&self) -> CreationTab {
        self.0
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum ListParent {
    Race,
    Class,
}

#[derive(Component, Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct ArchetypeItem;

#[derive(Resource, Copy, Clone, Debug, Default, PartialEq)]
pub struct SelectedClass(pub PlayableClass);
impl SelectedClass {
    pub fn inner(&self) -> PlayableClass {
        self.0
    }
}

#[derive(Resource, Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd)]
pub struct SelectedClassTab(pub ClassTab);
impl SelectedClassTab {
    pub fn inner(&self) -> ClassTab {
        self.0
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum ClassTab {
    #[default]
    Description,
    ClassFeatures,
    Progression,
    Archetypes,
}
impl ClassTab {
    pub fn array() -> [ClassTab; 4] {
        [
            Self::Description,
            Self::ClassFeatures,
            Self::Archetypes,
            Self::Progression,
        ]
    }
}
impl std::fmt::Display for ClassTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ClassTab::Description => write!(f, "Description"),
            ClassTab::ClassFeatures => write!(f, "Class Features"),
            ClassTab::Progression => write!(f, "Progression"),
            ClassTab::Archetypes => write!(f, "Archetypes"),
        }
    }
}

// Label for the Class left panel with selectable classes.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct ClassPanel;

// Lable for archetype left panel in the class tab and archetype subtab, with
// selectable archetypes, and the name of the class at the top.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct ArchetypePanel;

#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct SelectedArchetype(pub ArchetypeName);
impl SelectedArchetype {
    pub fn inner(&self) -> ArchetypeName {
        self.0
    }
}

#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct SelectedRaceButton(pub PlayableRace);

impl SelectedRaceButton {
    pub fn inner(&self) -> PlayableRace {
        self.0
    }
}

// Common traits displayed in the right panel of race selection in
// character creation.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum CommonTraits {
    #[default]
    Size,
    Speed,
    Type,
    Subtype,
    /* more here */
}

impl CommonTraits {
    pub fn as_array() -> [CommonTraits; 4] {
        [Self::Size, Self::Speed, Self::Type, Self::Subtype]
    }
}

impl std::fmt::Display for RaceTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::RaceDescription => write!(f, "Race Description"),
            Self::StandardTraitTab => write!(f, "Standard Racial Traits"),
            Self::AltTraitTab => write!(f, "Alternate Racial Traits"),
            Self::FavoredClassTab => write!(f, "Favored Class Option"),
        }
    }
}

impl RaceTab {
    pub fn array() -> [RaceTab; 4] {
        [
            Self::RaceDescription,
            Self::StandardTraitTab,
            Self::AltTraitTab,
            Self::FavoredClassTab,
        ]
    }
}

#[derive(Component, Copy, Clone)]
pub struct TestChosenStandardTrait;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum SubTabButton {
    Race(RaceTab),
    Class(ClassTab),
}
impl SubTabButton {
    pub fn get_race_tab(&self) -> Option<RaceTab> {
        match self {
            Self::Race(race) => Some(*race),
            Self::Class(_) => None,
        }
    }
    pub fn get_class_tab(&self) -> Option<ClassTab> {
        match self {
            Self::Race(_) => None,
            Self::Class(class) => Some(*class),
        }
    }
}

impl Default for SubTabButton {
    fn default() -> SubTabButton {
        Self::Race(RaceTab::RaceDescription)
    }
}
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct SubTabButtonText;

impl std::fmt::Display for CommonTraits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Self::Size => write!(f, "Size"),
            Self::Speed => write!(f, "Speed"),
            Self::Type => write!(f, "Type"),
            Self::Subtype => write!(f, "Subtype"),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub enum RaceTab {
    #[default]
    RaceDescription,
    StandardTraitTab,
    AltTraitTab,
    FavoredClassTab,
}

// Label for the Race left panel containing a list of selectable races
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacePanel;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AltTraitReplaces(pub Vec<RacialTraitName>);

// Right Panel Titles
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenStandardTraitTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenAlternateTraitTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct FavoredClassTitle;
// Right Panel Values
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenStandardTrait;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ChosenAlternateTrait;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct FavoredClassValueText;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialABSDisplay(AbilityScore);

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButtonText;

// Label for buttons that let you select a racial trait to replace a default
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct RacialTraitButton;

// Label for default race description text bundles
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct DefaultTraitDescriptionText;

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct DefaultRacialTraitRace(pub PlayableRace);

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListNode;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListTitle;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ButtonText;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct Description;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ListButtonColumn;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ReplacesText;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ReplacesContent;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct RaceItem;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct ClassItem;

// #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
// pub struct AlternateTrait;

// Bottom container buttons
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct CharacterSheetButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct PreviousButton;
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub struct NextButton;

// Tooltip labels
#[derive(Component, Clone, Debug, Copy)]
pub struct Tooltip;
#[derive(Component, Clone, Debug)]
pub struct TooltipText(pub Text);
#[derive(Resource, Clone)]
pub struct TooltipTimer(pub Timer);

impl TooltipTimer {
    pub fn inner_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct DescriptionSection;

#[derive(Component, Clone, Debug)]
pub struct RightPanel;

// Holds the currently selected race for reference by other functions.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct SelectedRaceTab(pub RaceTab);
impl SelectedRaceTab {
    pub fn inner(&self) -> RaceTab {
        self.0
    }
}
