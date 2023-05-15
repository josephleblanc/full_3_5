use crate::systems::game::{character::*, equipment::*, race::*};
use bevy::prelude::*;

pub fn race_builder(race_builder: Res<RaceBuilder>) {
    info!("RaceBuilder: {:#?}", race_builder);
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
            if let Some(_) = width.checked_sub(name_length) {
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
