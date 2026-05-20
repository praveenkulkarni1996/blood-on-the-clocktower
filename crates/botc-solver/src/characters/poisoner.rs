use crate::Registry;
use botc_core::{Player, Time};
use z3::ast::Bool;

#[must_use]
pub fn constrain(r: &Registry, t: Time, poisoner: Player, victim: Player) -> Bool {
    use botc_core::Character::Evil;
    use botc_core::Evil::Minion;
    use botc_core::Minion::Poisoner;

    crate::player_must_character(r, poisoner, Evil(Minion(Poisoner)))
        & r.is_poisoned[&victim][&t].clone()
}
