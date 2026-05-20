use crate::Registry;
use botc_core::{Character, Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn constrain(
    r: &Registry,
    t: Time,
    alpha: Player,
    bravo: Player,
    character: Character,
) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Ravenkeeper;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Ravenkeeper)))
        & crate::player_sees_other_players_character(r, alpha, bravo, character, t)
}
