use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn yes(r: &Registry, t: Time, alpha: Player, bravo: Player, charlie: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::FortuneTeller;

    let bravo_is_red_herring = &r.is_red_herring[&bravo];
    let charlie_is_red_herring = &r.is_red_herring[&charlie];

    crate::player_claims_character(r, alpha, Good(Townsfolk(FortuneTeller)))
        & crate::is_effective(r, alpha, t).implies(
            Bool::from_bool(false)
                | bravo_is_red_herring
                | charlie_is_red_herring
                | crate::registers::can_demon(r, bravo)
                | crate::registers::can_demon(r, charlie),
        )
}

#[must_use]
pub fn no(r: &Registry, t: Time, alpha: Player, bravo: Player, charlie: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Townsfolk;
    use botc_core::Townsfolk::FortuneTeller;

    let bravo_is_red_herring = &r.is_red_herring[&bravo];
    let charlie_is_red_herring = &r.is_red_herring[&charlie];

    crate::player_claims_character(r, alpha, Good(Townsfolk(FortuneTeller)))
        & crate::is_effective(r, alpha, t).implies(
            Bool::from_bool(true)
                & !bravo_is_red_herring
                & !charlie_is_red_herring
                & !crate::registers::must_demon(r, bravo)
                & !crate::registers::must_demon(r, charlie),
        )
}
