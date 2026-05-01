/// This is only supporting Trouble Brewing.
use strum::EnumIter;

pub mod solver;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Unresolved,
    Seat(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Outsider {
    Butler,
    Saint,
    Recluse,
    Drunk,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Good {
    Townsfolk(Townsfolk),
    Outsider(Outsider),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Evil {
    Minion(Minion),
    Demon(Demon),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Minion {
    Baron,
    Poisoner,
    ScarletWoman,
    Spy,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Demon {
    Imp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Character {
    Good(Good),
    Evil(Evil),
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

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    Is(Player, Character),
    IsPoisoned(Player),
    IsDrunk(Player, Townsfolk),
    IsRedHerring(Player),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompoundConstraint {
    AllOf(Vec<Constraint>),
    OneOf(Vec<Vec<Constraint>>),
}

pub fn expect(report_log: ReportLog) -> CompoundConstraint {
    use crate::Demon::*;
    use crate::Minion::*;
    use crate::Outsider::*;
    use crate::Townsfolk::*;

    use crate::Evil::{Demon, Minion};
    use crate::Good::{Outsider, Townsfolk};

    //
    use Character::*;
    use CompoundConstraint::OneOf;
    use Constraint::*;

    match report_log {
        // Washerwoman
        ReportLog::OnTime(washerwoman, Claim::WasherwomanSees(a, b, townsfolk)) => OneOf(vec![
            vec![
                Is(washerwoman, Good(Townsfolk(Washerwoman))),
                Is(a, Good(Townsfolk(townsfolk))),
            ],
            vec![
                Is(washerwoman, Good(Townsfolk(Washerwoman))),
                Is(b, Good(Townsfolk(townsfolk))),
            ],
            vec![
                Is(washerwoman, Good(Townsfolk(Washerwoman))),
                Is(a, Evil(Minion(Spy))),
            ],
            vec![
                Is(washerwoman, Good(Townsfolk(Washerwoman))),
                Is(b, Evil(Minion(Spy))),
            ],
            vec![IsPoisoned(washerwoman)],
            vec![IsDrunk(washerwoman, Washerwoman)],
        ]),

        ReportLog::OnTime(librarian, Claim::LibrarianSees(a, b, outsider)) => OneOf(vec![
            vec![
                Is(librarian, Good(Townsfolk(Librarian))),
                Is(a, Good(Outsider(outsider))),
            ],
            vec![
                Is(librarian, Good(Townsfolk(Librarian))),
                Is(b, Good(Outsider(outsider))),
            ],
            vec![
                Is(librarian, Good(Townsfolk(Librarian))),
                Is(a, Evil(Minion(Spy))),
            ],
            vec![
                Is(librarian, Good(Townsfolk(Librarian))),
                Is(b, Evil(Minion(Spy))),
            ],
            vec![IsPoisoned(librarian)],
            vec![IsDrunk(librarian, Librarian)],
        ]),

        ReportLog::OnTime(investigator, Claim::InvestigatorSees(a, b, minion)) => OneOf(vec![
            vec![
                Is(investigator, Good(Townsfolk(Investigator))),
                Is(a, Evil(Minion(minion))),
            ],
            vec![
                Is(investigator, Good(Townsfolk(Investigator))),
                Is(b, Evil(Minion(minion))),
            ],
            vec![
                Is(investigator, Good(Townsfolk(Investigator))),
                Is(a, Good(Outsider(Recluse))),
            ],
            vec![
                Is(investigator, Good(Townsfolk(Investigator))),
                Is(b, Good(Outsider(Recluse))),
            ],
            vec![IsPoisoned(investigator)],
            vec![IsDrunk(investigator, Investigator)],
        ]),

        // Fortune Teller True
        ReportLog::OnTime(ft, Claim::FortuneTellerLearns(a, b, true)) => OneOf(vec![
            vec![IsDrunk(ft, FortuneTeller)],
            vec![Is(ft, Good(Townsfolk(FortuneTeller))), IsPoisoned(ft)],
            vec![Is(ft, Good(Townsfolk(FortuneTeller))), IsRedHerring(a)],
            vec![Is(ft, Good(Townsfolk(FortuneTeller))), IsRedHerring(b)],
            vec![
                Is(ft, Good(Townsfolk(FortuneTeller))),
                Is(a, Evil(Demon(Imp))),
            ],
            vec![
                Is(ft, Good(Townsfolk(FortuneTeller))),
                Is(b, Evil(Demon(Imp))),
            ],
        ]),

        ReportLog::OnTime(ut, Claim::UndertakerSees(p, character)) => OneOf(vec![
            vec![IsDrunk(ut, Undertaker)],
            vec![Is(ut, Good(Townsfolk(Undertaker))), IsPoisoned(ut)],
            vec![Is(ut, Good(Townsfolk(Undertaker))), Is(p, character)],
            // TODO: Spy and Recluse can register falsely, but Is(p, character) might be enough if character is what they registered as.
        ]),

        ReportLog::OnTime(_, Claim::Executes(_)) => CompoundConstraint::AllOf(vec![]), // Execution is an event, doesn't directly constrain roles unless we have more logic.
        ReportLog::OnTime(_, Claim::PoisonerPoisons(_)) => CompoundConstraint::AllOf(vec![]),
        ReportLog::OnTime(_, Claim::ImpKills(_)) => CompoundConstraint::AllOf(vec![]),
        ReportLog::OnTime(_, Claim::MonkProtects(_)) => CompoundConstraint::AllOf(vec![]),

        _ => CompoundConstraint::AllOf(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Evil::{Demon, Minion};
    use crate::Good::{Outsider, Townsfolk};
    use crate::Demon::*;
    use crate::Minion::*;
    use crate::Outsider::*;
    use crate::Townsfolk::*;
    use Character::*;
    use Constraint::*;
    use Player::*;

    #[test]
    fn test_expect_washerwoman_sees_foo_or_bar_as_empath() {
        let ww = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(ww, Claim::WasherwomanSees(foo, bar, Empath));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), Good(Townsfolk(Washerwoman))),
                Is(Seat(2), Good(Townsfolk(Empath))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Washerwoman))),
                Is(Seat(3), Good(Townsfolk(Empath))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Washerwoman))),
                Is(Seat(2), Evil(Minion(Spy))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Washerwoman))),
                Is(Seat(3), Evil(Minion(Spy))),
            ],
            vec![IsPoisoned(Seat(1))],
            vec![IsDrunk(Seat(1), Washerwoman)],
        ]);
        assert_eq!(expect(reported), want);
    }

    #[test]
    fn test_expect_librarian_sees_foo_or_bar_as_drunk() {
        let lib = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(lib, Claim::LibrarianSees(foo, bar, Drunk));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), Good(Townsfolk(Librarian))),
                Is(Seat(2), Good(Outsider(Drunk))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Librarian))),
                Is(Seat(3), Good(Outsider(Drunk))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Librarian))),
                Is(Seat(2), Evil(Minion(Spy))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Librarian))),
                Is(Seat(3), Evil(Minion(Spy))),
            ],
            vec![IsPoisoned(Seat(1))],
            vec![IsDrunk(Seat(1), Librarian)],
        ]);
        assert_eq!(expect(reported), want);
    }

    #[test]
    fn test_expect_investigator_sees_foo_or_bar_as_baron() {
        let inv = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(inv, Claim::InvestigatorSees(foo, bar, Baron));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), Good(Townsfolk(Investigator))),
                Is(Seat(2), Evil(Minion(Baron))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Investigator))),
                Is(Seat(3), Evil(Minion(Baron))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Investigator))),
                Is(Seat(2), Good(Outsider(Recluse))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(Investigator))),
                Is(Seat(3), Good(Outsider(Recluse))),
            ],
            vec![IsPoisoned(Seat(1))],
            vec![IsDrunk(Seat(1), Investigator)],
        ]);
        assert_eq!(expect(reported), want);
    }

    #[test]
    fn test_expect_fortune_teller_yes() {
        let ft = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(ft, Claim::FortuneTellerLearns(foo, bar, true));
        let want = CompoundConstraint::OneOf(vec![
            vec![IsDrunk(Seat(1), FortuneTeller)],
            vec![
                Is(Seat(1), Good(Townsfolk(FortuneTeller))),
                IsPoisoned(Seat(1)),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(FortuneTeller))),
                IsRedHerring(Seat(2)),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(FortuneTeller))),
                IsRedHerring(Seat(3)),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(FortuneTeller))),
                Is(Seat(2), Evil(Demon(Imp))),
            ],
            vec![
                Is(Seat(1), Good(Townsfolk(FortuneTeller))),
                Is(Seat(3), Evil(Demon(Imp))),
            ],
        ]);
        assert_eq!(expect(reported), want);
    }
}

