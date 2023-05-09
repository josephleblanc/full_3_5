use crate::systems::game::character::SizeCategory;
use std::fmt;

pub struct Weapon {
    name: WeaponName,
    proficiency: WeaponProficiency,
    base_damage: WeaponDamage,
    size: SizeCategory,
    critical: Critical,
    range: f32,
    weight: f32,
    damage_type: WeaponDamageType,
}

pub enum WeaponDamageType {
    Bludgeoning,
    Slashing,
    Piercing,
}

pub struct Critical {
    range: usize,
    multiplier: usize,
}

pub struct WeaponDamage {
    dice_number: usize,
    dice_sides: usize,
}

pub enum WeaponProficiency {
    Simple,
    Martial,
    Exotic,
}

pub enum WeaponName {
    Gauntlet,
    UnarmedStrike,
    BattleAspergillum,
    BrassKnife,
    BrassKnuckles,
    Cestus,
    Dagger,
    DaggerPunching,
    GauntleSpiked,
    Handwraps,
    TravelingKettle,
    HookHand,
    Kunai,
    MaceLight,
    Sickle,
    SpringBlade,
    WoodenStake,
    // Simple One-handed Melee
    Club,
    ClubMere,
    MaceHeavy,
    Shortspear,
    // Simple Two-handed Melee Weapons
    Bayonet,
    BoardingPike,
    Kumade,
    KumadeCollapsible,
    LanternStaff,
    Longspear,
    Quarterstaff,
    Spear,
    SpearBoar,
    SpearWeighted,
    // Simple Ranged Weapons
    Blowgun,
    CrossbowHeavy,
    CrossbowHeavyUnderwater,
    CrossbowLight,
    Dart,
    Javalin,
    Sling,
    Stingchuck,
    Stonebow,
    //// Martial Weapons
    // Light Melee Weapons
    AxeBoarding,
    AxeThrowing,
    BladeBoot,
    CatoNineTails,
    ClawBlades,
    DaggerDueling,
    Dogslicer,
    HammerLight,
    Gladius,
    Handaxe,
    KatarTriBladed,
    KnifeSwitchblade,
    KoboldTailAttachmentLongLash,
    KoboldTailAttachmentPounder,
    KoboldTailAttachmentRazored,
    KoboldTailAttachmentSpiked,
    KoboldTailAttachmentSweeper,
    Kukri,
    Machete,
    PickLight,
    RatfolkTailblade,
    Sap,
    SeaKnife,
    ShieldLight,
    SpikedArmor,
    SpikedShieldLight,
    Starknife,
    SwordShort,
    WarRazor,
    // One-Handed Melee Weapons
    Ankus,
    BattleAxe,
    CombatScabbard,
    Cutlass,
    FlailLight,
    Gandasa,
    Klar,
    Longsword,
    Manople,
    PickHeavy,
    Rapier,
    CombatScabbardSharpened,
    Scimitar,
    Scizore,
    ShieldHeavy,
    SpikedShieldHeavy,
    SwordCane,
    Terbutje,
    TerbutjeSteel,
    Trident,
    Warhammer,
    // Two-Handed Melee Weapons
    Bardiche,
    BecDeCorbin,
    Bill,
    EarthBreaker,
    Falchion,
    FlailHeavy,
    Glaive,
    GlaiveGuisarme,
    Greataxe,
    Greatclub,
    Greatsword,
    Guisarme,
    Halberd,
    HammerLucerne,
    Horsechopper,
    Lance,
    OgreHook,
    Pickaxe,
    Planson,
    Ranseur,
    Scythe,
    SpearSyringe,
    // Ranged Weapons
    Ammentum,
    Chakram,
    DartJolting,
    HungaMunga,
    Hurlbat,
    Longbow,
    LongbowComposite,
    Pilum,
    Shortbow,
    ShortbowComposite,
    SpearSling,
    ThrowingArrowCord,
    //// Exotic
    // Light Melee Weapons
    Aklys,
    AxeGauntletDwarvenLight,
    AxeKnuckle,
    BarbazuBeard,
    BattlePoi,
    DaggerSwordbreaker,
    FlyingTalon,
    GnomePincher,
    HalflingRopeShot,
    HelmetDwarvenBoulder,
    Kama,
    KnifeButterfly,
    KnifeDeerHorn,
    MaulaxeDwarven,
    Nunchaku,
    Quadrens,
    RazorDrow,
    RopeGauntlet,
    SabreSawtoothLight, // can sometimes be light or medium, see details
    Sai,
    Sanpkhang,
    Siangham,
    Sica,
    ThornBracer,
    WarShieldDwarven,
    Waveblade,
    WhipScorpion,
    // One-Handed
    AxeGauntletDwarvenHeavy,
    AxeHooked,
    BrokenBackSeax,
    Estoc,
    Falcata,
    Flickmace,
    Flindbar,
    Khopesh,
    Knobkerrie,
    RamHammerDwarven,
    RapierSpiral,
    Rhoka,
    SabreSawtoothMedium, // can sometimes be light or medium, see details
    Shotel,
    SickleSword,
    SplitBladeSword,
    SwordDueling,
    SwordBastard,
    Tongi,
    WaraxeDwarven,
    WaraxeDwarvenDouble,
    Whip,
    // Two-Handed
    AxeOrcDouble,
    AxeButchering,
    BattleLadderGnome,
    BoardingGaff,
    ChainHammer,
    ChainSpiked,
    Crook,
    CurveBladeElven,
    DornDergarDwarven,
    DoubleSpear,
    ElvenBranchedSpear,
    Fauchard,
    FlailDire,
    Flailpole,
    Flambard,
    FlyingBlade,
    Garrote,
    GiantStickerDwarven,
    HammerGnomeHooked,
    Harpoon,
    LongaxeDwarven,
    LongHammerDwarven,
    Mancatcher,
    OrcSkullRam,
    PistonMaulGnome,
    RipsawGlaiveGnome,
    ScarfBladed,
    SpearTotem,
    SphinxHammerDwarven,
    Switchscythe,
    SwordTwoBladed,
    UgroshDwarven,
    // Ranged
    DartsFeatherweight,
    Bola,
    BolaBrutal,
    Boomerang,
    BowThorn,
    CrossbowCrankHeavy,
    CrossbowCrankLight,
    CrossbowDouble,
    CrossbowHand,
    CrossbowLaunching,
    CrossbowRepeatingHeavy,
    CrossbowRepeatingLight,
    FlaskThrower,
    GrapplingHook,
    HornbowOrc,
    JavalinStormshaft,
    Lasso,
    Net,
    NetSnag,
    PelletbowDwarvenLight,
    PelletbowDwarvenHeavy,
    ShieldThrowing,
    ShrillshaftJavalin,
    Shuriken,
    SlingDouble,
    SlingGlove,
    SlingStaffHalfling,
    SlingStitched,
    WristLauncher,
    WristLauncherHeavy,
}

pub enum RacialWeapon {
    Dwarven,
    Orc,
    Elf,
}

impl RacialWeapon {
    fn weapon_names_vec(&self) -> Vec<WeaponName> {
        use WeaponName::*;
        match self {
            Self::Dwarven => vec![
                WaraxeDwarvenDouble,
                WaraxeDwarven,
                UgroshDwarven,
                MaulaxeDwarven,
                LongaxeDwarven,
                HelmetDwarvenBoulder,
                PelletbowDwarvenLight,
                WarShieldDwarven,
                RamHammerDwarven,
                LongHammerDwarven,
                SphinxHammerDwarven,
                GiantStickerDwarven,
                AxeGauntletDwarvenHeavy,
                AxeGauntletDwarvenLight,
                DornDergarDwarven,
                MaulaxeDwarven,
            ],
            Self::Orc => vec![],
            Self::Elf => vec![],
        }
    }
}

impl fmt::Display for WeaponName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use WeaponName::*;
        match self {
            Gauntlet => write!(f, "Gauntlet"),
            UnarmedStrike => write!(f, "Unarmed Strike"),
            BattleAspergillum => write!(f, "Battle Aspergillum"),
            BrassKnife => write!(f, "Brass Knife"),
            BrassKnuckles => write!(f, "Brass Knuckles"),
            Cestus => write!(f, "Cestus"),
            Dagger => write!(f, "Dagger"),
            DaggerPunching => write!(f, "Dagger Punching"),
            GauntleSpiked => write!(f, "Gauntle Spiked"),
            Handwraps => write!(f, "Handwraps"),
            TravelingKettle => write!(f, "Traveling Kettle"),
            HookHand => write!(f, "Hook Hand"),
            Kunai => write!(f, "Kunai"),
            MaceLight => write!(f, "Mace, Light"),
            Sickle => write!(f, "Sickle"),
            SpringBlade => write!(f, "Spring Blade"),
            WoodenStake => write!(f, "Wooden Stake"),
            Club => write!(f, "Club"),
            ClubMere => write!(f, "Club, Mere"),
            MaceHeavy => write!(f, "Mace, Heavy"),
            Shortspear => write!(f, "Shortspear"),
            Bayonet => write!(f, "Bayonet"),
            BoardingPike => write!(f, "Boarding Pike"),
            Kumade => write!(f, "Kumade"),
            KumadeCollapsible => write!(f, "Kumade, Collapsible"),
            LanternStaff => write!(f, "Lantern Staff"),
            Longspear => write!(f, "Longspear"),
            Quarterstaff => write!(f, "Quarterstaff"),
            Spear => write!(f, "Spear"),
            SpearBoar => write!(f, "Spear, Boar"),
            SpearWeighted => write!(f, "Spear, Weighted"),
            Blowgun => write!(f, "Blowgun"),
            CrossbowHeavy => write!(f, "Crossbow, Heavy"),
            CrossbowHeavyUnderwater => write!(f, "Crossbow, Heavy Underwater"),
            CrossbowLight => write!(f, "Crossbow, Light"),
            Dart => write!(f, "Dart"),
            Javalin => write!(f, "Javalin"),
            Sling => write!(f, "Sling"),
            Stingchuck => write!(f, "Stingchuck"),
            Stonebow => write!(f, "Stonebow"),
            AxeBoarding => write!(f, "Axe, Boarding"),
            AxeThrowing => write!(f, "Axe, Throwing"),
            BladeBoot => write!(f, "Blade Boot"),
            CatoNineTails => write!(f, "Cat-o'-Nine-Tails"),
            ClawBlades => write!(f, "Claw Blades"),
            DaggerDueling => write!(f, "Dagger, Dueling"),
            Dogslicer => write!(f, "Dogslicer"),
            HammerLight => write!(f, "Hammer, Light"),
            Gladius => write!(f, "Gladius"),
            Handaxe => write!(f, "Handaxe"),
            KatarTriBladed => write!(f, "Katar, Tri-Bladed"),
            KnifeSwitchblade => write!(f, "Knife, Switchblade"),
            KoboldTailAttachmentLongLash => write!(f, "Kobold Tail Attachment (LongLash)"),
            KoboldTailAttachmentPounder => write!(f, "Kobold Tail Attachment (Pounder)"),
            KoboldTailAttachmentRazored => write!(f, "Kobold Tail Attachment (Razored)"),
            KoboldTailAttachmentSpiked => write!(f, "Kobold Tail Attachment (Spiked)"),
            KoboldTailAttachmentSweeper => write!(f, "Kobold Tail Attachment (Sweeper)"),
            Kukri => write!(f, "Kukri"),
            Machete => write!(f, "Machete"),
            PickLight => write!(f, "Pick, Light"),
            RatfolkTailblade => write!(f, "Ratfolk Tailblade"),
            Sap => write!(f, "Sap"),
            SeaKnife => write!(f, "Sea Knife"),
            ShieldLight => write!(f, "Shield, Light"),
            SpikedArmor => write!(f, "Spiked Armor"),
            SpikedShieldLight => write!(f, "Spiked Shield, Light"),
            Starknife => write!(f, "Starknife"),
            SwordShort => write!(f, "Short Sword"),
            WarRazor => write!(f, "War Razor"),
            Ankus => write!(f, "Ankus"),
            BattleAxe => write!(f, "Battle Axe"),
            CombatScabbard => write!(f, "Combat Scabbard"),
            Cutlass => write!(f, "Cutlass"),
            FlailLight => write!(f, "Flail, Light"),
            Gandasa => write!(f, "Gandasa"),
            Klar => write!(f, "Klar"),
            Longsword => write!(f, "Longsword"),
            Manople => write!(f, "Manople"),
            PickHeavy => write!(f, "Pick, Heavy"),
            Rapier => write!(f, "Rapier"),
            CombatScabbardSharpened => write!(f, "Combat Scabbard, Sharpened"),
            Scimitar => write!(f, "Scimitar"),
            Scizore => write!(f, "Scizore"),
            ShieldHeavy => write!(f, "Shield, Heavy"),
            SpikedShieldHeavy => write!(f, "Spiked Shield, Heavy"),
            SwordCane => write!(f, "Sword Cane"),
            Terbutje => write!(f, "Terbutje"),
            TerbutjeSteel => write!(f, "Terbutje, Steel"),
            Trident => write!(f, "Trident"),
            Warhammer => write!(f, "Warhammer"),
            Bardiche => write!(f, "Bardiche"),
            BecDeCorbin => write!(f, "Bec de Corbin"),
            Bill => write!(f, "Bill"),
            EarthBreaker => write!(f, "Earth Breaker"),
            Falchion => write!(f, "Falchion"),
            FlailHeavy => write!(f, "Flail, Heavy"),
            Glaive => write!(f, "Glaive"),
            GlaiveGuisarme => write!(f, "Glaive, Guisarme"),
            Greataxe => write!(f, "Greataxe"),
            Greatclub => write!(f, "Greatclub"),
            Greatsword => write!(f, "Greatsword"),
            Guisarme => write!(f, "Guisarme"),
            Halberd => write!(f, "Halberd"),
            HammerLucerne => write!(f, "Hammer, Lucerne"),
            Horsechopper => write!(f, "Horsechopper"),
            Lance => write!(f, "Lance"),
            OgreHook => write!(f, "Ogre Hook"),
            Pickaxe => write!(f, "Pickaxe"),
            Planson => write!(f, "Planson"),
            Ranseur => write!(f, "Ranseur"),
            Scythe => write!(f, "Scythe"),
            SpearSyringe => write!(f, "Spear, Syringe"),
            Ammentum => write!(f, "Ammentum"),
            Chakram => write!(f, "Chakram"),
            DartJolting => write!(f, "Dart, Jolting"),
            HungaMunga => write!(f, "Hunga Munga"),
            Hurlbat => write!(f, "Hurlbat"),
            Longbow => write!(f, "Longbow"),
            LongbowComposite => write!(f, "Longbow, Composite"),
            Pilum => write!(f, "Pilum"),
            Shortbow => write!(f, "Shortbow"),
            ShortbowComposite => write!(f, "Shortbow, Composite"),
            SpearSling => write!(f, "Spear Sling"),
            ThrowingArrowCord => write!(f, "Throwing Arrow Cord"),
            Aklys => write!(f, "Aklys"),
            AxeGauntletDwarvenLight => write!(f, "Dwarven Axe Gauntlet, Light"),
            AxeKnuckle => write!(f, "Axe, Knuckle"),
            BarbazuBeard => write!(f, "Barbazu Beard"),
            BattlePoi => write!(f, "Battle Poi"),
            DaggerSwordbreaker => write!(f, "Dagger, Swordbreaker"),
            FlyingTalon => write!(f, "Flying Talon"),
            GnomePincher => write!(f, "Gnome Pincher"),
            HalflingRopeShot => write!(f, "Halfling Rope Shot"),
            HelmetDwarvenBoulder => write!(f, "Dwarven Boulder Helmet"),
            Kama => write!(f, "Kama"),
            KnifeButterfly => write!(f, "Butterfly Knife"),
            KnifeDeerHorn => write!(f, "Deer Horn Knife"),
            MaulaxeDwarven => write!(f, "Dwarven Maulaxe"),
            Nunchaku => write!(f, "Nunchaku"),
            Quadrens => write!(f, "Quadrens"),
            RazorDrow => write!(f, "Drow Razor"),
            RopeGauntlet => write!(f, "Rope Gauntlet"),
            SabreSawtoothLight => write!(f, "Sawtooth Sabre (Light)"),
            Sai => write!(f, "Sai"),
            Sanpkhang => write!(f, "Sanpkhang"),
            Siangham => write!(f, "Siangham"),
            Sica => write!(f, "Sica"),
            ThornBracer => write!(f, "Thorn Bracer"),
            WarShieldDwarven => write!(f, "Dwarven War Shield"),
            Waveblade => write!(f, "Waveblade"),
            WhipScorpion => write!(f, "Scorpion Whip"),
            AxeGauntletDwarvenHeavy => write!(f, "Dwarven Axe Gauntlet, Heavy"),
            AxeHooked => write!(f, "Hooked Axe"),
            BrokenBackSeax => write!(f, "Broken-back Seax"),
            Estoc => write!(f, "Estoc"),
            Falcata => write!(f, "Falcata"),
            Flickmace => write!(f, "Flickmace"),
            Flindbar => write!(f, "Flindbar"),
            Khopesh => write!(f, "Khopesh"),
            Knobkerrie => write!(f, "Knobkerrie"),
            RamHammerDwarven => write!(f, "Dwarven Ram Hammer"),
            RapierSpiral => write!(f, "Spiral Rapier"),
            Rhoka => write!(f, "Rhoka"),
            SabreSawtoothMedium => write!(f, "Sawtooth Sabre (Medium)"),
            Shotel => write!(f, "Shotel"),
            SickleSword => write!(f, "Sickle-sword"),
            SplitBladeSword => write!(f, "Split-blade Sword"),
            SwordDueling => write!(f, "Sword, Dueling"),
            SwordBastard => write!(f, "Sword, Bastard"),
            Tongi => write!(f, "Tongi"),
            WaraxeDwarven => write!(f, "Dwarven Waraxe"),
            WaraxeDwarvenDouble => write!(f, "Dwarven Double Waraxe"),
            Whip => write!(f, "Whip"),
            AxeOrcDouble => write!(f, "Orc Double Axe"),
            AxeButchering => write!(f, "Butchering Axe"),
            BattleLadderGnome => write!(f, "Gnome Battle Ladder"),
            BoardingGaff => write!(f, "Boarding Gaff"),
            ChainHammer => write!(f, "Chain-hammer"),
            ChainSpiked => write!(f, "Spiked Chain"),
            Crook => write!(f, "Crook"),
            CurveBladeElven => write!(f, "Elven Curve Blade"),
            DornDergarDwarven => write!(f, "Dwarven Dorn-Dergar"),
            DoubleSpear => write!(f, "Double Spear"),
            ElvenBranchedSpear => write!(f, "Elven Branched Spear"),
            Fauchard => write!(f, "Fauchard"),
            FlailDire => write!(f, "Dire Flail"),
            Flailpole => write!(f, "Flailpole"),
            Flambard => write!(f, "Flambard"),
            FlyingBlade => write!(f, "Flying Blade"),
            Garrote => write!(f, "Garrote"),
            GiantStickerDwarven => write!(f, "Dwarven Giant Sticker"),
            HammerGnomeHooked => write!(f, "Gnome Hooked Hammer"),
            Harpoon => write!(f, "Harpoon"),
            LongaxeDwarven => write!(f, "Dwarven Longaxe"),
            LongHammerDwarven => write!(f, "Dwarven Long Hammer"),
            Mancatcher => write!(f, "Mancatcher"),
            OrcSkullRam => write!(f, "Orc Skull Ram"),
            PistonMaulGnome => write!(f, "Gnome Piston Maul"),
            RipsawGlaiveGnome => write!(f, "Gnome Ripsaw Glaive"),
            ScarfBladed => write!(f, "Bladed Scarf"),
            SpearTotem => write!(f, "Totem Spear"),
            SphinxHammerDwarven => write!(f, "Dwarven Sphinx Hammer"),
            Switchscythe => write!(f, "Switchscythe"),
            SwordTwoBladed => write!(f, "Two-Bladed Sword"),
            UgroshDwarven => write!(f, "Dwarven Ugrosh"),
            DartsFeatherweight => write!(f, "Featherweight Darts"),
            Bola => write!(f, "Bola"),
            BolaBrutal => write!(f, "Brutal Bola"),
            Boomerang => write!(f, "Boomerang"),
            BowThorn => write!(f, "Thorn Bow"),
            CrossbowCrankHeavy => write!(f, "Crank Crossbow, Heavy"),
            CrossbowCrankLight => write!(f, "Crank Crossbow, Light"),
            CrossbowDouble => write!(f, "Double Crossbow"),
            CrossbowHand => write!(f, "Hand Crossbow"),
            CrossbowLaunching => write!(f, "Launching Crossbow"),
            CrossbowRepeatingHeavy => write!(f, "Repeating Crossbow, Heavy"),
            CrossbowRepeatingLight => write!(f, "Repeating Crossbow, Light"),
            FlaskThrower => write!(f, "Flask Thrower"),
            GrapplingHook => write!(f, "Grappling Hook"),
            HornbowOrc => write!(f, "Orc Hornbow"),
            JavalinStormshaft => write!(f, "Stormshaft Javalin"),
            Lasso => write!(f, "Lasso"),
            Net => write!(f, "Net"),
            NetSnag => write!(f, "Snag Net"),
            PelletbowDwarvenLight => write!(f, "Dwarven Pelletbow, Light"),
            PelletbowDwarvenHeavy => write!(f, "Dwarven Pelletbow, Heavy"),
            ShieldThrowing => write!(f, "Throwing Shield"),
            ShrillshaftJavalin => write!(f, "Javalin Shrillshaft"),
            Shuriken => write!(f, "Shuriken"),
            SlingDouble => write!(f, "Double Sling"),
            SlingGlove => write!(f, "Glove Sling"),
            SlingStaffHalfling => write!(f, "Halfling Sling Staff"),
            SlingStitched => write!(f, "Stitched Sling"),
            WristLauncher => write!(f, "Wrist Launcher"),
            WristLauncherHeavy => write!(f, "Wrist Launcher, Heavy"),
        }
    }
}
