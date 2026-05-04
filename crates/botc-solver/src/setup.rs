#![warn(clippy::pedantic)]
use itertools::Itertools;

/// # Panics
///
/// This panics if there are too few (< 0) or too many players (> 15).
pub fn assert_player_count_rules(r: &super::Registry) -> z3::ast::Bool {
    let num_players = usize::try_from(r.num_players).unwrap();
    let base_setup: PlayerCount = BASE_SETUP[num_players];
    let baron_setup: PlayerCount = PlayerCount {
        townsfolk: base_setup.townsfolk - 2,
        outsider: base_setup.outsider + 2,
        minion: base_setup.minion,
        demon: base_setup.demon,
    };

    let has_base_setup = assert_setup(r, base_setup);
    let has_baron_setup = assert_setup(r, baron_setup);
    let baron_present = assert_player_count_by_predicate(r, is_baron, 1);
    let baron_absent = assert_player_count_by_predicate(r, is_baron, 0);

    (has_base_setup.clone() ^ has_baron_setup.clone())
        & has_base_setup.implies(baron_absent)
        & has_baron_setup.implies(baron_present)
}

#[derive(Debug, Copy, Clone)]
struct PlayerCount {
    townsfolk: i8,
    outsider: i8,
    minion: i8,
    demon: i8,
}

#[rustfmt::skip]
static BASE_SETUP: &[PlayerCount] = &[
    // 0-4 players is not a valid game.
    PlayerCount { townsfolk: 0, outsider: 0, minion: 0, demon: 0}, // 0 invalid
    PlayerCount { townsfolk: 0, outsider: 0, minion: 0, demon: 0}, // 1 invalid
    PlayerCount { townsfolk: 0, outsider: 0, minion: 0, demon: 0}, // 2 invalid
    PlayerCount { townsfolk: 0, outsider: 0, minion: 0, demon: 0}, // 3 invalid
    PlayerCount { townsfolk: 0, outsider: 0, minion: 0, demon: 0}, // 4 invalid
    // 5-player and 6-player are Teensyville.
    // The minion and demon are not told each other.
    PlayerCount { townsfolk: 3, outsider: 0, minion: 1, demon: 1}, // 5
    PlayerCount { townsfolk: 3, outsider: 1, minion: 1, demon: 1}, // 6
    // 1-minion evil team.
    PlayerCount { townsfolk: 5, outsider: 0, minion: 1, demon: 1}, // 7
    PlayerCount { townsfolk: 5, outsider: 1, minion: 1, demon: 1}, // 8
    PlayerCount { townsfolk: 5, outsider: 2, minion: 1, demon: 1}, // 9
    // 2 minion evil team.
    PlayerCount { townsfolk: 7, outsider: 0, minion: 2, demon: 1}, // 10
    PlayerCount { townsfolk: 7, outsider: 1, minion: 2, demon: 1}, // 11
    PlayerCount { townsfolk: 7, outsider: 2, minion: 2, demon: 1}, // 12
    // 3-minion evil team.
    PlayerCount { townsfolk: 9, outsider: 0, minion: 3, demon: 1}, // 13
    PlayerCount { townsfolk: 9, outsider: 1, minion: 3, demon: 1}, // 14
    PlayerCount { townsfolk: 9, outsider: 2, minion: 3, demon: 1}, // 15
];

fn is_baron(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::Character::Evil;
    use botc_core::Evil::Minion;
    use botc_core::Minion::Baron;

    r.get(p, Evil(Minion(Baron))).clone()
}

fn is_minion(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::Character::Evil;
    use botc_core::Evil::Minion;
    use botc_core::Minion::{Baron, Poisoner, ScarletWoman, Spy};

    r.get(p, Evil(Minion(Baron)))
        | r.get(p, Evil(Minion(Poisoner)))
        | r.get(p, Evil(Minion(ScarletWoman)))
        | r.get(p, Evil(Minion(Spy)))
}

fn is_demon(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::Character::Evil;
    use botc_core::Demon::Imp;
    use botc_core::Evil::Demon;

    r.get(p, Evil(Demon(Imp))).clone()
}

fn is_townsfolk(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::{
        Chef, Empath, FortuneTeller, Investigator, Librarian, Mayor, Monk, Ravenkeeper, Slayer,
        Soldier, Undertaker, Virgin, Washerwoman,
    };

    r.get(p, Good(Townsfolk(Washerwoman)))
        | r.get(p, Good(Townsfolk(Librarian)))
        | r.get(p, Good(Townsfolk(Investigator)))
        | r.get(p, Good(Townsfolk(Chef)))
        | r.get(p, Good(Townsfolk(Empath)))
        | r.get(p, Good(Townsfolk(FortuneTeller)))
        | r.get(p, Good(Townsfolk(Undertaker)))
        | r.get(p, Good(Townsfolk(Monk)))
        | r.get(p, Good(Townsfolk(Ravenkeeper)))
        | r.get(p, Good(Townsfolk(Virgin)))
        | r.get(p, Good(Townsfolk(Slayer)))
        | r.get(p, Good(Townsfolk(Soldier)))
        | r.get(p, Good(Townsfolk(Mayor)))
}

fn is_outsider(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Outsider;
    use botc_core::Outsider::{Butler, Drunk, Recluse, Saint};

    r.get(p, Good(Outsider(Butler)))
        | r.get(p, Good(Outsider(Drunk)))
        | r.get(p, Good(Outsider(Recluse)))
        | r.get(p, Good(Outsider(Saint)))
}

pub fn assert_player_count_by_predicate(
    r: &super::Registry,
    predicate: fn(&super::Registry, botc_core::Player) -> z3::ast::Bool,
    count: i8,
) -> z3::ast::Bool {
    let zero = z3::ast::Int::from_i64(0);
    let one = z3::ast::Int::from_i64(1);
    let player_satisfies_predicate: Vec<z3::ast::Int> = (0..r.num_players)
        .map(botc_core::Player::Seat)
        .map(|p| predicate(r, p).ite(&one, &zero))
        .collect_vec();

    let want = z3::ast::Int::from_i64(i64::from(count));
    z3::ast::Int::add(player_satisfies_predicate.iter().as_slice()).eq(want)
}

fn assert_setup(r: &super::Registry, setup: PlayerCount) -> z3::ast::Bool {
    assert_player_count_by_predicate(r, is_townsfolk, setup.townsfolk)
        & assert_player_count_by_predicate(r, is_demon, setup.demon)
        & assert_player_count_by_predicate(r, is_minion, setup.minion)
        & assert_player_count_by_predicate(r, is_outsider, setup.outsider)
}
