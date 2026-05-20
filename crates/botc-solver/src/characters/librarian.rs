use crate::Registry;
use botc_core::{Outsider, Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn zero(r: &Registry, t: Time, alpha: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Librarian;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Librarian)))
        & crate::is_effective(r, alpha, t).implies(crate::setup::assert_player_count_by_predicate(
            r,
            crate::registers::must_outsider,
            0,
        ))
}

#[must_use]
pub fn sees(
    r: &Registry,
    t: Time,
    alpha: Player,
    bravo: Player,
    charlie: Player,
    outsider: Outsider,
) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::{Outsider as OutsiderGood, Townsfolk};
    use botc_core::Townsfolk::Librarian;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Librarian)))
        & crate::see_character_between_player_pair(
            r,
            alpha,
            bravo,
            charlie,
            Good(OutsiderGood(outsider)),
            t,
        )
}
