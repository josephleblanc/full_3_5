use crate::{
    menu::components::SelectedWrapper,
    systems::game::{
        archetype::ArchetypeName,
        character::{AbilityScore, PlayableRace},
        class::PlayableClass,
        race::RacialTraitName,
    },
};

use bevy::prelude::*;

use super::layout::generics::list_traits::AsVec;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Tab {
    Race,
    Class,
}

impl AsVec for Tab {
    fn vec() -> Vec<Self> {
        vec![Self::Race, Self::Class]
    }
}

impl Into<TabButton> for Tab {
    fn into(self) -> TabButton {
        TabButton(self)
    }
}

impl Into<ListParent> for Tab {
    fn into(self) -> ListParent {
        match self {
            Self::Race => ListParent::Race,
            Self::Class => ListParent::Class,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SelectTabEvent {
    pub entity: Entity,
    pub tab: Tab,
    pub tab_state: InTab,
}

#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq)]
pub struct SelectedTab(pub Tab);
impl SelectedTab {
    pub fn new(other: Tab) -> SelectedTab {
        SelectedTab(other)
    }
}
impl SelectedWrapper<Tab> for SelectedTab {
    fn selected(&self) -> Tab {
        self.0
    }
}

#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq)]
pub struct SelectedSubTab(pub SubTab);
impl SelectedSubTab {
    pub fn new(other: SubTab) -> SelectedSubTab {
        SelectedSubTab(other)
    }
}
impl SelectedWrapper<SubTab> for SelectedSubTab {
    fn selected(&self) -> SubTab {
        self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SelectSubTabEvent {
    pub tab: Tab,
    pub subtab: SubTab,
    pub entity: Entity,
    pub tab_state: InTab,
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SubTab {
    RaceDescription,
    RaceDefaultTraits,
    RaceAltTraits,
    RaceFavoredClass,
    ClassDescription,
    ClassFeatures,
    ClassArchetype,
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq)]
pub struct SubTabListParent {
    pub tab: Tab,
    pub subtab: SubTab,
}

#[derive(Copy, Clone)]
pub struct SelectButton {
    pub entity: Entity,
    pub tab: Tab,
    pub subtab: SubTab,
    pub tab_state: InTab,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct TabButton(Tab);
impl TabButton {
    pub fn inner(&self) -> Tab {
        self.0
    }
    pub fn new(other: Tab) -> SelectedTab {
        SelectedTab(other)
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct SubTabButton(SubTab);

#[derive(Component, Copy, Clone, Debug)]
pub enum InTab {
    Entering,
    Exiting,
}

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
pub struct SelectedCreationTab(pub CreationTab);

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
impl Into<SelectedCreationTab> for CreationTab {
    fn into(self) -> SelectedCreationTab {
        SelectedCreationTab(self)
    }
}

// TODO: Delete the inner method for SelectedCreationTab and replace it with
// the method from SelectedWrapper instead.
impl SelectedCreationTab {
    pub fn inner(&self) -> CreationTab {
        self.0
    }
}
impl SelectedWrapper<CreationTab> for SelectedCreationTab {
    fn selected(&self) -> CreationTab {
        self.0
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum ListParent {
    Race,
    Class,
}

impl std::fmt::Display for ListParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Race => write!(f, "Race"),
            Self::Class => write!(f, "Class"),
        }
    }
}

impl Into<Tab> for ListParent {
    fn into(self) -> Tab {
        match self {
            Self::Race => Tab::Race,
            Self::Class => Tab::Race,
        }
    }
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
impl SelectedWrapper<PlayableClass> for SelectedClass {
    fn selected(&self) -> PlayableClass {
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

impl Into<SelectedClassTab> for ClassTab {
    fn into(self) -> SelectedClassTab {
        SelectedClassTab(self)
    }
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
pub struct SelectedRace(pub PlayableRace);

// used in character_creation generics
impl SelectedWrapper<PlayableRace> for SelectedRace {
    fn selected(&self) -> PlayableRace {
        self.0
    }
}

impl SelectedRace {
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

impl Into<SelectedRaceTab> for RaceTab {
    fn into(self) -> SelectedRaceTab {
        SelectedRaceTab(self)
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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash, Default)]
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
// TODO: Replace the inner() method wherever it appears with the `selected` method
// below instead, and delete `inner`.
impl SelectedRaceTab {
    pub fn inner(&self) -> RaceTab {
        self.0
    }
}
impl SelectedWrapper<RaceTab> for SelectedRaceTab {
    fn selected(&self) -> RaceTab {
        self.0
    }
}
