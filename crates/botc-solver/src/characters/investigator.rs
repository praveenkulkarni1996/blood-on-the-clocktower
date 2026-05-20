use crate::Registry;
use botc_core::{Minion, Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn constrain(
    r: &Registry,
    t: Time,
    alpha: Player,
    bravo: Player,
    charlie: Player,
    minion: Minion,
) -> Bool {
    use botc_core::Character::Evil;
    use botc_core::Evil::Minion as MinionEvil;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Investigator;

    crate::player_claims_character(
        r,
        alpha,
        botc_core::Character::Good(Townsfolk(Investigator)),
    ) & crate::see_character_between_player_pair(
        r,
        alpha,
        bravo,
        charlie,
        Evil(MinionEvil(minion)),
        t,
    )
}
