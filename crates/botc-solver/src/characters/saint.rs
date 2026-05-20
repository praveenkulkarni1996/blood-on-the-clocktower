use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn constrain(r: &Registry, t: Time, saint: Player) -> Bool {
    use botc_core::Character::Good;
    use botc_core::Good::Outsider;
    use botc_core::Outsider::Saint;

    crate::player_claims_character(r, saint, Good(Outsider(Saint)))
        & !crate::is_effective(r, saint, t)
}
