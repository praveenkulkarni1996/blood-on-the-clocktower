/// This is only supporting Trouble Brewing.
use strum::EnumIter;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    Unresolved,
    Seat(i32),
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum Townsfolk {
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

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outsider {
    Butler,
    Saint,
    Recluse,
    Drunk,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Minion {
    Baron,
    Poisoner,
    ScarletWoman,
    Spy,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Demon {
    Imp,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Character {
    GoodTownsfolk(Townsfolk),
    GoodOutsider(Outsider),
    EvilMinion(Minion),
    EvilDemon(Demon),
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LiveOrDie {
    Lives,
    Dies,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Info {
    Neither,
    One,
    Both,
}

// This is the player visible log.
// Some of this might come out publicily.

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Log {
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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum ReportLog {
    OnTime(Player, Log),
}

fn main() {
    println!("Hello, world!");
    let alice = Player::Seat(1);
    let bob = Player::Seat(2);
    let log = Log::FortuneTellerLearns(alice, bob, true);
    println!("{:?}", &log);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum Constraint {
    Is(Player, Character),
    IsPoisoned(Player),
    IsDrunk(Player, Townsfolk),
    IsRedHerring(Player),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum CompoundConstraint {
    AllOf(Vec<Constraint>),
    OneOf(Vec<Vec<Constraint>>),
}

fn expect(report_log: ReportLog) -> CompoundConstraint {
    use Character::*;
    use CompoundConstraint::OneOf;
    use Constraint::*;
    use Demon::*;
    use Townsfolk::*;

    return match report_log {
        // Washerwoman
        ReportLog::OnTime(washerwoman, Log::WasherwomanSees(a, b, townsfolk)) => OneOf(vec![
            vec![
                Is(washerwoman, GoodTownsfolk(Washerwoman)),
                Is(a, GoodTownsfolk(townsfolk)),
            ],
            vec![
                Is(washerwoman, GoodTownsfolk(Washerwoman)),
                Is(b, GoodTownsfolk(townsfolk)),
            ],
            vec![
                Is(washerwoman, GoodTownsfolk(Washerwoman)),
                Is(a, EvilMinion(Minion::Spy)),
            ],
            vec![
                Is(washerwoman, GoodTownsfolk(Washerwoman)),
                Is(b, EvilMinion(Minion::Spy)),
            ],
            vec![IsPoisoned(washerwoman)],
            vec![IsDrunk(washerwoman, Washerwoman)],
        ]),

        ReportLog::OnTime(librarian, Log::LibrarianSees(a, b, outsider)) => OneOf(vec![
            vec![
                Is(librarian, GoodTownsfolk(Librarian)),
                Is(a, GoodOutsider(outsider)),
            ],
            vec![
                Is(librarian, GoodTownsfolk(Librarian)),
                Is(b, GoodOutsider(outsider)),
            ],
            vec![
                Is(librarian, GoodTownsfolk(Librarian)),
                Is(a, EvilMinion(Minion::Spy)),
            ],
            vec![
                Is(librarian, GoodTownsfolk(Librarian)),
                Is(b, EvilMinion(Minion::Spy)),
            ],
            vec![IsPoisoned(librarian)],
            vec![IsDrunk(librarian, Librarian)],
        ]),

        ReportLog::OnTime(investigator, Log::InvestigatorSees(a, b, minion)) => OneOf(vec![
            vec![
                Is(investigator, GoodTownsfolk(Investigator)),
                Is(a, EvilMinion(minion)),
            ],
            vec![
                Is(investigator, GoodTownsfolk(Investigator)),
                Is(b, EvilMinion(minion)),
            ],
            vec![
                Is(investigator, GoodTownsfolk(Investigator)),
                Is(a, GoodOutsider(Outsider::Recluse)),
            ],
            vec![
                Is(investigator, GoodTownsfolk(Investigator)),
                Is(b, GoodOutsider(Outsider::Recluse)),
            ],
            vec![IsPoisoned(investigator)],
            vec![IsDrunk(investigator, Investigator)],
        ]),

        // Fortune Teller True
        ReportLog::OnTime(ft, Log::FortuneTellerLearns(a, b, true)) => OneOf(vec![
            vec![IsDrunk(ft, FortuneTeller)],
            vec![Is(ft, GoodTownsfolk(FortuneTeller)), IsPoisoned(ft)],
            vec![Is(ft, GoodTownsfolk(FortuneTeller)), IsRedHerring(a)],
            vec![Is(ft, GoodTownsfolk(FortuneTeller)), IsRedHerring(b)],
            vec![Is(ft, GoodTownsfolk(FortuneTeller)), Is(a, EvilDemon(Imp))],
            vec![Is(ft, GoodTownsfolk(FortuneTeller)), Is(b, EvilDemon(Imp))],
        ]),

        ReportLog::OnTime(ut, Log::UndertakerSees(p, character)) => OneOf(vec![
            vec![IsDrunk(ut, Undertaker)],
            vec![Is(ut, GoodTownsfolk(Undertaker)), IsPoisoned(ut)],
            vec![Is(ut, GoodTownsfolk(Undertaker)), Is(p, character)],
            // TODO: Spy and Recluse
        ]),

        _ => todo!(),
    };
}

#[cfg(test)] // Only compiles when running 'cargo test'
mod tests {
    use super::*; // Import names from the outer scope
    use Character::*;
    use Constraint::*;
    use Demon::*;
    use Minion::*;
    use Outsider::*;
    use Player::*;
    use Townsfolk::*;

    #[test]
    fn test_expect_washerwoman_sees_foo_or_bar_as_empath() {
        let ww = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(ww, Log::WasherwomanSees(foo, bar, Empath));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), GoodTownsfolk(Washerwoman)),
                Is(Seat(2), GoodTownsfolk(Empath)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Washerwoman)),
                Is(Seat(3), GoodTownsfolk(Empath)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Washerwoman)),
                Is(Seat(2), EvilMinion(Spy)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Washerwoman)),
                Is(Seat(3), EvilMinion(Spy)),
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

        let reported = ReportLog::OnTime(lib, Log::LibrarianSees(foo, bar, Drunk));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), GoodTownsfolk(Librarian)),
                Is(Seat(2), GoodOutsider(Drunk)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Librarian)),
                Is(Seat(3), GoodOutsider(Drunk)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Librarian)),
                Is(Seat(2), EvilMinion(Spy)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Librarian)),
                Is(Seat(3), EvilMinion(Spy)),
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

        let reported = ReportLog::OnTime(inv, Log::InvestigatorSees(foo, bar, Baron));
        let want = CompoundConstraint::OneOf(vec![
            vec![
                Is(Seat(1), GoodTownsfolk(Investigator)),
                Is(Seat(2), EvilMinion(Baron)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Investigator)),
                Is(Seat(3), EvilMinion(Baron)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Investigator)),
                Is(Seat(2), GoodOutsider(Recluse)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(Investigator)),
                Is(Seat(3), GoodOutsider(Recluse)),
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

        let reported = ReportLog::OnTime(ft, Log::FortuneTellerLearns(foo, bar, true));
        let want = CompoundConstraint::OneOf(vec![
            vec![IsDrunk(Seat(1), FortuneTeller)],
            vec![
                Is(Seat(1), GoodTownsfolk(FortuneTeller)),
                IsPoisoned(Seat(1)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(FortuneTeller)),
                IsRedHerring(Seat(2)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(FortuneTeller)),
                IsRedHerring(Seat(3)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(FortuneTeller)),
                Is(Seat(2), EvilDemon(Imp)),
            ],
            vec![
                Is(Seat(1), GoodTownsfolk(FortuneTeller)),
                Is(Seat(3), EvilDemon(Imp)),
            ],
        ]);
        assert_eq!(expect(reported), want);
    }

    // TODO:
    // Live and Imp-Person | NRB Play Blood On The Clocktower
    // https://www.youtube.com/watch?v=m14N28Lq-jM
}
