use crate::Registry;
use botc_core::{Player, Time, Townsfolk};
use z3::ast::Bool;

#[must_use]
pub fn constrain(
    r: &Registry,
    t: Time,
    alpha: Player,
    bravo: Player,
    charlie: Player,
    townsfolk: Townsfolk,
) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk as TownsfolkGood;
    use botc_core::Townsfolk::Washerwoman;

    crate::player_claims_character(r, alpha, Good(TownsfolkGood(Washerwoman)))
        & crate::see_character_between_player_pair(
            r,
            alpha,
            bravo,
            charlie,
            Good(TownsfolkGood(townsfolk)),
            t,
        )
}
