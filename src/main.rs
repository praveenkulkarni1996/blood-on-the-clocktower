/// This is only supporting Trouble Brewing.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Player {
    Unresolved,
    Seat(i32),
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

// This is the player visible log.
// Some of this might come out publicily.

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
enum Log {
    WasherwomanSees(Player, Player, Townsfolk), // https://wiki.bloodontheclocktower.com/Washerwoman
    LibrarianSees(Player, Player, Outsider),    // https://wiki.bloodontheclocktower.com/Librarian
    InvestigatorSees(Player, Player, Minion), // https://wiki.bloodontheclocktower.com/Investigator
    Chef(i32),                                // https://wiki.bloodontheclocktower.com/Chef
    Empath(Player, Player, i32),              // https://wiki.bloodontheclocktower.com/Empath
    FortuneTeller(Player, Player, bool), // https://wiki.bloodontheclocktower.com/Fortune_Teller
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
    let log = Log::FortuneTeller(alice, bob, true);
    println!("{:?}", &log);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum Constraint {
    Is(Player, Character),
    IsPoisoned(Player),
    IsDrunk(Player, Character),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
enum CompoundConstraint {
    AllOf(Vec<Constraint>),
    OneOf(Vec<Vec<Constraint>>),
}


fn expect(report_log : ReportLog) ->  CompoundConstraint {
    use Character::GoodTownsfolk;
    use Character::EvilMinion;
    use Constraint::Is;
    use Constraint::IsDrunk;
    use Constraint::IsPoisoned;
    use CompoundConstraint::OneOf;
    use Townsfolk::Washerwoman;

    return match report_log {
        // Washerwoman
        ReportLog::OnTime(washerwoman, Log::WasherwomanSees(a, b, townsfolk)) =>  OneOf(
            vec![
                vec![Is(washerwoman, GoodTownsfolk(Washerwoman)), Is(a, GoodTownsfolk(townsfolk))],
                vec![Is(washerwoman, GoodTownsfolk(Washerwoman)), Is(b, GoodTownsfolk(townsfolk))],
                vec![Is(washerwoman, GoodTownsfolk(Washerwoman)), Is(a, EvilMinion(Minion::Spy))],
                vec![Is(washerwoman, GoodTownsfolk(Washerwoman)), Is(b, EvilMinion(Minion::Spy))],
                vec![IsPoisoned(washerwoman)],
                vec![IsDrunk(washerwoman, GoodTownsfolk(Washerwoman))],
            ]),
        _ => todo!(),
    }
}


#[cfg(test)] // Only compiles when running 'cargo test'
mod tests {
    use super::*; // Import names from the outer scope
    use Character::*;
    use Townsfolk::*;
    use Minion::*;
    use Player::*;
    use Constraint::*;
    

    #[test]
    fn test_expect_washerwoman_sees_foo_or_bar_as_empath() {
        let ww = Seat(1);
        let foo = Seat(2);
        let bar = Seat(3);

        let reported = ReportLog::OnTime(ww, Log::WasherwomanSees(foo, bar, Empath));
        let want = CompoundConstraint::OneOf(vec![
             vec![Is(Seat(1), GoodTownsfolk(Washerwoman)), Is(Seat(2), GoodTownsfolk(Empath))], 
             vec![Is(Seat(1), GoodTownsfolk(Washerwoman)), Is(Seat(3), GoodTownsfolk(Empath))], 
             vec![Is(Seat(1), GoodTownsfolk(Washerwoman)), Is(Seat(2), EvilMinion(Spy))], 
             vec![Is(Seat(1), GoodTownsfolk(Washerwoman)), Is(Seat(3), EvilMinion(Spy))], 
             vec![IsPoisoned(Seat(1))], 
             vec![IsDrunk(Seat(1), GoodTownsfolk(Washerwoman))],
        ]);
        assert_eq!(expect(reported), want);
    }

    // TODO:
    // Live and Imp-Person | NRB Play Blood On The Clocktower
    // https://www.youtube.com/watch?v=m14N28Lq-jM

}
