/// This is only supporting Trouble Brewing.
use strum::{EnumIter, IntoEnumIterator};

pub mod solver;

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, Hash)]
pub enum Player {
    Unresolved,
    Seat(i32),
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Townsfolk {
    Washerwoman,
    Librarian,
    Investigator,
    Chef,
    Empath,
    FortuneTeller,
    Undertaker,
    Monk,
    Ravenkeeper,
    Virgin,
    Slayer,
    Soldier,
    Mayor,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Outsider {
    Butler,
    Saint,
    Recluse,
    Drunk,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Good {
    Townsfolk(Townsfolk),
    Outsider(Outsider),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Evil {
    Minion(Minion),
    Demon(Demon),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Minion {
    Baron,
    Poisoner,
    ScarletWoman,
    Spy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Demon {
    Imp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Character {
    Good(Good),
    Evil(Evil),
}

impl Good {
    pub fn iter() -> impl Iterator<Item = Self> {
        Townsfolk::iter()
            .map(Good::Townsfolk)
            .chain(Outsider::iter().map(Good::Outsider))
    }
}

impl Evil {
    pub fn iter() -> impl Iterator<Item = Self> {
        Minion::iter()
            .map(Evil::Minion)
            .chain(Demon::iter().map(Evil::Demon))
    }
}

impl Character {
    pub fn iter() -> impl Iterator<Item = Self> {
        Townsfolk::iter()
            .map(|t| Character::Good(Good::Townsfolk(t)))
            .chain(Outsider::iter().map(|o| Character::Good(Good::Outsider(o))))
            .chain(Minion::iter().map(|m| Character::Evil(Evil::Minion(m))))
            .chain(Demon::iter().map(|d| Character::Evil(Evil::Demon(d))))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LiveOrDie {
    Lives,
    Dies,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Info {
    Neither,
    One,
    Both,
}

// This is the player visible log.
// Some of this might come out publicily.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claim {
    WasherwomanSees(Player, Player, Townsfolk), // https://wiki.bloodontheclocktower.com/Washerwoman
    LibrarianSees(Player, Player, Outsider),    // https://wiki.bloodontheclocktower.com/Librarian
    InvestigatorSees(Player, Player, Minion), // https://wiki.bloodontheclocktower.com/Investigator
    Chef(i32),                                // https://wiki.bloodontheclocktower.com/Chef
    EmpathLearns(Player, Player, Info),       // https://wiki.bloodontheclocktower.com/Empath
    FortuneTellerLearns(Player, Player, bool), // https://wiki.bloodontheclocktower.com/Fortune_Teller
    UndertakerSees(Player, Character),
    MonkProtects(Player),
    Ravenkeeper(Player, Character),
    VirginIsNominatedBy(Player, LiveOrDie), // Virgin is nominated by `Player`, and might LiveOrDie
    SlayerShoots(Player, LiveOrDie),        // Slayer shoots the `Player` who will LiveOrDie

    // Events that happen.
    Executes(Player),
    DiesAtNight(Player),

    // TODO: It might be worth it to take a good look at minion events.
    // In particular, I think most of these might not be very useful.
    PoisonerPoisons(Player),
    Baron,
    ScarletWomanDemonizes,
    Spy,
    ImpKills(Player),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportLog {
    OnTime(Player, Claim),
}
