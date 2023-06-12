use bevy::prelude::*;

use crate::systems::game::{
    archetype::{ArchetypeFeature, ArchetypeInfo, ArchetypeMap, ArchetypeName},
    class::{ClassFeature, FighterFeature, PlayableClass},
};

pub fn setup_archetypes(mut commands: Commands) {
    let mut archetype_map = ArchetypeMap::new();

    //// brawler features
    let close_control = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::CloseControl(None)),
        replaces: vec![ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(
            1,
        )))],
        level: 2,
    };
    let close_combatant = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::CloseCombatant(None)),
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(1))),
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(2))),
        ],
        level: 3,
    };
    let menacing_stance = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::MenacingStance(None)),
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(2))),
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(3))),
            ClassFeature::Fighter(FighterFeature::ArmorTraining(Some(4))),
            ClassFeature::Fighter(FighterFeature::ArmorMastery),
        ],
        level: 7,
    };
    let no_escape = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::NoEscape(None)),
        replaces: vec![
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(3))),
            ClassFeature::Fighter(FighterFeature::WeaponTraining(Some(4))),
        ],
        level: 9,
    };
    let stand_still = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::StandStill),
        replaces: vec![],
        level: 9,
    };
    let weapon_mastery_brawler = ArchetypeFeature {
        feature: ClassFeature::Fighter(FighterFeature::WeaponMasteryBrawler),
        replaces: vec![ClassFeature::Fighter(FighterFeature::WeaponMastery)],
        level: 20,
    };
    let brawler = ArchetypeInfo {
        name: ArchetypeName::Brawler,
        class: PlayableClass::Fighter,
        restrictions: None,
        archetype_features: Some(vec![
            close_control,
            close_combatant,
            menacing_stance,
            no_escape,
            stand_still,
            weapon_mastery_brawler,
        ]),
        skills: None,
        skill_ranks: None,
        gains_proficiency: None,
        loses_proficiency: None,
    };

    archetype_map
        .inner_ref_mut()
        .insert(ArchetypeName::Brawler, brawler);

    commands.insert_resource(archetype_map);
}
