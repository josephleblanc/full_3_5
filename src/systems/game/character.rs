use crate::systems::game::equipment::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::systems::game::race::IntoHashMapVecBuilder;
use crate::systems::game::{magic, skills};
use crate::systems::game::{magic::*, skills::*};

////////////////////////////////////////////////////////
//// Things that should probably go somewhere else
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub enum PlayerState {
    Casting,
    /* more possible states here */
}

////////////////////////////////////////////////////////
#[derive(Component, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PlayerName(String);

// Description of the race. Mostly flavor text, with varying length and detail
// depending on the race, cribbed right from d20pfsrd (ty!) :)
// --> Move this?
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Description(String);

// SleepImmunity here works on "magical sleep effects", and includes spells and
// spell-like abilities, but not supernatoral or other similar effects.
// This label can be used in the queries of the relevant effects, as in
// Query<Entity, Without<SleepImmunity>>
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct SleepImmunity;

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct SpellLikeAbilities(pub Vec<SpellLikeAbility>);

impl From<Vec<SpellLikeAbility>> for SpellLikeAbilities {
    fn from(other: Vec<SpellLikeAbility>) -> Self {
        Self(other)
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct SpellDCBonuses(pub HashMap<BonusType, Vec<SpellDCBonus>>);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct SpellDCBonus {
    pub bonus: i32,
    pub bonus_type: BonusType,
    pub limitation: Vec<LimitationEnum>,
}

impl From<Vec<SpellDCBonus>> for SpellDCBonuses {
    fn from(other: Vec<SpellDCBonus>) -> Self {
        let mut map: HashMap<BonusType, Vec<SpellDCBonus>> = HashMap::new();
        for entry in other.into_iter() {
            map.entry(entry.bonus_type)
                .and_modify(|vec| vec.push(entry.clone()))
                .or_insert(vec![entry]);
        }
        Self(map)
    }
}

pub trait UsesBonusKey
where
    Self: std::hash::Hash + Eq,
{
    fn bonus_type(&self) -> BonusType;
}

pub trait BonusContainer<U>
where
    Self: Component + From<Vec<U>>,
    U: UsesBonusKey + IntoHashMapVecBuilder<Self> + Clone,
{
    fn get_hashmap(&mut self) -> &mut HashMap<BonusType, Vec<U>>;

    fn add_or_insert_all(&mut self, bonus_vec_to_add: Vec<U>) {
        use std::collections::hash_map::Entry::Vacant;
        let map = self.get_hashmap();
        for single_bonus in bonus_vec_to_add.into_iter() {
            if let Vacant(entry) = map.entry(single_bonus.bonus_type()) {
                entry.insert(vec![single_bonus]);
            } else {
                map //.get_hashmap()
                    .entry(single_bonus.bonus_type())
                    .and_modify(|vec| {
                        if !vec.as_slice().contains(&single_bonus) {
                            vec.push(single_bonus);
                        }
                    });
            }
        }
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct ArmorClassBonuses(pub HashMap<BonusType, Vec<ArmorClassBonus>>);
impl BonusContainer<ArmorClassBonus> for ArmorClassBonuses {
    fn get_hashmap(&mut self) -> &mut HashMap<BonusType, Vec<ArmorClassBonus>> {
        &mut self.0
    }
}

impl From<Vec<ArmorClassBonus>> for ArmorClassBonuses {
    fn from(other: Vec<ArmorClassBonus>) -> Self {
        let mut map: HashMap<BonusType, Vec<ArmorClassBonus>> = HashMap::new();
        for entry in other {
            map.entry(entry.bonus_type)
                .and_modify(|vec| vec.push(entry))
                .or_insert(vec![entry]);
        }
        Self(map)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub struct ArmorClassBonus {
    pub bonus: i32,
    pub bonus_type: BonusType,
    pub limitation: LimitationEnum,
}

impl UsesBonusKey for ArmorClassBonus {
    fn bonus_type(&self) -> BonusType {
        self.bonus_type
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct CharacterWeaponProficiency {
    pub simple: HashMap<WeaponName, bool>,
    pub martial: HashMap<WeaponName, bool>,
    pub exotic: HashMap<WeaponName, bool>,
}

impl CharacterWeaponProficiency {
    pub fn new() -> Self {
        let simple = WeaponName::array_simple()
            .into_iter()
            .map(|weapon| (weapon, false))
            .collect();
        let martial = WeaponName::array_martial()
            .into_iter()
            .map(|weapon| (weapon, false))
            .collect();
        let exotic = WeaponName::array_exotic()
            .into_iter()
            .map(|weapon| (weapon, false))
            .collect();
        Self {
            simple,
            martial,
            exotic,
        }
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct CasterLevelBonuses(pub HashMap<BonusType, Vec<CasterLevelBonus>>);
impl WrapsBonus<BonusType, CasterLevelBonus> for CasterLevelBonuses {}

#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Ord, Hash)]
pub struct CasterLevelBonus {
    pub bonus: i32,
    pub bonus_type: BonusType,
    pub limitation: CasterLevelUse,
}

pub trait Bonus<U>
where
    Self: std::hash::Hash,
    U: std::hash::Hash,
{
    fn key(&self) -> U;
}

impl Bonus<BonusType> for CasterLevelBonus {
    fn key(&self) -> BonusType {
        self.bonus_type
    }
}

pub trait WrapsBonus<U, T>
where
    Self: Component,
    T: std::hash::Hash + Clone + Copy + Bonus<U>,
    U: std::hash::Hash + Clone + Copy + Eq,
{
    fn from_builder(other: Vec<T>) -> HashMap<U, Vec<T>> {
        let mut map: HashMap<U, Vec<T>> = HashMap::new();
        for entry in other {
            map.entry(entry.key())
                .and_modify(|vec| vec.push(entry))
                .or_insert(vec![entry]);
        }
        map
    }
}

// Ability Score bonuses, from all sources.
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct AbilityScoreBonuses(pub HashMap<AbilityScore, Vec<AbilityScoreBonus>>);

impl From<Vec<AbilityScoreBonus>> for AbilityScoreBonuses {
    fn from(other: Vec<AbilityScoreBonus>) -> Self {
        let mut map: HashMap<AbilityScore, Vec<AbilityScoreBonus>> = HashMap::new();
        for entry in other {
            map.entry(entry.ability)
                .and_modify(|vec| vec.push(entry))
                .or_insert(vec![entry]);
        }
        Self(map)
    }
}

#[derive(Component, Clone, Debug, Eq, PartialEq, Copy, Hash)]
pub struct AbilityScoreBonus {
    pub ability: AbilityScore,
    pub bonus: i32,
    pub bonus_type: BonusType,
}

// used during character creation to apply a specific value bonus
// to a chosen ability score,
// e.x. base human ability score modifier
#[derive(Component, Clone, Debug, PartialEq)]
pub struct FloatingAbilityBonuses(pub Vec<FloatingAbilityBonus>);

#[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatingAbilityBonus {
    pub val: i32,
    pub choices: Vec<AbilityScore>,
    pub choices_num: usize,
}

impl FloatingAbilityBonuses {
    pub fn push(&mut self, other: FloatingAbilityBonus) {
        self.0.push(other);
    }
}

impl From<FloatingAbilityBonus> for FloatingAbilityBonuses {
    fn from(other: FloatingAbilityBonus) -> Self {
        FloatingAbilityBonuses(vec![other])
    }
}

// Used during character creation to apply a floating skill bonus to a chosen
// skill, for example, the gnome trait:
//   Obsessive: Gnomes receive a +2 racial bonus on a Craft or Profession of
//   their choice.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct FloatingSkillBonuses(pub Vec<FloatingSkillBonus>);

#[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatingSkillBonus {
    pub val: i32,
    pub choices: Vec<SkillName>,
}

impl FloatingSkillBonuses {
    pub fn push(&mut self, other: FloatingSkillBonus) {
        self.0.push(other)
    }
}

impl From<FloatingSkillBonus> for FloatingSkillBonuses {
    fn from(other: FloatingSkillBonus) -> Self {
        FloatingSkillBonuses(vec![other])
    }
}

// Bonus Skills each level and on character creation
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct BonusSkillPerLevel {
    pub bonus_size: usize,
}

// Wraps the SavingThrowBonus values, which are not Components, so they can
// be applied all together to come to a total, taking into account the
// bonus type.
#[derive(Component, Clone, Debug)]
pub struct SavingThrowBonuses(pub HashMap<SavingThrowName, Vec<SavingThrowBonus>>);

impl From<Vec<SavingThrowBonus>> for SavingThrowBonuses {
    fn from(other: Vec<SavingThrowBonus>) -> Self {
        let mut map: HashMap<SavingThrowName, Vec<SavingThrowBonus>> = HashMap::new();
        for entry in other {
            map.entry(entry.saving_throw)
                .and_modify(|vec| vec.push(entry))
                .or_insert(vec![entry]);
        }
        Self(map)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash)]
pub struct SavingThrowBonus {
    // Not a Component
    pub bonus: i32,
    pub bonus_type: BonusType,
    pub saving_throw: SavingThrowName,
    pub limitation: LimitationEnum,
}

// Wraps the SkillBonus values, which are not Components, so they can
// be applied all together to come to a total, taking into account the
// bonus type.
#[derive(Component, Debug)]
pub struct SkillBonuses(pub HashMap<SkillName, Vec<SkillBonus>>);

impl From<Vec<SkillBonus>> for SkillBonuses {
    fn from(other: Vec<SkillBonus>) -> Self {
        let mut map: HashMap<SkillName, Vec<SkillBonus>> = HashMap::new();
        for entry in other {
            map.entry(entry.skill_name)
                .and_modify(|vec| vec.push(entry))
                .or_insert(vec![entry]);
        }
        Self(map)
    }
}

pub trait Limitation {}
// pub trait Limitation: Sync + Send + 'static + std::fmt::Debug {}
// struct LimitBox {
//     boxed_limitation: Box<dyn Limitation>,
// }
//
// impl LimitBox {
//     fn new(trait_impl: impl Limitation + 'static) -> Self {
//         LimitBox {
//             boxed_limitation: Box::new(trait_impl),
//         }
//     }
// }

#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash, Ord)]
pub struct NoLimitation;

// #[derive(Debug)]
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub struct SkillBonus {
    // Not a Component
    pub bonus: i32,
    pub bonus_type: BonusType,
    pub skill_name: SkillName,
    pub limitation: LimitationEnum,
    // pub limitation: Box<dyn Limitation>,
}

////// Limitation
// Many racial traits include limitations on when they can be applied.
// This is my attempt to include that logic in the bonuses added by those
// racial traits.
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash)]
pub enum LimitationEnum {
    CasterLevelBonus(CasterLevelUse),
    Spellcraft(SpellcraftUses),
    AttacksByCreatureSubtype(CreatureSubtype),
    SpellSchool(magic::SpellSchool),
    PlayerState(PlayerState),
    AbilityScoreAbove(AbilityScore, u32),
    None,
}

// Bonus types that may be applied to any given stat. Most bonuses of the same
// type do not stack, and even those that do often do not stack if they come
// from the same source.
// --> Work on this more later, as it may need to change to deal with the
//     difference in source of an effect.
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy, Eq, Hash, Ord)]
pub enum BonusType {
    //// Notes here are for how they can affect things in spells, but are more
    //   or less guidelines for most other sources.
    // Ability scores, Saves
    Alchemical,
    // AC
    Armor,
    // Base Attack Bonus
    BaseAttackBonus,
    // Attacks, checks
    Circumstance,
    // Attacks, checks, saves
    Competence,
    // AC
    Deflection,
    // AC
    Dodge,
    // Ability scores, AC, attacks, damage, speed
    Enhancement,
    // Ability scores
    Inherent,
    // AC, attacks, checks, saves,
    Insight,
    // AC, attacks, checks, damage, saves
    Luck,
    // Attacks, checks, damage, saves, Str, Con, Dex
    Morale,
    // AC*
    NaturalArmor,
    // AC, checks, damage, DCs, saves
    Profane,
    // Lots, check later
    Racial,
    // Saves
    Resistance,
    // AC, checks, damage, DCs, saves
    Sacred,
    // AC
    Shield,
    // Ability scores, attacks, AC, Stealth checks, CMD checks, others
    Size,
    // ? All kinds
    Trait,
    // Always stack, unless from the same source
    Untyped,
}

impl BonusType {
    fn is_self_stackable(&self) -> bool {
        use BonusType::*;
        match self {
            Alchemical => true,
            Untyped => true,
            Armor => true,
            Circumstance => true,
            Dodge => true,

            BaseAttackBonus => false,
            Competence => false,
            Deflection => false,
            Enhancement => false,
            Inherent => false,
            Insight => false,
            Luck => false,
            Morale => false,
            NaturalArmor => false,
            Profane => false,
            Racial => false,
            Resistance => false,
            Sacred => false,
            Shield => false,
            Size => false,
            Trait => false,
        }
    }
}

// Bonus feats to be chosen by the player
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct FloatingBonusFeat {
    pub group: FloatingFeatGroup,
    pub number: usize,
}

#[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatingBonusFeats(pub Vec<FloatingBonusFeat>);

impl FloatingBonusFeats {
    pub fn push(&mut self, other: FloatingBonusFeat) {
        self.0.push(other);
    }
}

impl From<FloatingBonusFeat> for FloatingBonusFeats {
    fn from(other: FloatingBonusFeat) -> Self {
        FloatingBonusFeats(vec![other])
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy)]
pub enum FloatingFeatGroup {
    Any,
    Fighter,
    // more feat groups to go here, impl an Into<Vec<Feat>> once
    // feats are set up with the group of choices.
}

// Base languages learned automatically and the possible optinos available
// to characters with high intelligence, based on their race selection.
#[derive(Component, Clone, Debug, PartialEq, PartialOrd)]
pub struct BaseLanguages {
    pub given: Vec<Language>,
    pub choices: Vec<Language>,
}

// Normal speed over ground, as opposed to fly, burrow, and swim speed.
#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Copy)]
pub struct GroundSpeed(pub f32);

pub trait RemovesSelf
where
    Self: Component + Copy,
{
    fn remove_self_from(&self, player_entity: Entity, mut commands: Commands) {
        commands.entity(player_entity).remove::<Self>();
    }
}

pub trait CharacterModifier<T: Component> // where
//     Self: Component + IntoCharBuilder,
{
    // fn modify_character(
    //     &self,
    //     mut commands: Commands,
    //     character_id: Entity,
    //     character_traits: Vec<RacialTraitName>,
    // ) {
    //     if let Some(mut player) = commands.get_entity(character_id) {
    //         if let Some(contrary_traits) = self.contrary_traits() {
    //             contrary_traits
    //                 .iter()
    //                 .filter(|contrary| character_traits.contains(contrary))
    //                 .map(|contrary| contrary.remove_self_from(player));
    //             {
    //                 player.insert(*self);
    //             }
    //         } else {
    //             player.insert(*self);
    //         }
    //     }
    // }
}

// All languages, plus two meta enum members to indicate any with secrets and
// any without secrets.
// --> refactor this later, and impl something to just give back those secret
//      and non-secret as arrays
#[derive(Component, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
pub enum Language {
    Aboleth,
    Abyssal,
    Aklo,
    Aquan,
    Auran,
    Boggard,
    Celestial,
    Common,
    Cyclops,
    DarkFolk,
    Draconic,
    // DrowSignLanguage secret
    // Druidic secret
    Dwarven,
    Dziriak, // D'ziriak
    Elven,
    Giant,
    Gnoll,
    Gnome,
    Goblin,
    Grippli,
    Halfling,
    Ignan,
    Infernal,
    Necril,
    Orc,
    Protean,
    Rougarou,
    Sphinx,
    Sylvan,
    Tengu,
    Terran,
    Treant,
    Undercommon,
    Vegepygmy,
    // Represents any non-secret languages like Druidic
    AnyNotSecret,
    // Represents any language including secret ones like Druidic
    AnyWithSecret,
}

#[derive(Component, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Copy)]
pub enum SpeedType {
    Swim,
    Burrow,
    Fly,
    Ground,
}

//// Vision
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct NormalVision(pub bool);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct LowLightVision(pub f32);

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Darkvision(pub f32);

//// Ability Scores
#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct AbilityScores(HashMap<AbilityScore, i32>);

impl AbilityScores {
    pub fn new() -> Self {
        let hashmap: HashMap<AbilityScore, i32> = HashMap::new();
        AbilityScores(hashmap)
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AbilityScore {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl AbilityScore {
    pub fn as_array() -> [AbilityScore; 6] {
        [
            AbilityScore::Strength,
            AbilityScore::Dexterity,
            AbilityScore::Constitution,
            AbilityScore::Intelligence,
            AbilityScore::Wisdom,
            AbilityScore::Charisma,
        ]
    }
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum AbilityScoreModifier {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub struct CharacterSize {
    pub category: SizeCategory,
    pub size_type: SizeType,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum SizeCategory {
    Fine,
    Diminutive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

#[derive(Component, Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum SizeType {
    Tall,
    Long,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum PlayableRace {
    #[default]
    Human,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Aasimar,
    Catfolk,
    Dhampir,
    Drow,
    Fetchling,
    Goblin,
    Hobgoblin,
    Ifrit,
    Kobold,
    Orc,
    Oread,
    Ratfolk,
    Sylph,
    Tengu,
    Tiefling,
    Undine,
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum CreatureType {
    Aberration,
    Animal,
    Construct,
    Dragon,
    Fey,
    #[default]
    Humanoid,
    MagicalBeast,
    MonstrousHumanoid,
    Ooze,
    Outsider,
    Plant,
    Undead,
    Vermin,
}

#[derive(
    Component, Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize, PartialOrd, Eq, Hash,
)]
pub enum CreatureSubtype {
    #[default]
    Adlet,
    Aeon,
    Agathion,
    Air,
    Angle,
    Aquatic,
    Archon,
    Asura,
    Augmented,
    Automaton,
    Azata,
    Behemoth,
    Catfolk,
    Chaotic,
    Clockwork,
    Cold,
    Colossus,
    Daemon,
    DarkFolk,
    DeepOne,
    Demodand,
    Demon,
    Devil,
    Div,
    Dwarf,
    Earth,
    Elemental,
    Elf,
    Evil,
    Extraplanar,
    Fire,
    Giant,
    Gnome,
    Goblinoid,
    Godspawn,
    Good,
    GreatOldOne,
    Halfling,
    Herald,
    Hive,
    Human,
    Incorporeal,
    Inevitable,
    Kaiju,
    Kami,
    Kasatha,
    Kitsune,
    Kyton,
    Lawful,
    Leshy,
    Mortic,
    Mythic,
    Native,
    Nightshade,
    Oni,
    Orc,
    Protean,
    Psychopomp,
    Qlippoth,
    Rakshasa,
    Ratfolk,
    Reptilian,
    Robot,
    Samsaran,
    Sasquatch,
    Shapechangeer,
    Swarm,
    Troop,
    Udaeus,
    Unbreathing,
    Vanara,
    Vishkanya,
    Water,
    Wayang,
    WildHunt,
}

///////////////////////////////////////////////////////////////////////////////
//// Impls

//// iterator impls
impl PlayableRace {
    pub fn iterator() -> impl Iterator<Item = PlayableRace> {
        [
            Self::Human,
            Self::Elf,
            Self::Gnome,
            Self::HalfElf,
            Self::HalfOrc,
            Self::Halfling,
            Self::Aasimar,
            Self::Catfolk,
            Self::Dhampir,
            Self::Drow,
            Self::Fetchling,
            Self::Goblin,
            Self::Hobgoblin,
            Self::Ifrit,
            Self::Kobold,
            Self::Orc,
            Self::Oread,
            Self::Ratfolk,
            Self::Sylph,
            Self::Tengu,
            Self::Tiefling,
            Self::Undine,
        ]
        .iter()
        .copied()
    }
}

//// Display impls
impl fmt::Display for PlayableRace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Human => write!(f, "Human"),
            Self::Elf => write!(f, "Elf"),
            Self::Gnome => write!(f, "Gnome"),
            Self::HalfElf => write!(f, "Half-Elf"),
            Self::HalfOrc => write!(f, "Half-Orc"),
            Self::Halfling => write!(f, "Halfling"),
            Self::Aasimar => write!(f, "Aasimar"),
            Self::Catfolk => write!(f, "Catfolk"),
            Self::Dhampir => write!(f, "Dhampir"),
            Self::Drow => write!(f, "Drow"),
            Self::Fetchling => write!(f, "Fetchling"),
            Self::Goblin => write!(f, "Goblin"),
            Self::Hobgoblin => write!(f, "Hobgoblin"),
            Self::Ifrit => write!(f, "Ifrit"),
            Self::Kobold => write!(f, "Kobold"),
            Self::Orc => write!(f, "Orc"),
            Self::Oread => write!(f, "Oread"),
            Self::Ratfolk => write!(f, "Ratfolk"),
            Self::Sylph => write!(f, "Sylph"),
            Self::Tengu => write!(f, "Tengu"),
            Self::Tiefling => write!(f, "Tiefling"),
            Self::Undine => write!(f, "Undine"),
        }
    }
}
// Size
impl fmt::Display for SizeCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Fine => write!(f, "Fine"),
            Self::Diminutive => write!(f, "Diminutive"),
            Self::Tiny => write!(f, "Tiny"),
            Self::Small => write!(f, "Small"),
            Self::Medium => write!(f, "Medium"),
            Self::Large => write!(f, "Large"),
            Self::Huge => write!(f, "Huge"),
            Self::Gargantuan => write!(f, "Gargantuan"),
            Self::Colossal => write!(f, "Colossal"),
        }
    }
}
// Creature Type and Subtype
impl CreatureType {
    pub fn iterator() -> impl Iterator<Item = CreatureType> {
        [
            Self::Aberration,
            Self::Animal,
            Self::Construct,
            Self::Dragon,
            Self::Fey,
            Self::Humanoid,
            Self::MagicalBeast,
            Self::MonstrousHumanoid,
            Self::Ooze,
            Self::Outsider,
            Self::Plant,
            Self::Undead,
            Self::Vermin,
        ]
        .iter()
        .copied()
    }
}

impl fmt::Display for CreatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Aberration => write!(f, "Aberration"),
            Self::Animal => write!(f, "Animal"),
            Self::Construct => write!(f, "Construct"),
            Self::Dragon => write!(f, "Dragon"),
            Self::Fey => write!(f, "Fey"),
            Self::Humanoid => write!(f, "Humanoid"),
            Self::MagicalBeast => write!(f, "Magical beast"),
            Self::MonstrousHumanoid => write!(f, "Monstrous humanoid"),
            Self::Ooze => write!(f, "Ooze"),
            Self::Outsider => write!(f, "Outsider"),
            Self::Plant => write!(f, "Plant"),
            Self::Undead => write!(f, "Undead"),
            Self::Vermin => write!(f, "Vermin"),
        }
    }
}

impl CreatureSubtype {
    pub fn iterator() -> impl Iterator<Item = CreatureSubtype> {
        [
            Self::Adlet,
            Self::Aeon,
            Self::Agathion,
            Self::Air,
            Self::Angle,
            Self::Aquatic,
            Self::Archon,
            Self::Asura,
            Self::Augmented,
            Self::Automaton,
            Self::Azata,
            Self::Behemoth,
            Self::Catfolk,
            Self::Chaotic,
            Self::Clockwork,
            Self::Cold,
            Self::Colossus,
            Self::Daemon,
            Self::DarkFolk,
            Self::DeepOne,
            Self::Demodand,
            Self::Demon,
            Self::Devil,
            Self::Div,
            Self::Dwarf,
            Self::Earth,
            Self::Elemental,
            Self::Elf,
            Self::Evil,
            Self::Extraplanar,
            Self::Fire,
            Self::Giant,
            Self::Gnome,
            Self::Goblinoid,
            Self::Godspawn,
            Self::Good,
            Self::GreatOldOne,
            Self::Halfling,
            Self::Herald,
            Self::Hive,
            Self::Human,
            Self::Incorporeal,
            Self::Inevitable,
            Self::Kaiju,
            Self::Kami,
            Self::Kasatha,
            Self::Kitsune,
            Self::Kyton,
            Self::Lawful,
            Self::Leshy,
            Self::Mortic,
            Self::Mythic,
            Self::Native,
            Self::Nightshade,
            Self::Oni,
            Self::Orc,
            Self::Protean,
            Self::Psychopomp,
            Self::Qlippoth,
            Self::Rakshasa,
            Self::Ratfolk,
            Self::Reptilian,
            Self::Robot,
            Self::Samsaran,
            Self::Sasquatch,
            Self::Shapechangeer,
            Self::Swarm,
            Self::Troop,
            Self::Udaeus,
            Self::Unbreathing,
            Self::Vanara,
            Self::Vishkanya,
            Self::Water,
            Self::Wayang,
            Self::WildHunt,
        ]
        .iter()
        .copied()
    }
}
impl fmt::Display for CreatureSubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Adlet => write!(f, "Adlet"),
            Self::Aeon => write!(f, "Aeon"),
            Self::Agathion => write!(f, "Agathion"),
            Self::Air => write!(f, "Air"),
            Self::Angle => write!(f, "Angle"),
            Self::Aquatic => write!(f, "Aquatic"),
            Self::Archon => write!(f, "Archon"),
            Self::Asura => write!(f, "Asura"),
            Self::Augmented => write!(f, "Augmented"),
            Self::Automaton => write!(f, "Automaton"),
            Self::Azata => write!(f, "Azata"),
            Self::Behemoth => write!(f, "Behemoth"),
            Self::Catfolk => write!(f, "Catfolk"),
            Self::Chaotic => write!(f, "Chaotic"),
            Self::Clockwork => write!(f, "Clockwork"),
            Self::Cold => write!(f, "Cold"),
            Self::Colossus => write!(f, "Colossus"),
            Self::Daemon => write!(f, "Daemon"),
            Self::DarkFolk => write!(f, "Dark Folk"),
            Self::DeepOne => write!(f, "Deep One"),
            Self::Demodand => write!(f, "Demodand"),
            Self::Demon => write!(f, "Demon"),
            Self::Devil => write!(f, "Devil"),
            Self::Div => write!(f, "Div"),
            Self::Dwarf => write!(f, "Dwarf"),
            Self::Earth => write!(f, "Earth"),
            Self::Elemental => write!(f, "Elemental"),
            Self::Elf => write!(f, "Elf"),
            Self::Evil => write!(f, "Evil"),
            Self::Extraplanar => write!(f, "Extraplanar"),
            Self::Fire => write!(f, "Fire"),
            Self::Giant => write!(f, "Giant"),
            Self::Gnome => write!(f, "Gnome"),
            Self::Goblinoid => write!(f, "Goblinoid"),
            Self::Godspawn => write!(f, "Godspawn"),
            Self::Good => write!(f, "Good"),
            Self::GreatOldOne => write!(f, "Great Old One"),
            Self::Halfling => write!(f, "Halfling"),
            Self::Herald => write!(f, "Herald"),
            Self::Hive => write!(f, "Hive"),
            Self::Human => write!(f, "Human"),
            Self::Incorporeal => write!(f, "Incorporeal"),
            Self::Inevitable => write!(f, "Inevitable"),
            Self::Kaiju => write!(f, "Kaiju"),
            Self::Kami => write!(f, "Kami"),
            Self::Kasatha => write!(f, "Kasatha"),
            Self::Kitsune => write!(f, "Kitsune"),
            Self::Kyton => write!(f, "Kyton"),
            Self::Lawful => write!(f, "Lawful"),
            Self::Leshy => write!(f, "Leshy"),
            Self::Mortic => write!(f, "Mortic"),
            Self::Mythic => write!(f, "Mythic"),
            Self::Native => write!(f, "Native"),
            Self::Nightshade => write!(f, "Nightshade"),
            Self::Oni => write!(f, "Oni"),
            Self::Orc => write!(f, "Orc"),
            Self::Protean => write!(f, "Protean"),
            Self::Psychopomp => write!(f, "Psychopomp"),
            Self::Qlippoth => write!(f, "Qlippoth"),
            Self::Rakshasa => write!(f, "Rakshasa"),
            Self::Ratfolk => write!(f, "Ratfolk"),
            Self::Reptilian => write!(f, "Reptilian"),
            Self::Robot => write!(f, "Robot"),
            Self::Samsaran => write!(f, "Samsaran"),
            Self::Sasquatch => write!(f, "Sasquatch"),
            Self::Shapechangeer => write!(f, "Shapechangeer"),
            Self::Swarm => write!(f, "Swarm"),
            Self::Troop => write!(f, "Troop"),
            Self::Udaeus => write!(f, "Udaeus"),
            Self::Unbreathing => write!(f, "Unbreathing"),
            Self::Vanara => write!(f, "Vanara"),
            Self::Vishkanya => write!(f, "Vishkanya"),
            Self::Water => write!(f, "Water"),
            Self::Wayang => write!(f, "Wayang"),
            Self::WildHunt => write!(f, "WildHunt"),
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq)]
pub enum SavingThrowName {
    Fort,
    Reflex,
    Will,
}

use std::fmt::{Display, Formatter};
impl Display for SavingThrowName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use SavingThrowName::*;
        match self {
            Fort => write!(f, "Fort"),
            Reflex => write!(f, "Reflex"),
            Will => write!(f, "Will"),
        }
    }
}

impl SavingThrowName {
    pub fn iterator(&self) -> impl Iterator<Item = SavingThrowName> {
        use SavingThrowName::*;
        [Fort, Reflex, Will].iter().copied()
    }
}
