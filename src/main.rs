#[allow(dead_code)]
#[derive(Debug)]
enum Player {
    Unresolved,
    Seat(i32),
}

#[allow(dead_code)]
#[derive(Debug)]
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
#[derive(Debug)]
enum Outsider {
    Butler,
    Saint,
    Recluse,
    Drunk,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Minion {
    Baron,
    Poisoner,
    ScarletWoman,
    Spy,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Demon {
    Imp,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Character {
    GoodTownsfolk(Townsfolk),
    GoodOutsider(Outsider),
    EvilMinion(Minion),
    EvilDemon(Demon),
}

#[allow(dead_code)]
#[derive(Debug)]
enum LiveOrDie {
    Lives,
    Dies,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Log {
    WasherwomanSees(Player, Player, Townsfolk), // https://wiki.bloodontheclocktower.com/Washerwoman
    LibrarianSees(Player, Player, Outsider),    // https://wiki.bloodontheclocktower.com/Librarian
    InvestigatorSees(Player, Player, Minion), // https://wiki.bloodontheclocktower.com/Investigator
    ChefSees(i32),                            // https://wiki.bloodontheclocktower.com/Chef
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

fn main() {
    println!("Hello, world!");
    let alice = Player::Seat(1);
    let bob = Player::Seat(2);
    let log = Log::FortuneTeller(alice, bob, true);
    println!("{:?}", &log);
}
