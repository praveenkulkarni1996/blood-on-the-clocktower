use botc_core::Character::*;
use botc_core::Evil::*;
use botc_core::Good::*;
use botc_core::Minion::*;
use botc_core::Outsider::*;
use botc_core::Player::Seat;
use botc_core::Townsfolk::*;
use botc_core::{Claim, ReportLog, Time};
use botc_solver::poisoner_can_poison_one_person_only_if_alive;
use botc_solver::{Registry, constrain, setup};
use z3::Solver;

#[test]
fn test_chef_spy_recluse_washerwoman_empath_gets_chef_0() {
    let solver = Solver::new();
    let registry = Registry::new(5, Time::Night(1));

    // --- Add General Constraints ---
    solver.assert(&setup::assert_unique_player_tokens(&registry));

    // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
    solver.assert(registry.get(Seat(0), Good(Townsfolk(Chef))));
    solver.assert(registry.get(Seat(1), Evil(Minion(Spy))));
    solver.assert(registry.get(Seat(2), Good(Outsider(Recluse))));
    solver.assert(registry.get(Seat(3), Good(Townsfolk(Washerwoman))));
    solver.assert(registry.get(Seat(4), Good(Townsfolk(Empath))));

    // --- Initialize is_poisoned map for relevant players/times ---
    solver.assert(poisoner_can_poison_one_person_only_if_alive(&registry));

    // --- Add Chef's Claim Log ---
    // Chef (Player 0) claims a chef number of 0 at Time 0.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(0));
    let chef_constraint = constrain(&registry, &vec![], &chef_claim);
    solver.assert(&chef_constraint);

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_chef_spy_recluse_washerwoman_empath_gets_chef_1() {
    let solver = Solver::new();
    let registry = Registry::new(5, Time::Night(1));

    // --- Add General Constraints ---
    solver.assert(&setup::assert_unique_player_tokens(&registry));

    // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
    solver.assert(registry.get(Seat(0), Good(Townsfolk(Chef))));
    solver.assert(registry.get(Seat(1), Evil(Minion(Spy))));
    solver.assert(registry.get(Seat(2), Good(Outsider(Recluse))));
    solver.assert(registry.get(Seat(3), Good(Townsfolk(Washerwoman))));
    solver.assert(registry.get(Seat(4), Good(Townsfolk(Empath))));

    // --- Initialize is_poisoned map for relevant players/times ---
    solver.assert(poisoner_can_poison_one_person_only_if_alive(&registry));

    // --- Add Chef's Claim Log ---
    // Chef (Player 0) claims a chef number of 1 at Time 0.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(1));
    let chef_constraint = constrain(&registry, &vec![], &chef_claim);
    solver.assert(&chef_constraint);

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_chef_spy_recluse_washerwoman_empath_cannot_get_chef2() {
    let solver = Solver::new();
    let registry = Registry::new(5, Time::Night(1));

    // --- Add General Constraints ---
    solver.assert(&setup::assert_unique_player_tokens(&registry));

    // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
    solver.assert(registry.get(Seat(0), Good(Townsfolk(Chef))));
    solver.assert(registry.get(Seat(1), Evil(Minion(Spy))));
    solver.assert(registry.get(Seat(2), Good(Outsider(Recluse))));
    solver.assert(registry.get(Seat(3), Good(Townsfolk(Washerwoman))));
    solver.assert(registry.get(Seat(4), Good(Townsfolk(Empath))));

    // --- Initialize is_poisoned map for relevant players/times ---
    // Assert Chef is not poisoned.
    solver.assert(poisoner_can_poison_one_person_only_if_alive(&registry));

    // --- Add Chef's Claim Log ---
    // Chef (Player 0) claims a chef number of 2 at Time 0.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(2));
    let chef_constraint = constrain(&registry, &vec![], &chef_claim);
    solver.assert(&chef_constraint);

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
