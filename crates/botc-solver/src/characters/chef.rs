use crate::Registry;
use botc_core::{Player, Time};
use itertools::Itertools;
use z3::ast::Bool;

#[must_use]
pub fn constrain(r: &Registry, t: Time, alpha: Player, num: i32) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Chef;

    let zero = z3::ast::Int::from_i64(0);
    let one = z3::ast::Int::from_i64(1);

    let must_pairs: Vec<z3::ast::Int> = (0..r.num_players)
        .circular_tuple_windows::<(_, _)>()
        .map(|(p1, p2)| {
            crate::must_evil_pair(r, Player::Seat(p1), Player::Seat(p2)).ite(&one, &zero)
        })
        .collect();

    let can_pairs: Vec<z3::ast::Int> = (0..r.num_players)
        .circular_tuple_windows::<(_, _)>()
        .map(|(p1, p2)| {
            crate::can_evil_pair(r, Player::Seat(p1), Player::Seat(p2)).ite(&one, &zero)
        })
        .collect();

    let chef_num = z3::ast::Int::from_i64(i64::from(num));
    let chef_min = z3::ast::Int::add(&must_pairs.iter().collect::<Vec<_>>()).le(&chef_num);
    let chef_max = z3::ast::Int::add(&can_pairs.iter().collect::<Vec<_>>()).ge(&chef_num);
    let chef_correct = chef_min & chef_max;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Chef)))
        & crate::is_effective(r, alpha, t).implies(chef_correct)
}
