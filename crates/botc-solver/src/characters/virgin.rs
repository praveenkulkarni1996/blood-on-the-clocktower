use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn kills_townsfolk(r: &Registry, t: Time, alpha: Player, nominator: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Virgin;

    crate::player_must_character(r, alpha, Good(Townsfolk(Virgin)))
        & crate::is_effective(r, alpha, t)
        & crate::registers::can_townsfolk(r, nominator)
}

#[must_use]
pub fn misses(r: &Registry, t: Time, virgin: Player, nominator: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Virgin;

    crate::player_claims_character(r, virgin, Good(Townsfolk(Virgin)))
        & crate::is_effective(r, virgin, t).implies(!crate::registers::must_townsfolk(r, nominator))
}
