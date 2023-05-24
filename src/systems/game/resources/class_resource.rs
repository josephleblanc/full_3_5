#![allow(unused_mut, unused_variables)]
use crate::systems::game::character::*;
use crate::systems::game::{class::*, skills::SkillName::*};
use crate::technical::class::ClassAsset;
use bevy::prelude::*;
pub fn setup_classes(mut commands: Commands, class_asset: Res<Assets<ClassAsset>>) {
    use BABProgression::*;
    use ClassFeature::*;
    use FighterFeature::*;
    use SavingThrowName::*;
    let mut class_map = ClassMap::new();

    let mut fighter = ClassInfo {
        name_str: String::new(),
        description: String::new(),
        class_name: PlayableClass::Fighter,
        class_skills: vec![
            Climb,
            Craft,
            HandleAnimal,
            Intimidate,
            KnowledgeDungeoneering,
            KnowledgeEngineering,
            Profession,
            Ride,
            Survival,
            Swim,
        ],
        skill_ranks_per_level: 2,
        hit_die: Dice::D10,
        alignment_restriction: Option::None,
        bab_progression: Full,
        save_progression: [Will, SavingThrowName::None, SavingThrowName::None],
        class_features_list: vec![
            Fighter(BonusFeat),
            Fighter(Bravery),
            Fighter(ArmorTraining),
            Fighter(WeaponTraining),
            Fighter(ArmorMastery),
            Fighter(WeaponMastery),
        ],
        class_features: vec![
            vec![Fighter(BonusFeat)],                         // 1
            vec![Fighter(Bravery)],                           // 2
            vec![Fighter(ArmorTraining)],                     // 3
            vec![Fighter(BonusFeat)],                         // 4
            vec![Fighter(WeaponTraining)],                    // 5
            vec![Fighter(BonusFeat), Fighter(Bravery)],       // 6
            vec![Fighter(ArmorTraining)],                     // 7
            vec![Fighter(BonusFeat)],                         // 8
            vec![Fighter(WeaponTraining)],                    // 9
            vec![Fighter(BonusFeat), Fighter(Bravery)],       // 10
            vec![Fighter(ArmorTraining)],                     // 11
            vec![Fighter(BonusFeat)],                         // 12
            vec![Fighter(WeaponTraining)],                    // 13
            vec![Fighter(BonusFeat), Fighter(Bravery)],       // 14
            vec![Fighter(ArmorTraining)],                     // 15
            vec![Fighter(BonusFeat)],                         // 16
            vec![Fighter(WeaponTraining)],                    // 17
            vec![Fighter(BonusFeat), Fighter(Bravery)],       // 18
            vec![Fighter(ArmorMastery)],                      // 19
            vec![Fighter(BonusFeat), Fighter(WeaponMastery)], // 20
        ],
    };
}
