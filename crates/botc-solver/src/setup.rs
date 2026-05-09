#![warn(clippy::pedantic)]
use botc_core::Character;
use itertools::Itertools;
use strum::IntoEnumIterator;

/// # Panics
///
/// This panics if there are too few (< 0) or too many players (> 15).
#[must_use]
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

fn is_townsfolk(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::{Good, Townsfolk};
    let variants = Townsfolk::iter()
        .map(|t| r.get(p, Character::Good(Good::Townsfolk(t))).clone())
        .collect_vec();
    z3::ast::Bool::or(variants.as_slice())
}

fn is_outsider(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::{Good, Outsider};
    let variants = Outsider::iter()
        .map(|o| r.get(p, Character::Good(Good::Outsider(o))).clone())
        .collect_vec();
    z3::ast::Bool::or(variants.as_slice())
}

fn is_minion(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::{Evil, Minion};
    let variants = Minion::iter()
        .map(|m| r.get(p, Character::Evil(Evil::Minion(m))).clone())
        .collect_vec();
    z3::ast::Bool::or(variants.as_slice())
}

fn is_demon(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    use botc_core::{Demon, Evil};
    let variants = Demon::iter()
        .map(|d| r.get(p, Character::Evil(Evil::Demon(d))).clone())
        .collect_vec();
    z3::ast::Bool::or(variants.as_slice())
}

#[must_use]
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

/// Enforce that every player gets exactly one distinct-token.
/// NOTE: This logic cannot model Imp moves (Scarlet Woman, or via Starpass).
#[must_use]
pub fn assert_unique_player_tokens(r: &super::Registry) -> z3::ast::Bool {
    let players = (0..r.num_players)
        .map(botc_core::Player::Seat)
        .collect_vec();

    // List of variables for "Character $C has at-most one player".
    let character_has_one_player = Character::iter()
        .map(|c| z3::ast::atmost(players.iter().map(|&p| r.get(p, c)), 1))
        .collect_vec();

    // List of variables for "Player $P has at-least one token".
    let player_has_atleast_one_character = players
        .iter()
        .map(|&p| z3::ast::atleast(Character::iter().map(|c| r.get(p, c)), 1))
        .collect_vec();

    // List of variables for "Player $P has at-most one token".
    let player_has_atmost_one_character = players
        .iter()
        .map(|&p| z3::ast::atmost(Character::iter().map(|c| r.get(p, c)), 1))
        .collect_vec();

    z3::ast::Bool::and(character_has_one_player.as_slice())
        & z3::ast::Bool::and(player_has_atleast_one_character.as_slice())
        & z3::ast::Bool::and(player_has_atmost_one_character.as_slice())
}
