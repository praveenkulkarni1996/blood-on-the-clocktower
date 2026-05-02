/// This is only supporting Trouble Brewing.
use strum::{EnumIter, IntoEnumIterator};

pub mod solver;

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter, Hash, PartialOrd, Ord)]
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

/// Somewhat annoyingly, the Blood on the Clocktower has decided that it will
/// follow a 1-indexed night naming convention, i.e. we have:
/// Night-1, Day-1, Night-2, etc.
#[derive(Debug, Clone, Copy, Hash)]
pub enum Time {
    Night(i32),
    Day(i32),
}

pub struct TimeIterator {
    current: Time,
    end: Time,
}

impl TimeIterator {
    pub fn new(end: Time) -> Self {
        Self {
            current: Time::Night(1),
            end,
        }
    }
}

impl Iterator for TimeIterator {
    type Item = Time;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let result = self.current;

        // Logic: Night(n) -> Day(n) -> Night(n+1)
        self.current = match self.current {
            Time::Night(n) => Time::Day(n),
            Time::Day(n) => Time::Night(n + 1),
        };

        Some(result)
    }
}

impl Ord for Time {
    /// The ordering is: Night(1) < Day(1) < Night(2) < Day(2) < ...
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn canonicalize(t: &Time) -> i32 {
            match t {
                Time::Night(x) => 2 * x,
                Time::Day(x) => (2 * x) + 1,
            }
        }
        return canonicalize(self).cmp(&canonicalize(other));
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// This is the player visible log.
// Some of this might come out publicily.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Claim {
    WasherwomanSees(Player, Player, Townsfolk), // https://wiki.bloodontheclocktower.com/Washerwoman
    LibrarianSees(Player, Player, Outsider),    // https://wiki.bloodontheclocktower.com/Librarian
    InvestigatorSees(Player, Player, Minion), // https://wiki.bloodontheclocktower.com/Investigator
    ChefGets(i32),                            // https://wiki.bloodontheclocktower.com/Chef
    EmpathLearnsZero(Player, Player),
    EmpathLearnsOne(Player, Player),
    EmpathLearnsTwo(Player, Player),
    FortuneTellerYes(Player, Player), // https://wiki.bloodontheclocktower.com/Fortune_Teller
    FortuneTellerNo(Player, Player),  // https://wiki.bloodontheclocktower.com/Fortune_Teller
    UndertakerSees(Player, Character),
    RavenkeeperSees(Player, Character),
    VirginIsNominatedBy(Player, LiveOrDie), // Virgin is nominated by `Player`, and might LiveOrDie
    SlayerShoots(Player, LiveOrDie),        // Slayer shoots the `Player` who will LiveOrDie

    MonkClaims(),
    // TODO: It might be worth it to take a good look at minion events.
    // In particular, I think most of these might not be very useful.
    // These are primarily there for debugging purposes.
    PoisonerPoisons(Player),
    Baron,
    ScarletWomanDemonizes,
    ImpKills(Player),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportLog {
    OnTime(Time, Player, Claim),
    Executes(Time, Player),
    DiesAtNight(Time, Player),
}
