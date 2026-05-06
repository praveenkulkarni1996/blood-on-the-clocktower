//! Debugging and exploration utilities.
//!
//! These helpers are **not** part of the stable public API.
//! They exist to help understand and debug the solver's behavior
//! (e.g. inspecting multiple satisfying models / "worlds").
//!
//! Currently used by the `nrb` example to print all true variables
//! across different Demon candidates.

use crate::Registry;

use botc_core::Player::Seat;
use botc_core::{Character, TimeIterator};

/// Prints every Boolean variable that the `Registry` created
/// which evaluates to `true` under the given Z3 model.
///
/// This produces a (potentially long) list of everything that is
/// true in a particular satisfying assignment — roles, life state,
/// poisoning, red herrings, etc.
///
/// Intended for exploration only.
pub fn print_true_variables(model: &z3::Model, r: &Registry) {
    println!("--- TRUE VARIABLES ---");

    // 1. Character assignments (who is what role)
    for seat in 0..r.num_players {
        let p = Seat(seat);
        for c in Character::iter() {
            let var = r.get(p, c);
            if let Some(val) = model.eval(var, true)
                && val.as_bool() == Some(true)
            {
                println!("  {:?} is {:?}", p, c);
            }
        }
    }

    // 2. Red herrings
    for seat in 0..r.num_players {
        let p = Seat(seat);
        if let Some(var) = r.is_red_herring.get(&p)
            && let Some(val) = model.eval(var, true)
            && val.as_bool() == Some(true)
        {
            println!("  {:?} is_red_herring", p);
        }
    }

    // 3. Poisoned states over relevant times
    // (alive state is no longer printed to reduce noise)
    for time in TimeIterator::new(r.until) {
        for seat in 0..r.num_players {
            let p = Seat(seat);

            if let Some(var) = r.is_poisoned.get(&p).and_then(|m| m.get(&time))
                && let Some(val) = model.eval(var, true)
                && val.as_bool() == Some(true)
            {
                println!("  {:?} poisoned_at {:?}", p, time);
            }
        }
    }

    println!("--- END TRUE VARIABLES ---");
}
