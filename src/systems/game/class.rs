use crate::menu::character_creation::layout::generics::list_traits::{AsVec, HasArray};
use crate::systems::game::character::*;
use crate::systems::game::skills::SkillName;
use bevy::prelude::*;
use serde::Deserialize;
use std::fmt;

#[derive(Default, Deserialize, Clone, Component, Debug)]
pub struct FavoredClass {
    pub class: PlayableClass,
    pub race: PlayableRace,
    pub description: String,
    pub source: String,
}

#[derive(
    Component, Default, Deserialize, Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum PlayableClass {
    Alchemist,
    Arcanist,
    Barbarian,
    Bard,
    Bloodrager,
    Brawler,
    Cavalier,
    Cleric,
    Druid,
    Fighter,
    Gunslinger,
    Hunter,
    Inquisitor,
    Investigator,
    Kineticist,
    Magus,
    Medium,
    Mesmerist,
    Monk,
    Occultist,
    Oracle,
    Paladin,
    Psychic,
    Ranger,
    Rogue,
    Shaman,
    Skald,
    Slayer,
    Sorcerer,
    Spiritualist,
    Summoner,
    Swashbuckler,
    Vigilante,
    Warpriest,
    Witch,
    Wizard,
    #[default]
    None,
}

impl PlayableClass {
    pub fn iterator() -> impl Iterator<Item = PlayableClass> {
        [
            Self::Alchemist,
            Self::Arcanist,
            Self::Barbarian,
            Self::Bard,
            Self::Bloodrager,
            Self::Brawler,
            Self::Cavalier,
            Self::Cleric,
            Self::Druid,
            Self::Fighter,
            Self::Gunslinger,
            Self::Hunter,
            Self::Inquisitor,
            Self::Investigator,
            Self::Kineticist,
            Self::Magus,
            Self::Medium,
            Self::Mesmerist,
            Self::Monk,
            Self::Occultist,
            Self::Oracle,
            Self::Paladin,
            Self::Psychic,
            Self::Ranger,
            Self::Rogue,
            Self::Shaman,
            Self::Skald,
            Self::Slayer,
            Self::Sorcerer,
            Self::Spiritualist,
            Self::Summoner,
            Self::Swashbuckler,
            Self::Vigilante,
            Self::Warpriest,
            Self::Witch,
            Self::Wizard,
        ]
        .iter()
        .copied()
    }
    pub fn array() -> [PlayableClass; 36] {
        [
            Self::Alchemist,
            Self::Arcanist,
            Self::Barbarian,
            Self::Bard,
            Self::Bloodrager,
            Self::Brawler,
            Self::Cavalier,
            Self::Cleric,
            Self::Druid,
            Self::Fighter,
            Self::Gunslinger,
            Self::Hunter,
            Self::Inquisitor,
            Self::Investigator,
            Self::Kineticist,
            Self::Magus,
            Self::Medium,
            Self::Mesmerist,
            Self::Monk,
            Self::Occultist,
            Self::Oracle,
            Self::Paladin,
            Self::Psychic,
            Self::Ranger,
            Self::Rogue,
            Self::Shaman,
            Self::Skald,
            Self::Slayer,
            Self::Sorcerer,
            Self::Spiritualist,
            Self::Summoner,
            Self::Swashbuckler,
            Self::Vigilante,
            Self::Warpriest,
            Self::Witch,
            Self::Wizard,
        ]
    }
}

impl HasArray<PLAYABLE_CLASSES_LEN> for PlayableClass {
    fn has_array() -> [PlayableClass; PLAYABLE_CLASSES_LEN] {
        PlayableClass::array()
    }
}

impl AsVec for PlayableClass {
    fn vec() -> Vec<Self> {
        Vec::from(PlayableClass::array())
    }
}

impl fmt::Display for PlayableClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Alchemist => write!(f, "Alchemist"),
            Self::Arcanist => write!(f, "Arcanist"),
            Self::Barbarian => write!(f, "Barbarian"),
            Self::Bard => write!(f, "Bard"),
            Self::Bloodrager => write!(f, "Bloodrager"),
            Self::Brawler => write!(f, "Brawler"),
            Self::Cavalier => write!(f, "Cavalier"),
            Self::Cleric => write!(f, "Cleric"),
            Self::Druid => write!(f, "Druid"),
            Self::Fighter => write!(f, "Fighter"),
            Self::Gunslinger => write!(f, "Gunslinger"),
            Self::Hunter => write!(f, "Hunter"),
            Self::Inquisitor => write!(f, "Inquisitor"),
            Self::Investigator => write!(f, "Investigator"),
            Self::Kineticist => write!(f, "Kineticist"),
            Self::Magus => write!(f, "Magus"),
            Self::Medium => write!(f, "Medium"),
            Self::Mesmerist => write!(f, "Mesmerist"),
            Self::Monk => write!(f, "Monk"),
            Self::Occultist => write!(f, "Occultist"),
            Self::Oracle => write!(f, "Oracle"),
            Self::Paladin => write!(f, "Paladin"),
            Self::Psychic => write!(f, "Psychic"),
            Self::Ranger => write!(f, "Ranger"),
            Self::Rogue => write!(f, "Rogue"),
            Self::Shaman => write!(f, "Shaman"),
            Self::Skald => write!(f, "Skald"),
            Self::Slayer => write!(f, "Slayer"),
            Self::Sorcerer => write!(f, "Sorcerer"),
            Self::Spiritualist => write!(f, "Spiritualist"),
            Self::Summoner => write!(f, "Summoner"),
            Self::Swashbuckler => write!(f, "Swashbuckler"),
            Self::Vigilante => write!(f, "Vigilante"),
            Self::Warpriest => write!(f, "Warpriest"),
            Self::Witch => write!(f, "Witch"),
            Self::Wizard => write!(f, "Wizard"),
            Self::None => write!(f, "None"),
        }
    }
}

#[derive(Default, Deserialize, Clone, Debug)]
pub enum PlayableClassDetails {
    // Alchemist(AlchemistClass),
    // Arcanist(ArcanistClass),
    // Barbarian(BarbarianClass),
    // Bard(BardClass),
    // Bloodrager(BloodragerClass),
    // Brawler(BrawlerClass),
    // Cavalier(CavalierClass),
    // Cleric(ClericClass),
    // Druid(DruidClass),
    // Fighter(FighterClass),
    // Gunslinger(GunslingerClass),
    // Hunter(HunterClass),
    // Inquisitor(InquisitorClass),
    // Investigator(InvestigatorClass),
    // Kineticist(KineticistClass),
    // Magus(MagusClass),
    // Medium(MediumClass),
    // Mesmerist(MesmeristClass),
    // Monk(MonkClass),
    // Occultist(OccultistClass),
    // Oracle(OracleClass),
    // Paladin(PaladinClass),
    // Psychic(PsychicClass),
    // Ranger(RangerClass),
    // Rogue(RogueClass),
    // Shaman(ShamanClass),
    // Skald(SkaldClass),
    // Slayer(SlayerClass),
    // Sorcerer(SorcererClass),
    // Spiritualist(SpiritualistClass),
    // Summoner(SummonerClass),
    // Swashbuckler(SwashbucklerClass),
    // Vigilante(VigilanteClass),
    // Warpriest(WarpriestClass),
    // Witch(WitchClass),
    // Wizard(WizardClass),
    #[default]
    None,
}

use std::collections::HashMap;
#[derive(Resource, Default, Deserialize, Clone, Debug)]
pub struct ClassMap(pub HashMap<PlayableClass, ClassInfo>);
impl ClassMap {
    pub fn inner_ref(&self) -> &HashMap<PlayableClass, ClassInfo> {
        &self.0
    }

    pub fn inner_ref_mut(&mut self) -> &mut HashMap<PlayableClass, ClassInfo> {
        &mut self.0
    }

    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Resource, Default, Deserialize, Clone, Debug, Hash)]
pub struct ClassInfo {
    pub name_str: String,
    pub description: String,
    pub class_name: PlayableClass,
    pub class_skills: Vec<SkillName>,
    pub skill_ranks_per_level: usize,
    pub hit_die: Dice,
    pub alignment_restriction: Option<Vec<Alignment>>,
    pub bab_progression: BABProgression,
    // The name of each primary saving throw
    pub save_progression: [SavingThrowName; 3],
    pub class_features: Vec<Vec<ClassFeature>>,
    pub class_features_list: Vec<ClassFeature>,
}

impl ClassInfo {
    pub fn saving_throw_bonus(&self, level: usize) -> Vec<SavingThrowBonus> {
        use crate::systems::game::character::BonusType::*;
        use crate::systems::game::character::SavingThrowName::*;

        vec![
            SavingThrowBonus {
                bonus: self.saving_throw_at_level(&Fort, level) as i32,
                bonus_type: Untyped,
                saving_throw: Fort,
                limitation: LimitationEnum::None,
            },
            SavingThrowBonus {
                bonus: self.saving_throw_at_level(&Reflex, level) as i32,
                bonus_type: Untyped,
                saving_throw: Reflex,
                limitation: LimitationEnum::None,
            },
            SavingThrowBonus {
                bonus: self.saving_throw_at_level(&Will, level) as i32,
                bonus_type: Untyped,
                saving_throw: Will,
                limitation: LimitationEnum::None,
            },
        ]
    }
    pub fn saving_throw_at_level(&self, saving_throw: &SavingThrowName, level: usize) -> usize {
        let (base_value, save_fraction) = {
            if self.save_progression.contains(saving_throw) {
                (2, [1, 2])
            } else {
                (0, [1, 3])
            }
        };
        base_value + level * save_fraction[0] / save_fraction[1]
    }
}

#[derive(Copy, Default, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub enum BABProgression {
    Full,
    ThreeFourths,
    Half,
    #[default]
    None,
}

#[derive(Copy, Default, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct BaseAttack {
    pub base: usize,
}
impl BaseAttack {
    pub fn from_progression(progression: &BABProgression, level: usize) -> Self {
        match progression {
            BABProgression::Full => Self::from(level),
            BABProgression::ThreeFourths => Self::from(level * 3 / 4),
            BABProgression::Half => Self::from(level / 2),
            BABProgression::None => {
                panic!("invalid BABProgression::None passed to BaseAttack::from_progression")
            }
        }
    }
}
impl From<usize> for BaseAttack {
    fn from(value: usize) -> Self {
        Self { base: value }
    }
}
impl std::fmt::Display for BaseAttack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bab = self.base;
        let mut output = String::from(format!("+{bab}"));
        while let Some(_) = bab.checked_sub(6) {
            bab -= 5;
            output.push_str(format!("/+{bab}").as_str());
        }
        write!(f, "{}", output)
    }
}

#[derive(Component, Default, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Copy)]
pub enum ClassFeature {
    Fighter(FighterFeature),
    #[default]
    None,
}

#[derive(Default, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Copy)]
pub enum FighterFeature {
    BonusFeat,
    Bravery,
    WeaponTraining,
    ArmorTraining,
    ArmorMastery,
    WeaponMastery,
    //// Archetypes
    // Archer
    Hawkeye,
    TrickShot,
    ExpertArcher,
    SafeShot,
    EvasiveArcher,
    Volley,
    RangedDefense,
    WeaponMasteryArcher,
    #[default]
    None,
}

impl From<ClassFeature> for FighterFeature {
    fn from(value: ClassFeature) -> Self {
        match value {
            ClassFeature::Fighter(feature) => feature,
            ClassFeature::None => {
                panic!("invalid value passed to From<ClassFeature> for FighterFeature")
            }
        }
    }
}

pub trait IntoComponent<T: Component> {
    fn into_component(&self) -> T;
}

#[derive(Default, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    #[default]
    D20,
}

// Into FloatingBonusFeats
impl TryFrom<&ClassInfo> for FloatingBonusFeats {
    type Error = &'static str;

    fn try_from(other: &ClassInfo) -> Result<Self, Self::Error> {
        let mut vec: Vec<FloatingBonusFeat> = Vec::new();
        for features_per_level in &other.class_features {
            for feature in features_per_level {
                match feature {
                    ClassFeature::Fighter(fighter_feature) => {
                        match fighter_feature {
                            FighterFeature::BonusFeat => {
                                vec.push(FloatingBonusFeat {
                                    group: FloatingFeatGroup::Fighter,
                                    number: 1,
                                });
                                ()
                            }
                            _ => (),
                        };
                    }
                    _ => (),
                }
            }
        }
        if !vec.is_empty() {
            return Ok(FloatingBonusFeats::from(vec));
        }
        Err("Invalid value passed to TryFrom for FloatingBonusFeat")
    }
}

use crate::systems::game::magic::SpellCauses;

use super::constants::PLAYABLE_CLASSES_LEN;
impl TryFrom<&ClassInfo> for SavingThrowBonuses {
    type Error = &'static str;
    fn try_from(other: &ClassInfo) -> Result<Self, Self::Error> {
        let mut vec: Vec<SavingThrowBonus> = Vec::new();
        for features_per_level in &other.class_features {
            for feature in features_per_level {
                match feature {
                    ClassFeature::Fighter(fighter_feature) => match fighter_feature {
                        FighterFeature::Bravery => {
                            vec.push(SavingThrowBonus {
                                // change later [001]
                                bonus: 1,
                                bonus_type: BonusType::Untyped,
                                saving_throw: SavingThrowName::Will,
                                limitation: LimitationEnum::SpellCauses(SpellCauses::Fear),
                            })
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        if !vec.is_empty() {
            return Ok(SavingThrowBonuses::from(vec));
        }
        Err("Invalid value passed to TryFrom<Class> for SavingThrowBonuses")
    }
}

pub fn add_class_bonusfeats(
    mut query_character: Query<(Entity, &mut FloatingBonusFeats)>,
    selected: Res<SelectedCharacter>,
    class: Res<ClassInfo>,
    mut commands: Commands,
) {
    if let Ok(class_bonus_feats) = FloatingBonusFeats::try_from(class.as_ref()) {
        if let Some((_entity, mut floating_feats)) = query_character
            .iter_mut()
            .filter(|(entity, _)| *entity == selected.inner())
            .next()
        {
            let mut class_clone = class_bonus_feats.inner().clone();
            floating_feats.ref_mut_inner().append(&mut class_clone);
        } else {
            if let Some(mut entity_commands) = commands.get_entity(selected.inner()) {
                entity_commands.insert(class_bonus_feats);
            }
        }
    }
}

pub fn add_class_savingthrowbonuses(
    mut query_character: Query<(Entity, &mut SavingThrowBonuses, &ClassLevels)>,
    selected: Res<SelectedCharacter>,
    class_map: Res<ClassMap>,
    mut commands: Commands,
) {
    for class in class_map.inner_ref().values() {
        if let Ok(class_bonuses) = SavingThrowBonuses::try_from(class) {
            if let Some((_entity, mut existing_bonuses, _class_levels)) = query_character
                .iter_mut()
                .filter(|(entity, _, _)| *entity == selected.inner())
                .next()
            {
                for (_, bonuses) in class_bonuses.0.iter() {
                    existing_bonuses.add_or_insert_all(bonuses.clone());
                }
            } else {
                if let Some(mut entity_commands) = commands.get_entity(selected.inner()) {
                    entity_commands.insert(class_bonuses);
                }
            }
        }
    }
}
