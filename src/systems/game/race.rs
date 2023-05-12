#![allow(clippy::uninlined_format_args)]

use crate::systems::game::character::*;
use crate::systems::game::equipment::*;
use crate::systems::game::magic::*;
use crate::systems::game::skills::*;
use bevy::prelude::*;
use std::collections::HashMap;

////// Things that need to be revisited when other systems are better understood:
//  - FloatingSkillBonus: BaseGnomeObsessive: This trait adds +2 to a choice
//      of Crafting or Profession. Not sure how those skills will work yet.
//      For now I'm just leaving the just leaving them as Skill and Craft,
//      which won't be very useful in character creation.
//  - Gnome trait: BaseGnomeIllusionResistance: Currently has the limitation
//      SpellSchool::Illusion. This may not be enough to cover "illusion spells
//      and effects.
//      [0001]

#[derive(Resource, Clone, Debug, PartialEq, Hash, Eq)]
pub struct RaceBuilder(pub Vec<RacialTraitName>);

#[derive(Clone, Debug, PartialEq, Hash, Eq, Copy)]
pub enum RacialTraitName {
    //// Traits Common to many races
    //// Size
    SizeMedium,
    SizeSmall,
    //// Vision
    NormalVision,
    LowLightVision,
    DarkVision,
    DarkVisionSuperior,
    //// Speed
    SpeedNormal,
    SpeedSlow,
    //// Type
    Humanoid,
    Outsider,
    //// Subtype
    Human,
    Elf,
    Gnome,
    Native,
    Goblinoid,
    Reptilian,
    //// Skills
    //// Weapon Familiarity
    BaseElfWeaponFamiliarity,
    OrcWeaponFamiliarity,
    //// Languages
    BaseLanguagesCommonAny,
    //// Immunities and Resistances
    ElvenImmunities,
    //// Others
    KeenSenses,
    ChooseOneASM,
    // LightSensitivity, // Dhampir, Kobold
    // LanguagesCommon,  // Only Common, no extras
    // Dwarf Base Traits
    BaseDwarfASM,
    Dwarf,
    BaseDwarfLanguages,
    BaseDwarfHardy,
    BaseDwarfStability,
    BaseDwarfGreed,
    BaseDwarfStonecunning,
    BaseDwarfWeaponFamiliarity,
    // Human Base Traits
    BaseHumanBonusFeat,
    BaseHumanSkilled,
    //// Elf Base Traits
    BaseElfASM,
    BaseElfElvenMagic,
    BaseElfLanguages,
    // BaseElfType,
    // BaseElfLanguages,
    // BaseElfElvenMagic,
    // BaseElfWeaponFamiliarity,
    // // Gnome Base Traits
    BaseGnomeASM,
    // BaseGnomeType,
    BaseGnomeLanguages,
    DefensiveTraining,
    BaseGnomeIllusionResistance,
    BaseGnomeObsessive,
    BaseGnomeGnomeMagic,
    BaseGnomeHatred,
    BaseGnomeWeaponFamiliarity,
    // // Half-Elf Base Traits
    BaseHalfElfLanguages,
    // BaseHalfElfAdaptability,
    // BaseHalfElfElfBlood,
    // BaseHalfElfMultitalented,
    // // Half-Orc Base Traits,
    // BaseHalfOrcType,
    // BaseHalfOrcLanguages,
    // BaseHalfOrcIntimidating,
    // BaseHalfOrcOrcFerocity,
    // BaseHalfOrcOrcBlood,
    // // Halfling Base Traits
    // BaseHalflingASB,
    // BaseHalflingType,
    // BaseHalflingLanguages,
    // BaseHalflingFearless,
    // BaseHalflingHalflingLuck,
    // BaseHalflingSureFooted,
    BaseHalflingWeaponFamiliarity,
    // // Aasimar Base Traits
    // BaseAasimarASB, // +2 Wis, +2 Cha
    // BaseAasimarLanguages,
    // BaseAasimarCelestialResistance,
    // BaseAasimarSkilled,
    // BaseAasimarSLA, // Spell-like Ability
    // // Catfolk Base Traits
    // BaseCatfolkASM,
    // BaseCatfolkType,
    // BaseCatfolkLanguages,
    // BaseCatfolkCatsLuck,
    // BaseCatfolkNaturalHunter,
    // BaseCatfolkSprinter,
    // // Dhampir Base Traits
    // BaseDhampirASM, // +2 Dex, +2 Cha, -2 Con
    Dhampir,
    // BaseDhampirUndeadResistance,
    // BaseDhampirResistLevelDrain,
    // BaseDhampirManipulative,
    // BaseDhampirSLA,
    // BaseDhampirNegativeEnergyAffinity,
    // // Sunlight Syndrome?
    // // https://www.d20pfsrd.com/RACES/other-races/featured-races/arg-dhampir/#Sunlight_Syndrome
    // //
    // // Drow Base Traits
    // BaseDrowASM, // +2 Dex, +2 Cha, -2 Con
    // BaseDrowLanguages,
    // BaseDrowImmunities,
    // BaseDrowSpellResistance,
    // BaseDrowSLA,
    // BaseDrowPoisonUse,
    // BaseDrowLightBlindness,
    BaseDrowWeaponFamiliarity,
    // // Fetchling Base Traits
    // BaseFetchlingASM, // +2 Dex, +2 Cha, -2 Wis
    // BaseFetchlingLanguages,
    // BaseFetchlingShadowBlending,
    // BaseFetchlingShadowyResistance,
    // BaseFetchlingSkilled,
    // BaseFetchlingSLA,
    // // Goblin Base Traits,
    // BaseGoblinASM, // +4 Dex, -2 Str, -2 Cha
    // BaseGoblinLanguages,
    // BaseGoblinSkilled,
    // BaseGoblinFastMovement,
    // // Hobgoblin Base Traits
    // BaseHobgoblinASM, // +2 Dex, +2 Con
    // BaseHobgoblinLanguages,
    HobgoblinSneaky,
    // // Ifrit Base Traits
    // BaseIfritASM, // +2 Dex, +2 Cha, -2 Wis
    // BaseIfritLanguages,
    // BaseIfritEnergyResistance, // fire 5
    // BaseIfritSLA,
    // BaseIfritFireAffinity,
    // // Kobold Base Traits
    // BaseKoboldASM, // +2 Dex, -4 Str, -2 Con
    // BaseKoboldLanguages,
    // BaseKoboldArmor,
    // BaseKoboldCrafty,
    // // Orc Base Traits
    // BaseOrcASM, // +4 Str, -2 Int, -2 Wis, -2 Cha
    Orc,
    // BaseOrcLanguages,
    // BaseOrcFerocity,
    // // Oread Base Traits
    // BaseOreadASM, // +2 Str, +2 Wis, -2 Cha
    // BaseOreadLanguages,
    // BaseOreadEnergyResistance, // 5 Earth
    // BaseOreadSLA,
    // BaseOreadEarthAffinity,
    // // Ratfolk Base Traits
    // BaseRatfolkASM, // +2 Dex, +2 Int, -2 Str
    Ratfolk,
    // BaseRatfolkLanguages,
    // BaseRatfolkTinker,
    // BaseRatfolkRodentEmpathy,
    // BaseRatfolkSwarming,
    // // Sylph Base Traits
    // BaseSylphASM, // +2 Dex, +2 Int, -2 Con
    // BaseSylphLanguages,
    // BaseSylphEnergyResistance,
    // BaseSylphSLA,
    // BaseSylphAirAffinity,
    // // Tengu Base Traits
    // BaseTenguASM, // +2 Dex, +2 Wis, -2 Con
    TenguSneaky,
    Tengu,
    // BaseTenguLanguages,
    // BaseTenguGiftedLinguist,
    BaseTenguSwordtrained,
    // BaseTenguNaturalWeapons,
    // BaseTenguSenses, // Same as low-light vision
    // // Tiefling Base Traits
    // BaseTieflingASM, // +2 Dex, +2 Int, -2 Cha
    // BaseTieflingLanguages,
    // BaseTieflingFiendishResistance,
    // BaseTieflingSkilled,
    // BaseTieflingSLA,
    // BaseTieflingFiendishSorcery,
    // // Undine Base Traits
    // BaseUndineASM, // +2 Dex, +2 Wis, -2 Str
    // BaseUndineLanguages,
    // BaseUndineEnergyResistance,
    // BaseUndineSLA,
    // BaseUndineWaterAffinity,
    AlchemicallyEnhanced,
    DualTalent,
}

impl RacialTraitName {
    pub fn contrary(&self) -> Option<Vec<RacialTraitName>> {
        use RacialTraitName::*;
        match self {
            // Human
            ChooseOneASM => Some(vec![AlchemicallyEnhanced, DualTalent]),
            _ => None,
        }
    }
    pub fn default_traits(race: &PlayableRace) -> Vec<RacialTraitName> {
        use RacialTraitName::*;
        match race {
            PlayableRace::Human => vec![
                Humanoid,
                Human,
                SizeMedium,
                NormalVision,
                SpeedNormal,
                ChooseOneASM,
                BaseLanguagesCommonAny,
                BaseHumanBonusFeat,
                BaseHumanSkilled,
            ],
            PlayableRace::Elf => vec![
                SizeMedium,
                NormalVision,
                SpeedNormal,
                LowLightVision,
                BaseElfASM,
                Humanoid,
                Elf,
                BaseElfLanguages,
                ElvenImmunities,
                KeenSenses,
                BaseElfElvenMagic,
                BaseElfWeaponFamiliarity,
            ],
            PlayableRace::Gnome => vec![
                BaseGnomeASM,
                Humanoid,
                Gnome,
                SizeSmall,
                SpeedSlow,
                BaseGnomeLanguages,
                DefensiveTraining,
                BaseGnomeIllusionResistance,
                KeenSenses,
                BaseGnomeObsessive,
                BaseGnomeGnomeMagic,
                BaseGnomeHatred,
                BaseGnomeWeaponFamiliarity,
                LowLightVision,
            ],
            PlayableRace::HalfElf => vec![
                ChooseOneASM,
                Humanoid,
                Elf,
                Human,
                SizeMedium,
                BaseHalfElfLanguages,
                ElvenImmunities,
                // BaseHalfElfAdaptability,
                KeenSenses,
                LowLightVision,
                // BaseHalfElfElfBlood,
                // BaseHalfElfMultitalented,
            ],

            PlayableRace::HalfOrc => vec![
                ChooseOneASM,
                //         BaseHalfOrcType,
                SizeMedium,
                //         BaseOrcLanguages,
                //         BaseHalfOrcIntimidating,
                //         BaseHalfOrcOrcFerocity,
                OrcWeaponFamiliarity,
                DarkVision,
                //         BaseHalfOrcOrcBlood,
            ],
            PlayableRace::Halfling => vec![
                //         BaseHalflingASB,
                SizeSmall,
                //         BaseHalflingType,
                SpeedSlow,
                //         BaseHalflingLanguages,
                //         BaseHalflingFearless,
                //         BaseHalflingHalflingLuck,
                //         BaseHalflingSureFooted,
                BaseHalflingWeaponFamiliarity,
                KeenSenses,
            ],
            PlayableRace::Aasimar => vec![
                //         BaseAasimarASB, // +2 Wis, +2 Cha
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseAasimarLanguages,
                //         BaseAasimarCelestialResistance,
                //         BaseAasimarSkilled,
                //         BaseAasimarSLA, // Spell-like Ability
                DarkVision,
            ],
            PlayableRace::Catfolk => vec![
                //         BaseCatfolkASM,
                //         BaseCatfolkType,
                SpeedNormal,
                SizeMedium,
                //         BaseCatfolkLanguages,
                //         BaseCatfolkCatsLuck,
                //         BaseCatfolkNaturalHunter,
                //         BaseCatfolkSprinter,
                LowLightVision,
            ],
            PlayableRace::Dhampir => vec![
                //         BaseDhampirASM, // +2 Dex, +2 Cha, -2 Con
                Humanoid,
                Dhampir,
                SizeMedium,
                SpeedNormal,
                //         LanguagesCommon,
                //         BaseDhampirUndeadResistance,
                //         BaseDhampirResistLevelDrain,
                //         BaseDhampirManipulative,
                //         BaseDhampirSLA,
                DarkVision,
                LowLightVision,
                //         LightSensitivity,
                //         BaseDhampirNegativeEnergyAffinity,
                //         // Sunlight Syndrome?
                //         // https://www.d20pfsrd.com/RACES/other-races/featured-races/arg-dhampir/#Sunlight_Syndrome
            ],
            PlayableRace::Drow => vec![
                //         BaseDrowASM, // +2 Dex, +2 Cha, -2 Con
                Humanoid,
                Elf,
                SizeMedium,
                SpeedNormal,
                BaseDrowWeaponFamiliarity,
                //         BaseDrowLanguages,
                //         BaseDrowImmunities,
                //         BaseDrowSpellResistance,
                KeenSenses,
                //         BaseDrowSLA,
                //         BaseDrowPoisonUse,
                DarkVisionSuperior,
                //         BaseDrowLightBlindness,
            ],
            PlayableRace::Fetchling => vec![
                //         BaseFetchlingASM, // +2 Dex, +2 Cha, -2 Wis
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseFetchlingLanguages,
                //         BaseFetchlingShadowBlending,
                //         BaseFetchlingShadowyResistance,
                //         BaseFetchlingSkilled,
                //         BaseFetchlingSLA,
                DarkVision,
                //         LowLightVision,
            ],
            PlayableRace::Goblin => vec![
                //         BaseGoblinASM, // +4 Dex, -2 Str, -2 Cha
                Humanoid, Goblinoid, SizeSmall, SpeedSlow,
                //         BaseGoblinLanguages,
                //         BaseGoblinSkilled,
                //         BaseGoblinFastMovement,
                DarkVision,
            ],
            PlayableRace::Hobgoblin => vec![
                //         BaseHobgoblinASM, // +2 Dex, +2 Con
                Humanoid,
                Goblinoid,
                SpeedNormal,
                SizeMedium,
                //         BaseHobgoblinLanguages,
                HobgoblinSneaky,
                DarkVision,
            ],
            PlayableRace::Ifrit => vec![
                //         BaseIfritASM, // +2 Dex, +2 Cha, -2 Wis
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseIfritLanguages,
                //         BaseIfritEnergyResistance, // fire 5
                //         BaseIfritSLA,
                //         BaseIfritFireAffinity,
                DarkVision,
            ],
            PlayableRace::Kobold => vec![
                //         BaseKoboldASM, // +2 Dex, -4 Str, -2 Con
                Humanoid,
                Reptilian,
                SizeSmall,
                SpeedNormal,
                //         BaseKoboldLanguages,
                //         BaseKoboldArmor,
                //         BaseKoboldCrafty,
                DarkVision,
                //         LightSensitivity,
            ],
            PlayableRace::Orc => vec![
                //         BaseOrcASM, // +4 Str, -2 Int, -2 Wis, -2 Cha
                Humanoid,
                Orc,
                SizeMedium,
                SpeedNormal,
                //         BaseOrcLanguages,
                //         BaseOrcFerocity,
                //         OrcWeaponFamiliarity,
                DarkVision,
                //         LightSensitivity,
            ],
            PlayableRace::Oread => vec![
                //         BaseOreadASM, // +2 Str, +2 Wis, -2 Cha
                Outsider, Native, SizeMedium, SpeedSlow,
                //         BaseOreadLanguages,
                //         BaseOreadEnergyResistance, // 5 Earth
                //         BaseOreadSLA,
                DarkVision,
                //         BaseOreadEarthAffinity,
            ],
            PlayableRace::Ratfolk => vec![
                //         BaseRatfolkASM, // +2 Dex, +2 Int, -2 Str
                Humanoid, Ratfolk, SizeSmall, SpeedSlow,
                //         BaseRatfolkLanguages,
                //         BaseRatfolkTinker,
                //         BaseRatfolkRodentEmpathy,
                //         BaseRatfolkSwarming,
                DarkVision,
            ],
            PlayableRace::Sylph => vec![
                //         BaseSylphASM, // +2 Dex, +2 Int, -2 Con
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseSylphLanguages,
                //         BaseSylphEnergyResistance,
                //         BaseSylphSLA,
                DarkVision,
                //         BaseSylphAirAffinity,
            ],
            PlayableRace::Tengu => vec![
                //         BaseTenguASM, // +2 Dex, +2 Wis, -2 Con
                Humanoid,
                Tengu,
                SizeMedium,
                SpeedNormal,
                //         BaseTenguLanguages,
                TenguSneaky,
                //         BaseTenguGiftedLinguist,
                BaseTenguSwordtrained,
                //         BaseTenguNaturalWeapons,
                //         BaseTenguSenses, // Same as low-light vision
            ],
            PlayableRace::Tiefling => vec![
                //         BaseTieflingASM, // +2 Dex, +2 Int, -2 Cha
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseTieflingLanguages,
                //         BaseTieflingFiendishResistance,
                //         BaseTieflingSkilled,
                //         BaseTieflingSLA,
                DarkVision,
                //         BaseTieflingFiendishSorcery,
            ],
            PlayableRace::Undine => vec![
                //         BaseUndineASM, // +2 Dex, +2 Wis, -2 Str
                Outsider,
                Native,
                SizeMedium,
                SpeedNormal,
                //         BaseUndineLanguages,
                //         BaseUndineEnergyResistance,
                //         BaseUndineSLA,
                //         BaseUndineWaterAffinity,
                DarkVision,
            ],
            // }
            //    }
            // }
            _ => vec![],
        }
    }
}
// BaseElfWeaponFamiliarity,
pub struct CharacterTraits(Vec<RacialTraitName>);
///////////////////////////////////////////////////////////////////////////////
//// This trait is used to transform a semi-unique RacialTraitName into Components
//     that are ready to be inserted into a player's Entity.
pub trait IntoComponentBuilder
where
    Self: Sized,
{
    fn from_name(name: &RacialTraitName) -> Result<Self, Box<dyn Error>>;
    // Missing implementations for alternate racial traits:
    // - AlchemicallyEnhanced
    // - DualTalent
}
//// This is a similar trait to IntoComponentBuilder, but for traits which cannot be well
//     represented by a single Component, but rather as a Component containing a
//     hashmap of the relevent racial trait changes. This is because they are
//     providing a bonus to a stat that will have multiple bonuses, and so as
//     Components they would replace previous bonuses. This way all racial trait
//     changes and bonuses can be added to a hash/dict Component with all the
//     bonuses, and then used by a system to compose the total bonus.
//     Examples:
//       - FloatingAbilityBonus / FloatingAbilityBonuses
//       - SavingThrowBonus / SavingThrowBonuses
//       - SkillBonus / Skill Bonuses
pub trait IntoHashMapBuilder
where
    Self: Sized,
{
    fn from_name(name: &RacialTraitName) -> Result<Self, Box<dyn Error>>;
}

pub trait IntoVecBuilder
where
    Self: Sized,
{
    fn from_name(name: &RacialTraitName) -> Result<Self, Box<dyn Error>>;
}

pub trait IntoHashMapVecBuilder<T>
where
    Self: Sized,
    T: From<Vec<Self>> + Component,
{
    fn from_name(name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>>;
    fn vec_to_component(vec: Vec<Self>) -> T {
        T::from(vec)
    }
    fn into_component(self) -> T {
        T::from(vec![self])
    }
}

pub trait FromVecBuilder<T: IntoVecBuilder>
where
    Self: Sized,
{
    fn into_component(vec_builder: T) -> Self;
}

#[derive(Copy, Component, Clone)]
pub struct CharacterBuilder;

///////////////////////////////////////////////////////////////////
pub fn spawn_player(mut commands: Commands) {
    commands.spawn(CharacterBuilder);
}
//

pub fn build_race(
    race: Res<RaceBuilder>,
    mut commands: Commands,
    query_builder: Query<Entity, With<CharacterBuilder>>,
    mut q_floating_bonus_feats: Query<&mut FloatingBonusFeats>,
    mut q_floating_ability_bonuses: Query<&mut FloatingAbilityBonuses>,
    mut q_skill_bonuses: Query<&mut SkillBonuses>,
    mut q_ability_score_bonuses: Query<&mut AbilityScoreBonuses>,
    mut q_saving_throw_bonuses: Query<&mut SavingThrowBonuses>,
    mut q_caster_level_bonuses: Query<&mut CasterLevelBonuses>,
    mut q_ac_bonuses: Query<&mut ArmorClassBonuses>,
    mut q_floating_skill_bonuses: Query<&mut FloatingSkillBonuses>,
    mut q_spell_like_abilities: Query<&mut SpellLikeAbilities>,
    mut q_spell_dc_bonuses: Query<&mut SpellDCBonuses>,
    mut q_attack_roll_bonuses: Query<&mut AttackRollBonuses>,
) {
    let builder_entity = query_builder.get_single().unwrap();
    let mut entity_commands = &mut commands.get_entity(builder_entity).unwrap();
    for racial_trait_name in race.0.iter() {
        //// RacialTraitNames with a corresponding IntoComponentBuilder impl
        // CreatureType
        // CreatureSubtype
        // CharacterSize
        // BaseLanguages
        // BonusSkillPerLevel
        // GroundSpeed
        // NormalVision
        // LowLightVision
        // DarkVision
        // CharacterWeaponProficiency
        if let Ok(creature_type) = CreatureType::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(creature_type);
        };
        if let Ok(creature_subtype) = CreatureSubtype::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(creature_subtype);
        };
        if let Ok(character_size) = CharacterSize::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(character_size);
        };
        if let Ok(base_languages) = BaseLanguages::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(base_languages);
        };
        if let Ok(bonus_skill_per_level) = BonusSkillPerLevel::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(bonus_skill_per_level);
        };
        if let Ok(ground_speed) = GroundSpeed::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(ground_speed);
        };
        if let Ok(normal_vision) = NormalVision::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(normal_vision);
        };
        if let Ok(low_light_vision) = LowLightVision::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(low_light_vision);
        };
        if let Ok(dark_vision) = DarkVision::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(dark_vision);
        };
        if let Ok(racial_proficiency) = CharacterWeaponProficiency::from_name(racial_trait_name) {
            entity_commands = entity_commands.insert(racial_proficiency);
        };

        //// RacialTraitNames with a corresponding IntoVecBuilder impl
        // FloatingBonusFeat
        // FloatingAbilityBonus
        // FloatingSkillBonus
        if let Ok(floating_bonus_feat) = FloatingBonusFeat::from_name(racial_trait_name) {
            if let Ok(mut floating_feats) = q_floating_bonus_feats.get_mut(builder_entity) {
                floating_feats.push(floating_bonus_feat);
            } else {
                entity_commands.insert(FloatingBonusFeats::from(floating_bonus_feat));
            }
        }
        if let Ok(floating_ability_bonus) = FloatingAbilityBonus::from_name(racial_trait_name) {
            if let Ok(mut floating_feats) = q_floating_ability_bonuses.get_mut(builder_entity) {
                floating_feats.push(floating_ability_bonus);
            } else {
                entity_commands.insert(FloatingAbilityBonuses::from(floating_ability_bonus));
            }
        }
        if let Ok(floating_skill_bonus) = FloatingSkillBonus::from_name(racial_trait_name) {
            if let Ok(mut floating_feats) = q_floating_skill_bonuses.get_mut(builder_entity) {
                floating_feats.push(floating_skill_bonus);
            } else {
                entity_commands.insert(FloatingSkillBonuses::from(floating_skill_bonus));
            }
        }
        //// RacialTraitNames with a corresponding IntoHashMapVecBuilder impl
        // SkillBonus
        // ArmorClassBonus
        // AbilityScoreBonus
        // SpellLikeAbility
        // SpellDCBonus
        // AttackRollBonuses
        // CasterLevelBonuses
        // SavingThrowBonuses
        if let Ok(skill_bonus) = SkillBonus::from_name(racial_trait_name) {
            if let Ok(mut skill_bonuses) = q_skill_bonuses.get_mut(builder_entity) {
                skill_bonuses.add_or_insert_all(skill_bonus);
            } else {
                entity_commands.insert(SkillBonus::vec_to_component(skill_bonus));
            }
        }
        if let Ok(ac_bonus) = ArmorClassBonus::from_name(racial_trait_name) {
            if let Ok(mut ac_bonuses) = q_ac_bonuses.get_mut(builder_entity) {
                ac_bonuses.add_or_insert_all(ac_bonus);
            } else {
                entity_commands.insert(ArmorClassBonus::vec_to_component(ac_bonus));
            }
        }
        if let Ok(ability_score_bonus) = AbilityScoreBonus::from_name(racial_trait_name) {
            if let Ok(mut ability_score_bonuses) = q_ability_score_bonuses.get_mut(builder_entity) {
                ability_score_bonuses.add_or_insert_all(ability_score_bonus)
            } else {
                entity_commands.insert(AbilityScoreBonuses::from(ability_score_bonus));
            }
        }
        if let Ok(new_bonuses) = SpellLikeAbility::from_name(racial_trait_name) {
            if let Ok(mut old_bonuses) = q_spell_like_abilities.get_mut(builder_entity) {
                old_bonuses.add_or_insert_all(new_bonuses)
            } else {
                entity_commands.insert(SpellLikeAbilities::from(new_bonuses));
            }
        }
        if let Ok(new_bonuses) = SpellDCBonus::from_name(racial_trait_name) {
            if let Ok(mut old_bonuses) = q_spell_dc_bonuses.get_mut(builder_entity) {
                old_bonuses.add_or_insert_all(new_bonuses)
            } else {
                entity_commands.insert(SpellDCBonuses::from(new_bonuses));
            }
        }
        if let Ok(new_bonuses) = AttackRollBonus::from_name(racial_trait_name) {
            if let Ok(mut old_bonuses) = q_attack_roll_bonuses.get_mut(builder_entity) {
                old_bonuses.add_or_insert_all(new_bonuses)
            } else {
                entity_commands.insert(AttackRollBonuses::from(new_bonuses));
            }
        }
        if let Ok(new_bonuses) = CasterLevelBonus::from_name(racial_trait_name) {
            if let Ok(mut old_bonuses) = q_caster_level_bonuses.get_mut(builder_entity) {
                old_bonuses.add_or_insert_all(new_bonuses)
            } else {
                entity_commands.insert(CasterLevelBonuses::from(new_bonuses));
            }
        }
        if let Ok(new_bonuses) = SavingThrowBonus::from_name(racial_trait_name) {
            if let Ok(mut old_bonuses) = q_saving_throw_bonuses.get_mut(builder_entity) {
                old_bonuses.add_or_insert_all(new_bonuses)
            } else {
                entity_commands.insert(SavingThrowBonuses::from(new_bonuses));
            }
        }
    }
}
pub fn print_builder(query_builder: Query<Entity, With<CharacterBuilder>>, mut commands: Commands) {
    let width = 80_usize;
    println!("{}", format!("{:-^width$}", "Components"));
    let builder = query_builder.get_single().unwrap();
    commands.get_entity(builder).unwrap().log_components();
}
pub fn print_skill_bonuses(query_skill_bonuses: Query<&SkillBonuses, With<CharacterBuilder>>) {
    if let Ok(skill_bonuses) = query_skill_bonuses.get_single() {
        let width = 80_usize;
        println!("{}", format!("{:-^width$}", "SkillBonuses"));
        for skill in skill_bonuses.0.iter() {
            println!("{:#?}", skill);
        }
    }
}
pub fn print_weapon_proficiencies(
    query_weapon_proficiency: Query<&CharacterWeaponProficiency, With<CharacterBuilder>>,
) {
    if let Ok(weapon_proficiency) = query_weapon_proficiency.get_single() {
        let mut width: usize = 80;
        println!("{}", format!("{:-^width$}", "Weapon Proficiency"));
        println!("------------> Simple proficiency");
        let mut not_prof_simple: usize = 0;
        let expected_simple = WeaponName::array_simple();
        let total_simple = expected_simple.len();
        for skill in weapon_proficiency.simple.iter() {
            if !skill.1 {
                not_prof_simple += 1;
            }
            if !expected_simple.contains(skill.0) {
                println!("\t{}: in simple", skill.0);
            }
        }
        println!("\tnot proficient: {not_prof_simple}, total: {total_simple}");

        println!("------------> Martial proficiency");
        let mut not_prof_martial: usize = 0;
        let expected_martial = WeaponName::array_martial();
        let total_martial = expected_martial.len();
        for skill in weapon_proficiency.martial.iter() {
            let name_length = skill.0.to_string().len();
            width = 40;
            if let Some(u32) = width.checked_sub(name_length) {
                width = width - name_length;
            }
            if !skill.1 {
                not_prof_martial += 1;
                if !expected_martial.contains(skill.0) {
                    println!(
                        "{}{}",
                        format!("In Martial: {}", skill.0),
                        format!("{:->width$}", skill.1)
                    );
                }
            } else if expected_martial.contains(skill.0) {
                println!(
                    "{}{}",
                    format!("In Martial: {}", skill.0),
                    format!("{:->width$}", skill.1)
                );
            }
        }
        println!("\tnot proficient: {not_prof_martial}, total: {total_martial}");

        println!("\n--> Exotic proficiency");
        let mut not_prof_exotic: usize = 0;
        let expected_exotic = WeaponName::array_exotic();
        let total_exotic = expected_exotic.len();
        for skill in weapon_proficiency.exotic.iter() {
            if !skill.1 {
                not_prof_exotic += 1;
            }
            if !expected_exotic.contains(skill.0) {
                println!("\t{}: in exotic", skill.0);
            }
        }
        println!("\tnot proficient: {not_prof_exotic}, total: {total_exotic}");
    }
}
pub fn print_floating_bonus_feats(q_floating_bonus_feats: Query<&FloatingBonusFeats>) {
    if let Ok(bonuses) = q_floating_bonus_feats.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Floating Feat Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_floating_ability_bonuses(query: Query<&FloatingAbilityBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Floating Ability Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_ability_score_bonuses(query: Query<&AbilityScoreBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Ability Score Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_saving_throw_bonuses(query: Query<&SavingThrowBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "SavingThrow Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_caster_level_bonuses(query: Query<&CasterLevelBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Caster Level Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_armor_class_bonuses(query: Query<&ArmorClassBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Armor Class Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_floating_skill_bonuses(query: Query<&FloatingSkillBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "Floating Skill Bonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_spell_like_abilities(query: Query<&SpellLikeAbilities>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "SpellLikeAbilities"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_spell_dc_bonuses(query: Query<&SpellDCBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "SpellDCBonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
pub fn print_attack_roll_bonuses(query: Query<&AttackRollBonuses>) {
    if let Ok(bonuses) = query.get_single() {
        let width: usize = 80;
        println!("{}", format!("{:-^width$}", "AttackRollBonuses"));
        for bonus in bonuses.0.iter() {
            println!("{:#?}", bonus);
        }
    }
}
//     mut q_caster_level_bonus: Query<&mut CasterLevelBonuses>,

//// The reciprocal trait to IntoComponentBuilder and IntoCharBuilderHash above,
//     changing a builder element back into a RacialTraitName.
//     Personal note: Not sure where to use this yet, but it is a good way to
//                    check that all RacialTraitName have been matched in
//                    IntoComponentBuilder and IntoCharBuilderHash.
use std::error::Error;

//// Intermediate Builder Structs
// Some traits cannot be well represented by a single `Component`, and must have
// a middle stage before being separated into the `Component`s that will be
// inserted to the player's Entity later on.
// This way IntoComponentBuilder can still be applied to the RacialTraitName,
// but note that this is not a `Component`, but is composed of `Component`s.
// pub struct ElvenImmunitiesBuilder {
//     pub immunity: SleepImmunity,
//     pub saving_throw_bonus: SavingThrowBonus,
// }
///////////////////////////////////////////////////////////////////////////////
////// IntoComponentBuilder
//// Weapon Familiarity
// BaseElfWeaponFamiliarity
impl IntoComponentBuilder for CharacterWeaponProficiency {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseElfWeaponFamiliarity => {
                let mut proficiency = CharacterWeaponProficiency::new();
                let mut elf_martial = RacialWeapon::Elf.exotic_to_martial_vec();
                for _ in 0..elf_martial.len() {
                    let weapon = elf_martial.pop()
                        .expect("Invalid WeaponName in RacialWeapon::Elf.exotic_to_martial_vec(), or empty Vec, passed to from_name() method of CharacterWeaponProficiency from trait IntoComponentBuilder");
                    proficiency.exotic.remove(&weapon);
                    proficiency.martial.insert(weapon, false);
                }
                for elf_proficient in RacialWeapon::Elf.racial_proficient_vec() {
                    proficiency.martial.insert(elf_proficient, true);
                }

                Ok(proficiency)
            }
            RacialTraitName::BaseGnomeWeaponFamiliarity => {
                let mut proficiency = CharacterWeaponProficiency::new();
                let mut race_martial = RacialWeapon::Gnome.exotic_to_martial_vec();
                for _ in 0..race_martial.len() {
                    let weapon = race_martial.pop()
                        .expect("Invalid WeaponName in RacialWeapon::Gnome.exotic_to_martial_vec(), or empty Vec, passed to from_name() method of CharacterWeaponProficiency from trait IntoComponentBuilder");
                    proficiency.exotic.remove(&weapon);
                    proficiency.martial.insert(weapon, false);
                }
                Ok(proficiency)
            }
            RacialTraitName::BaseDrowWeaponFamiliarity => {
                let mut proficiency = CharacterWeaponProficiency::new();
                for drow_proficient in RacialWeapon::Drow.racial_proficient_vec() {
                    proficiency.martial.insert(drow_proficient, true);
                }
                Ok(proficiency)
            }
            RacialTraitName::BaseTenguSwordtrained => {
                let mut proficiency = CharacterWeaponProficiency::new();
                for tengu_proficient in RacialWeapon::Tengu.racial_proficient_vec() {
                    proficiency.martial.insert(tengu_proficient, true);
                }
                Ok(proficiency)
            }
            RacialTraitName::BaseDwarfWeaponFamiliarity => {
                let mut proficiency = CharacterWeaponProficiency::new();
                let mut race_martial = RacialWeapon::Dwarf.exotic_to_martial_vec();
                for _ in 0..race_martial.len() {
                    let weapon = race_martial.pop()
                        .expect("Invalid WeaponName in RacialWeapon::Gnome.exotic_to_martial_vec(), or empty Vec, passed to from_name() method of CharacterWeaponProficiency from trait IntoComponentBuilder");
                    proficiency.exotic.remove(&weapon);
                    proficiency.martial.insert(weapon, false);
                }
                Ok(proficiency)
            }
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CharacterWeaponProficiency in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// Skills per level
// BaseHumanBonusFeat -> 1 bonus feat
impl IntoComponentBuilder for BonusSkillPerLevel {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseHumanSkilled => Ok(Self { bonus_size: 1 }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for BonusSkillPerLevel in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// Creature Type
// Humanoid -> Humanoid
// Outsider -> Outsider
impl IntoComponentBuilder for CreatureType {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::Humanoid => Ok(Self::Humanoid),
            RacialTraitName::Outsider => Ok(Self::Outsider),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CreatureType in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Creature Subtype
// Human        -> Human
// Elf          -> Elf
// ... etc.
impl IntoComponentBuilder for CreatureSubtype {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::Human => Ok(Self::Human),
            RacialTraitName::Elf => Ok(Self::Elf),
            RacialTraitName::Gnome => Ok(Self::Gnome),
            RacialTraitName::Dwarf => Ok(Self::Dwarf),
            RacialTraitName::Native => Ok(Self::Native),
            RacialTraitName::Goblinoid => Ok(Self::Goblinoid),
            RacialTraitName::Reptilian => Ok(Self::Reptilian),
            RacialTraitName::Orc => Ok(Self::Orc),
            RacialTraitName::Ratfolk => Ok(Self::Ratfolk),
            RacialTraitName::Tengu => Ok(Self::Tengu),
            RacialTraitName::Dhampir => Ok(Self::Dhampir),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CreatureSubtype in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Size
// SizeMedium -> Medium
impl IntoComponentBuilder for CharacterSize {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::SizeMedium => Ok(Self {
                category: SizeCategory::Medium,
                size_type: SizeType::Tall,
            }),
            RacialTraitName::SizeSmall => Ok(Self {
                category: SizeCategory::Small,
                size_type: SizeType::Tall,
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CharacterSize in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Languages
// BaseLanguagesCommonAny
// BaseElfLanguages
impl IntoComponentBuilder for BaseLanguages {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        use Language::*;
        match racial_trait_name {
            RacialTraitName::BaseLanguagesCommonAny => Ok(Self {
                given: vec![Common],
                choices: vec![AnyNotSecret],
            }),
            RacialTraitName::BaseElfLanguages => Ok(Self {
                given: vec![Common, Elven],
                choices: vec![Celestial, Draconic, Gnoll, Gnome, Goblin, Orc, Sylvan],
            }),
            RacialTraitName::BaseGnomeLanguages => Ok(Self {
                given: vec![Common, Gnome, Sylvan],
                choices: vec![Draconic, Dwarven, Elven, Giant, Goblin, Orc],
            }),
            RacialTraitName::BaseDwarfLanguages => Ok(Self {
                given: vec![Common, Dwarven],
                choices: vec![Giant, Gnome, Goblin, Terran, Undercommon],
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for BaseLanguages in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Speed
// SpeedNormal -> GroundSpeed
impl IntoComponentBuilder for GroundSpeed {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::SpeedNormal => Ok(Self(30.)),
            RacialTraitName::SpeedSlow => Ok(Self(20.)),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for GroundSpeed in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Vision
// NormalVision -> NormalVision(true)
impl IntoComponentBuilder for NormalVision {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::NormalVision => Ok(Self(true)),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for NormalVision in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

// DarkVision -> DarkVision(distance)
impl IntoComponentBuilder for DarkVision {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::DarkVision => Ok(Self(60.)),
            RacialTraitName::DarkVisionSuperior => Ok(Self(120.)),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for DarkVision in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
// LowLightVision -> LowLightVision(distance)
impl IntoComponentBuilder for LowLightVision {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::LowLightVision => Ok(Self(60.)),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for LowLightVision in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
////// IntoVecBuilder
//// Floating Feats
impl IntoVecBuilder for FloatingBonusFeat {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseHumanBonusFeat => Ok(Self {
                group: FloatingFeatGroup::Any,
                number: 1,
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for FloatingBonusFeat in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// Floating Skills
impl IntoVecBuilder for FloatingSkillBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        use SkillName::*;
        match racial_trait_name {
            RacialTraitName::BaseGnomeObsessive => Ok(Self {
                val: 2,
                // ---> Revisit this when Craft and Profession are better nailed down
                choices: vec![Craft, Profession],
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for FloatingBonusFeat in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// Floating Ability Score Bonus (ABS)
impl IntoVecBuilder for FloatingAbilityBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::ChooseOneASM => Ok(Self {
                val: 2,
                choices: AbilityScore::as_array().to_vec(),
                choices_num: 1,
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for FloatingAbilityBonus in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
////// IntoHashMapVecBuilder
//// SavingThrowBonuses
// Gnome Trait: Illusion Resistance
//      This gives +2 to all saves against illusion spells and effects, so it
//      adds +2 individually to each save using the `SpellSchool` limitation.
//      This could potentially lead to bugs if, as the trait describes,
//      "Illusion spells and effects" does not cover all the bases.
//      Potentially revisit once I start implementing spells/effects.
//      Code: [0001]
impl IntoHashMapVecBuilder<SavingThrowBonuses> for SavingThrowBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseGnomeIllusionResistance => Ok(vec![
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Will,
                    limitation: LimitationEnum::SpellSchool(SpellSchool::Illusion),
                },
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Reflex,
                    limitation: LimitationEnum::SpellSchool(SpellSchool::Illusion),
                },
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Fort,
                    limitation: LimitationEnum::SpellSchool(SpellSchool::Illusion),
                },
            ]),
            RacialTraitName::BaseDwarfHardy => Ok(vec![
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Will,
                    limitation: LimitationEnum::PoisonAndSpells(Poison {}, Magic {}),
                },
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Reflex,
                    limitation: LimitationEnum::PoisonAndSpells(Poison {}, Magic {}),
                },
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    saving_throw: SavingThrowName::Fort,
                    limitation: LimitationEnum::PoisonAndSpells(Poison {}, Magic {}),
                },
            ]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SavingThrowBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoHashMapVecBuilder<CMDBonuses> for CMDBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseDwarfStability => Ok(vec![Self {
                bonus: 4,
                key: BonusType::Racial,
                bonus_type: BonusType::Racial,
                limitation: vec![
                    LimitationEnum::CombatManeuverName(CombatManeuverName::BullRush),
                    LimitationEnum::CombatManeuverName(CombatManeuverName::Trip),
                ],
            }]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CMDBonus in from_name() \
                 method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoHashMapVecBuilder<SpellDCBonuses> for SpellDCBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseGnomeGnomeMagic => Ok(vec![Self {
                bonus: 2,
                key: BonusType::Racial,
                bonus_type: BonusType::Racial,
                limitation: vec![LimitationEnum::SpellSchool(SpellSchool::Illusion)],
            }]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SpellDCBonus in from_name() \
                 method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// One-time Skill Bonus
// SkillBonuses
impl IntoHashMapVecBuilder<SkillBonuses> for SkillBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        use SkillName::*;
        match racial_trait_name {
            RacialTraitName::KeenSenses => Ok(vec![Self {
                bonus: 2,
                bonus_type: BonusType::Racial,
                skill_name: Perception,
                limitation: vec![LimitationEnum::None],
            }]),
            RacialTraitName::BaseElfElvenMagic => Ok(vec![Self {
                bonus: 2,
                bonus_type: BonusType::Racial,
                skill_name: SkillName::Spellcraft,
                limitation: vec![LimitationEnum::Spellcraft(SpellcraftUses::IdentifyItem)],
            }]),
            RacialTraitName::BaseDwarfGreed => Ok(vec![Self {
                bonus: 2,
                bonus_type: BonusType::Racial,
                skill_name: SkillName::Appraise,
                limitation: vec![
                    LimitationEnum::ItemContains(ItemContains::PreciousMetal),
                    LimitationEnum::ItemContains(ItemContains::Gemstones),
                ],
            }]),
            RacialTraitName::HobgoblinSneaky => Ok(vec![Self {
                bonus: 4,
                bonus_type: BonusType::Racial,
                skill_name: Stealth,
                limitation: vec![LimitationEnum::None],
            }]),
            RacialTraitName::TenguSneaky => Ok(vec![
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    skill_name: Perception,
                    limitation: vec![LimitationEnum::None],
                },
                Self {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    skill_name: Stealth,
                    limitation: vec![LimitationEnum::None],
                },
            ]),

            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SkillBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
//// Specific Ability Score Bonus (ABS)
impl IntoHashMapVecBuilder<AbilityScoreBonuses> for AbilityScoreBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        use AbilityScore::*;
        use BonusType::*;
        match racial_trait_name {
            RacialTraitName::BaseElfASM => Ok(vec![
                Self {
                    ability: Dexterity,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Intelligence,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Constitution,
                    bonus: -2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
            ]),
            RacialTraitName::BaseGnomeASM => Ok(vec![
                Self {
                    ability: Constitution,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Charisma,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Strength,
                    bonus: -2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
            ]),
            RacialTraitName::BaseDwarfASM => Ok(vec![
                Self {
                    ability: Constitution,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Wisdom,
                    bonus: 2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
                Self {
                    ability: Charisma,
                    bonus: -2,
                    bonus_type: Racial,
                    limitation: LimitationEnum::None,
                },
            ]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for AbilityScoreBonus in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoHashMapVecBuilder<ArmorClassBonuses> for ArmorClassBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::DefensiveTraining => Ok(vec![Self {
                bonus: 4,
                bonus_type: BonusType::Racial,
                // need to add a limitation for vs attacks from giant
                limitation: LimitationEnum::AttacksByCreatureSubtype(CreatureSubtype::Giant),
            }]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SkillBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoHashMapVecBuilder<AttackRollBonuses> for AttackRollBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseGnomeHatred => Ok(vec![Self {
                bonus: 1,
                bonus_type: BonusType::Racial,
                key: BonusType::Racial,
                // need to add a limitation for vs attacks from giant
                limitation: vec![
                    LimitationEnum::AttackingTargetTypeAndSubtype(
                        CreatureType::Humanoid,
                        CreatureSubtype::Reptilian,
                    ),
                    LimitationEnum::AttackingTargetTypeAndSubtype(
                        CreatureType::Humanoid,
                        CreatureSubtype::Goblinoid,
                    ),
                ],
            }]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SkillBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

//// Special cases that turn into more than one Component
//   - ElvenImmunities
//   - ElvenMagic
// impl IntoComponentBuilder for ElvenImmunitiesBuilder {
//     fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
//         use SavingThrowName::*;
//         use SpellSchool::*;
//         match racial_trait_name {
//             RacialTraitName::ElvenImmunities => Ok(Self {
//                 saving_throw_bonus: SavingThrowBonus {
//                     saving_throw: Will,
//                     bonus: 2,
//                     bonus_type: BonusType::Racial,
//                     limitation: LimitationEnum::SpellSchool(Enchantment),
//                 },
//                 immunity: SleepImmunity,
//             }),
//             _ => Err(format!(
//                 "Invalid RacialTraitName: {:?} for ElvenImmunitiesBuilder in \
//                 from_name() method of trait IntoComponentBuilder",
//                 racial_trait_name
//             )
//             .into()),
//         }
//     }
// }

pub struct ElvenMagicBuilder {
    caster_level_bonus: CasterLevelBonus,
    skill_bonus: SkillBonus,
}
// Elven Magic, adds to both CasterLevelBonus and SkillBonus
impl IntoComponentBuilder for ElvenMagicBuilder {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseElfElvenMagic => Ok(Self {
                caster_level_bonus: CasterLevelBonus {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    limitation: CasterLevelUse::OvercomeSpellResistance,
                },
                skill_bonus: SkillBonus {
                    bonus: 2,
                    bonus_type: BonusType::Racial,
                    skill_name: SkillName::Spellcraft,
                    limitation: vec![LimitationEnum::Spellcraft(SpellcraftUses::IdentifyItem)],
                },
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for LowLightVision in \
                from_name() method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoHashMapVecBuilder<CasterLevelBonuses> for CasterLevelBonus {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        match racial_trait_name {
            RacialTraitName::BaseElfElvenMagic => Ok(vec![Self {
                bonus: 2,
                bonus_type: BonusType::Racial,
                limitation: CasterLevelUse::OvercomeSpellResistance,
            }]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for CasterLevelBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

impl IntoHashMapVecBuilder<SpellLikeAbilities> for SpellLikeAbility {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Vec<Self>, Box<dyn Error>> {
        use SpellName::*;
        match racial_trait_name {
            RacialTraitName::BaseGnomeGnomeMagic => Ok(vec![
                SpellLikeAbility {
                    source: SlaSource::GnomeMagic,
                    spell_name: DancingLights,
                    cast_frequency: CastFrequency::PerDay,
                    uses: Some(1),
                    ability_score: AbilityScore::Charisma,
                    limitation: Some(LimitationEnum::AbilityScoreAbove(
                        AbilityScore::Charisma,
                        10,
                    )),
                },
                SpellLikeAbility {
                    source: SlaSource::GnomeMagic,
                    spell_name: GhostSound,
                    cast_frequency: CastFrequency::PerDay,
                    uses: Some(1),
                    ability_score: AbilityScore::Charisma,
                    limitation: Some(LimitationEnum::AbilityScoreAbove(
                        AbilityScore::Charisma,
                        10,
                    )),
                },
                SpellLikeAbility {
                    source: SlaSource::GnomeMagic,
                    spell_name: Prestidigitation,
                    cast_frequency: CastFrequency::PerDay,
                    uses: Some(1),
                    ability_score: AbilityScore::Charisma,
                    limitation: Some(LimitationEnum::AbilityScoreAbove(
                        AbilityScore::Charisma,
                        10,
                    )),
                },
                SpellLikeAbility {
                    source: SlaSource::GnomeMagic,
                    spell_name: SpeakWithAnimals,
                    cast_frequency: CastFrequency::PerDay,
                    uses: Some(1),
                    ability_score: AbilityScore::Charisma,
                    limitation: Some(LimitationEnum::AbilityScoreAbove(
                        AbilityScore::Charisma,
                        10,
                    )),
                },
            ]),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SkillBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}
impl IntoComponentBuilder for GnomeMagicBuilder {
    fn from_name(racial_trait_name: &RacialTraitName) -> Result<Self, Box<dyn Error>> {
        use SpellName::*;
        match racial_trait_name {
            RacialTraitName::BaseGnomeGnomeMagic => Ok(Self {
                spell_dc_bonus: SpellDCBonus {
                    bonus: 1,
                    bonus_type: BonusType::Racial,
                    key: BonusType::Racial,
                    limitation: vec![LimitationEnum::SpellSchool(SpellSchool::Illusion)],
                },
                spell_like_abilities: vec![
                    SpellLikeAbility {
                        source: SlaSource::GnomeMagic,
                        spell_name: DancingLights,
                        cast_frequency: CastFrequency::PerDay,
                        uses: Some(1),
                        ability_score: AbilityScore::Charisma,
                        limitation: Some(LimitationEnum::AbilityScoreAbove(
                            AbilityScore::Charisma,
                            10,
                        )),
                    },
                    SpellLikeAbility {
                        source: SlaSource::GnomeMagic,
                        spell_name: GhostSound,
                        cast_frequency: CastFrequency::PerDay,
                        uses: Some(1),
                        ability_score: AbilityScore::Charisma,
                        limitation: Some(LimitationEnum::AbilityScoreAbove(
                            AbilityScore::Charisma,
                            10,
                        )),
                    },
                    SpellLikeAbility {
                        source: SlaSource::GnomeMagic,
                        spell_name: Prestidigitation,
                        cast_frequency: CastFrequency::PerDay,
                        uses: Some(1),
                        ability_score: AbilityScore::Charisma,
                        limitation: Some(LimitationEnum::AbilityScoreAbove(
                            AbilityScore::Charisma,
                            10,
                        )),
                    },
                    SpellLikeAbility {
                        source: SlaSource::GnomeMagic,
                        spell_name: SpeakWithAnimals,
                        cast_frequency: CastFrequency::PerDay,
                        uses: Some(1),
                        ability_score: AbilityScore::Charisma,
                        limitation: Some(LimitationEnum::AbilityScoreAbove(
                            AbilityScore::Charisma,
                            10,
                        )),
                    },
                ],
            }),
            _ => Err(format!(
                "Invalid RacialTraitName: {:?} for SkillBonus in from_name() \
                method of trait IntoComponentBuilder",
                racial_trait_name
            )
            .into()),
        }
    }
}

pub struct GnomeMagicBuilder {
    spell_dc_bonus: SpellDCBonus,
    spell_like_abilities: Vec<SpellLikeAbility>,
}
