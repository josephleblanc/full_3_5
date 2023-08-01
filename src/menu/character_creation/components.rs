use crate::{
    menu::components::SelectedWrapper,
    systems::game::{
        archetype::ArchetypeName,
        character::{AbilityScore, PlayableRace},
        class::PlayableClass,
        race::RacialTraitName,
    },
};

use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::layout::generics::list_traits::{AsButtonList, AsVec};

#[derive(Hash, Component, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Default)]
pub enum Tab {
    #[default]
    Race,
    Class,
    Archetype,
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Race => write!(f, "Race"),
            Self::Class => write!(f, "Class"),
            Self::Archetype => write!(f, "Archetype"),
        }
    }
}

impl Into<SelectedTab> for Tab {
    fn into(self) -> SelectedTab {
        SelectedTab(self)
    }
}

impl AsVec for Tab {
    fn vec() -> Vec<Self> {
        vec![Self::Race, Self::Class, Self::Archetype]
    }
}

impl Into<TabButton> for Tab {
    fn into(self) -> TabButton {
        TabButton(self)
    }
}

impl Into<TabListParent> for Tab {
    fn into(self) -> TabListParent {
        match self {
            Self::Race => TabListParent::Race,
            Self::Class => TabListParent::Class,
            Self::Archetype => TabListParent::Archetype,
        }
    }
}

#[derive(Copy, Clone, Debug, Event)]
pub struct SelectTabEvent {
    pub entity: Entity,
    pub tab: Tab,
    pub tab_state: InTab,
}

#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq, Default)]
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

// TODO: Consider changing this to a struct which contains the selected subtab for each tab.
#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq, Default)]
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

#[derive(Copy, Clone, Debug, Event)]
pub struct SelectSubTabEvent {
    pub tab: Tab,
    pub subtab: SubTab,
    pub entity: Entity,
    pub tab_state: InTab,
}

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum SubTab {
    #[default]
    Description,
    DefaultTraits,
    AltTraits,
    FavoredClass,
    Features,
    Progression,
}

impl std::fmt::Display for SubTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Description => write!(f, "Description"),
            Self::DefaultTraits => write!(f, "Default Traits"),
            Self::AltTraits => write!(f, "Alt Traits"),
            Self::FavoredClass => write!(f, "Favored Class"),
            Self::Features => write!(f, "Features"),
            Self::Progression => write!(f, "Progression"),
        }
    }
}

// impl AsVec for SubTab {
//     fn vec() -> Vec<Self> {
//         vec![
//             Self::RaceDescription,
//             Self::RaceDefaultTraits,
//             Self::RaceAltTraits,
//             Self::RaceFavoredClass,
//             Self::ClassDescription,
//             Self::ClassFeatures,
//             Self::ArchetypeInfo,
//         ]
//     }
// }

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

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub struct SubTabButton {
    pub tab: Tab,
    pub subtab: SubTab,
}

impl SubTabButton {
    fn new(tab: Tab, subtab: SubTab) -> Self {
        Self { tab, subtab }
    }
}

impl std::fmt::Display for SubTabButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.tab, self.subtab)
    }
}

impl AsButtonList for SubTabButton {
    fn button_list() -> Vec<Self> {
        vec![
            Self::new(Tab::Race, SubTab::Description),
            Self::new(Tab::Race, SubTab::DefaultTraits),
            Self::new(Tab::Race, SubTab::AltTraits),
            Self::new(Tab::Class, SubTab::Description),
            Self::new(Tab::Class, SubTab::Progression),
            Self::new(Tab::Class, SubTab::Features),
            Self::new(Tab::Archetype, SubTab::Description),
            Self::new(Tab::Archetype, SubTab::Features),
            Self::new(Tab::Archetype, SubTab::Progression),
        ]
    }
}

impl Into<SubTab> for SubTabButton {
    fn into(self) -> SubTab {
        self.subtab
    }
}

#[derive(Resource, Clone, Debug)]
pub struct SelectedSubTabsMap(pub HashMap<Tab, SubTab>);
impl SelectedSubTabsMap {
    pub fn as_ref_mut(&mut self) -> &mut HashMap<Tab, SubTab> {
        &mut self.0
    }
    pub fn as_ref(&self) -> &HashMap<Tab, SubTab> {
        &self.0
    }
}

impl Default for SelectedSubTabsMap {
    fn default() -> Self {
        let mut hash_map: HashMap<Tab, SubTab> = HashMap::new();
        hash_map.insert(Tab::Race, SubTab::Description);
        hash_map.insert(Tab::Class, SubTab::Description);
        hash_map.insert(Tab::Archetype, SubTab::Description);
        SelectedSubTabsMap(hash_map)
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub enum InTab {
    Entering,
    Exiting,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum LeftPanelEnum {
    Race(PlayableRace),
    Class(PlayableClass),
    Archetype(ArchetypeName),
}

// impl Into<LeftPanelEvent> for LeftPanelEnum {
//     fn into(self) -> LeftPanelEvent {
//         match self {
//             Self::Race(race) => LeftPanelEvent {
//                 race: Some(race),
//                 ..default()
//             },
//             Self::Class(class) => LeftPanelEvent {
//                 class: Some(class),
//                 ..default()
//             },
//         }
//     }
// }

#[derive(Copy, Clone, Debug, Event)]
/// Event sent when clicking on a button in the left panel of character creation.
/// Used to determine whether a node in the central area is displayed or hidden.
pub struct LeftPanelEvent {
    pub race: Option<PlayableRace>,
    pub class: Option<PlayableClass>,
    pub archetype: Option<ArchetypeName>,
    pub status: Option<Status>,
}

impl LeftPanelEvent {
    pub fn set_status(mut self, other: Status) -> Self {
        self.status = Some(other);
        self
    }
}

impl From<PlayableRace> for LeftPanelEvent {
    fn from(value: PlayableRace) -> Self {
        Self {
            race: Some(value),
            ..default()
        }
    }
}
impl From<PlayableClass> for LeftPanelEvent {
    fn from(value: PlayableClass) -> Self {
        Self {
            class: Some(value),
            ..default()
        }
    }
}
impl From<ArchetypeName> for LeftPanelEvent {
    fn from(value: ArchetypeName) -> Self {
        Self {
            archetype: Some(value),
            ..default()
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Entering,
    Exiting,
}

impl Default for LeftPanelEvent {
    fn default() -> Self {
        LeftPanelEvent {
            race: None,
            class: None,
            archetype: None,
            status: None,
        }
    }
}

impl From<LeftPanelEnum> for LeftPanelEvent {
    fn from(value: LeftPanelEnum) -> Self {
        match value {
            LeftPanelEnum::Race(race) => Self {
                race: Some(race),
                ..default()
            },
            LeftPanelEnum::Class(class) => Self {
                class: Some(class),
                ..default()
            },
            LeftPanelEnum::Archetype(archetype) => Self {
                archetype: Some(archetype),
                ..default()
            },
        }
    }
}

impl TryFrom<LeftPanelEvent> for PlayableRace {
    type Error = &'static str;
    fn try_from(value: LeftPanelEvent) -> Result<Self, Self::Error> {
        if let Some(race) = value.race {
            return Ok(race);
        }
        Err("Invalid LeftPanelEvent passed as parameter to TryFrom<LeftPanelEvent> for PlayableRace")
    }
}
impl TryFrom<LeftPanelEvent> for PlayableClass {
    type Error = &'static str;
    fn try_from(value: LeftPanelEvent) -> Result<Self, Self::Error> {
        if let Some(class) = value.class {
            return Ok(class);
        }
        Err("Invalid LeftPanelEvent passed as parameter to TryFrom<LeftPanelEvent> for PlayableClass")
    }
}
impl TryFrom<LeftPanelEvent> for ArchetypeName {
    type Error = &'static str;
    fn try_from(value: LeftPanelEvent) -> Result<Self, Self::Error> {
        if let Some(archetype) = value.archetype {
            return Ok(archetype);
        }
        Err("Invalid LeftPanelEvent passed as parameter to TryFrom<LeftPanelEvent> for ArchetypeName")
    }
}

#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Panel {
    pub tab: Tab,
    pub subtab: Option<SubTab>,
    pub excluded_subtab: Option<SubTab>,
}

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
    pub fn get_archetype(&self) -> Option<ArchetypeName> {
        match self {
            Self::Archetype(playable_archetype) => Some(*playable_archetype),
            _ => None,
        }
    }
}
impl std::fmt::Display for LeftPanelEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Race(race) => write!(f, "{}", race),
            Self::Class(class) => write!(f, "{}", class),
            Self::Archetype(archetype) => write!(f, "{}", archetype),
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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
pub enum TabListParent {
    Race,
    Class,
    Archetype,
}

impl std::fmt::Display for TabListParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Race => write!(f, "Race"),
            Self::Class => write!(f, "Class"),
            Self::Archetype => write!(f, "Archetype"),
        }
    }
}

impl Into<Tab> for TabListParent {
    fn into(self) -> Tab {
        match self {
            Self::Race => Tab::Race,
            Self::Class => Tab::Class,
            Self::Archetype => Tab::Archetype,
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
    pub fn set(&mut self, other: PlayableClass) -> &mut Self {
        self.0 = other;
        self
    }
}
impl SelectedWrapper<PlayableClass> for SelectedClass {
    fn selected(&self) -> PlayableClass {
        self.0
    }
}
impl From<PlayableClass> for SelectedClass {
    fn from(value: PlayableClass) -> Self {
        Self(value)
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
    pub fn set(&mut self, other: ArchetypeName) -> &mut Self {
        self.0 = other;
        self
    }
}
impl SelectedWrapper<ArchetypeName> for SelectedArchetype {
    fn selected(&self) -> ArchetypeName {
        self.0
    }
}

#[derive(Resource, Copy, Clone, Debug, Default)]
/// Stores the currently selected race
pub struct SelectedRace(pub PlayableRace);

// used in character_creation generics
impl SelectedWrapper<PlayableRace> for SelectedRace {
    fn selected(&self) -> PlayableRace {
        self.0
    }
}

impl From<PlayableRace> for SelectedRace {
    fn from(value: PlayableRace) -> Self {
        Self(value)
    }
}

impl SelectedRace {
    pub fn inner(&self) -> PlayableRace {
        self.0
    }
    pub fn set(&mut self, other: PlayableRace) {
        self.0 = other;
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
#[derive(Component, Clone, Debug)]
pub struct RightPanelContainer;

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
