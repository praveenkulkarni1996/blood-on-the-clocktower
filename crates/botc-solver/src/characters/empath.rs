use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn learns_zero(r: &Registry, t: Time, alpha: Player, bravo: Player, charlie: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Empath;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Empath)))
        & crate::is_effective(r, alpha, t)
            .implies(crate::registers::can_good(r, bravo) & crate::registers::can_good(r, charlie))
}

#[must_use]
pub fn learns_one(r: &Registry, t: Time, alpha: Player, bravo: Player, charlie: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Empath;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Empath)))
        & crate::is_effective(r, alpha, t).implies(
            (crate::registers::can_good(r, bravo) & crate::registers::can_evil(r, charlie))
                | (crate::registers::can_evil(r, bravo) & crate::registers::can_good(r, charlie)),
        )
}

#[must_use]
pub fn learns_two(r: &Registry, t: Time, alpha: Player, bravo: Player, charlie: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Empath;

    crate::player_claims_character(r, alpha, Good(Townsfolk(Empath)))
        & crate::is_effective(r, alpha, t)
            .implies(crate::registers::can_evil(r, bravo) & crate::registers::can_evil(r, charlie))
}
