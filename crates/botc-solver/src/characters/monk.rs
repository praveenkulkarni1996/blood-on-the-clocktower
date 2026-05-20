use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn constrain(r: &Registry, t: Time, alpha: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Monk;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Monk)))
        & !crate::is_effective(r, alpha, t)
}
