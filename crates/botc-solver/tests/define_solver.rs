use botc_core::{Character, Player::Seat, Time};
use botc_solver::{game_setup, Registry};
use z3::Solver;

/// High-level helper that creates a solver with a *complete* game setup
/// (player counts, unique tokens, life/death, poisoning & red-herring rules)
/// for the supplied seating arrangement. Returns only the Solver.
/// This produces the beautiful one-line tests seen in setup_tests.rs.
#[allow(dead_code)]
pub fn define_solver(tokens: &[Character]) -> Solver {
    define_solver_until(tokens, Time::Day(1)).0
}

/// Same as `define_solver`, but returns the Registry as well so that
/// claim-based tests can append `constrain(&registry, ...)` after the call.
/// Uses the full game_setup rule set.
pub fn define_solver_until(tokens: &[Character], until: Time) -> (Solver, Registry) {
    let solver = Solver::new();
    let registry = Registry::new(tokens.len(), until);

    solver.assert(game_setup(&registry));

    for (index, &token) in tokens.iter().enumerate() {
        solver.assert(registry.get(Seat(index as i32), token));
    }

    (solver, registry)
}
