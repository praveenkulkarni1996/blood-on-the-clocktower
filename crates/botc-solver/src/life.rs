#![warn(clippy::pedantic)]
use itertools::{Itertools, iproduct};
use z3::ast::Bool;

pub fn assert_life_until_death(r: &super::Registry) -> z3::ast::Bool {
    let players = (0..r.num_players)
        .map(botc_core::Player::Seat)
        .collect_vec();
    let timeslots = botc_core::TimeIterator::new(r.until).collect_vec();
    let timepair_players = iproduct!(timeslots.iter().tuple_windows(), players.iter());

    let rules = timepair_players
        .map(|((previous, current), player)| -> Bool {
            let is_alive_prev: &Bool = &r.is_alive[player][previous];
            let is_alive_curr: &Bool = &r.is_alive[player][current];

            let is_dead_prev: Bool = r.is_alive[player][previous].not();
            let is_dead_curr: Bool = r.is_alive[player][current].not();

            let alive_rule: Bool = is_alive_curr.implies(is_alive_prev);
            let death_rule: Bool = is_dead_prev.implies(is_dead_curr);

            alive_rule & death_rule
        })
        .collect_vec();

    Bool::and(rules.iter().collect_vec().as_slice())
}
