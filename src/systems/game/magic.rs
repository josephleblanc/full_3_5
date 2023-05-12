use crate::systems::game::character;
use bevy::prelude::*;

//// How Spells will work
// Spells will each have their own `system` and `Event`. When the spell is
// cast by a player, the `Event` will alert the spell's `system` to run, and
// that will make any needed queries and mutations to effect the spell.
//
// Hopefully, most of the effects of a spell, as implemented in the unique-per-
// spell `system`, will simply be to add components to effected entities and
// then have those background systems handle the results, while the unique-per-
// spell `system` goes back to sleep.
//
// This could also be done by making a struct for each Spell and attaching the
// spell logic as a method there, perhaps using the
//
// Example:
//
// fn fireball(query: <Query<&Position, &mut HealthPointes, &SavingThrow>>) {
//     /* check if inside position, check savingthrow,
//     deduct healthpoints, etc */
// }
//
// fn main {
// /* ... */
//    app
//      .add_event::<FireBallEvent>()
//      .add_system(
//          fireball.run_if(on_event::<FireBallEvent>())
//      );
//  }

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq, Ord)]
pub enum SlaSource {
    GnomeMagic,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq)]
pub struct SpellLikeAbility {
    pub source: SlaSource,
    pub spell_name: SpellName,
    pub cast_frequency: CastFrequency,
    pub uses: Option<u32>,
    // The ability score used to determine the DC of the SLA
    pub ability_score: character::AbilityScore,
    pub limitation: Option<character::LimitationEnum>,
    /* more fields here */
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq, Ord)]
pub enum CastFrequency {
    PerDay,
    AtWill,
}

// Ways in which Caster Level can be used, useful for setting limitations
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq, Ord)]
pub enum CasterLevelUse {
    Cast,
    Dispel,
    OvercomeSpellResistance,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Hash, Copy, Eq, Ord)]
pub enum SpellName {
    //// Spells from Gnome Magic trait, used to implement the racial trait.
    // Level 0
    DancingLights,
    GhostSound,
    Prestidigitation,
    // Level 1
    SpeakWithAnimals,
}

// Each spell should have it's own system
pub struct Spell {
    spell_name: SpellName,
    school: SpellSchool,
    sub_school: SpellSubSchool,
    descriptor: SpellDescriptor,
    range: SpellRange,
    /* more fields */
}

use bevy::core::NonSendMarker;
trait SpellEffect<T>
where
    T: IntoSystemAppConfig<NonSendMarker>,
{
    fn spell_effect(spell_function: T) {}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Tranmutation,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellSubSchool {
    // Conjuration Subschools
    Calling,
    Creation,
    Healing,
    Summoning,
    Teleportation,
    // Divination Subschools
    Scrying,
    // Enchantment Subschools
    Charm,
    Compulsion,
    // No Evocation Subschools noted on d20pfsrd
    //
    // Illusion Subschools
    Figment,
    Glamer,
    Pattern,
    Phantasm,
    Shadow,
    // No Necromancy Subschools noted on d20pfsrd
    //
    // Transmutation Subschools
    Polymorph,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellDescriptor {
    Acid,
    Air,
    Chaotic,
    Cold,
    Curse, // Source PZO117 & additional info from PZO1135
    Darkness,
    Death,
    Disease,  // PZO1117
    Draconic, // PZO9470
    Earth,
    Electricity,
    Emotion, // PZO1117
    Evil,    // PZO1135 - additional information
    Force,
    Good,
    LanguageDependent,
    Lawful,
    Light,
    Meditative, // PPC:DA
    MindAffecting,
    Pain,   // PZO1117
    Poison, // PZO1117
    Ruse,   // PZO1134
    Shadow, // PZO1117
    Sonic,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellRange {
    Personal,
    Touch,
    Close,
    Medium,
    Long,
    Unlimited,
    NonStandard(usize),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellDuration {
    Timed,
    Instant,
    Permanent,
    Concentration,
    ChargeTouch,
    Discharge,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub struct Dismissable(bool);

impl Dismissable {
    fn is_dismissable(&self) -> bool {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SavingThrow {
    Negates,
    Partial,
    Half,
    None,
    Disbelief,
    Object,
    Harmless,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Hash, Eq)]
pub enum SpellResistance {
    Yes,
    No,
    Partial,
}
