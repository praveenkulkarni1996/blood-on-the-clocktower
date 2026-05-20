use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn kills_demon(r: &Registry, t: Time, slayer: Player, target: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Slayer;

    crate::player_must_character(r, slayer, Good(Townsfolk(Slayer)))
        & crate::is_effective(r, slayer, t)
        & crate::registers::can_demon(r, target)
}

#[must_use]
pub fn misses(r: &Registry, t: Time, slayer: Player, target: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::Slayer;

    crate::player_claims_character(r, slayer, Good(Townsfolk(Slayer)))
        & crate::is_effective(r, slayer, t).implies(!crate::registers::must_demon(r, target))
}
