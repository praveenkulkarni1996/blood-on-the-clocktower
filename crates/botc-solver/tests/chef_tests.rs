use botc_core::Character::*;
use botc_core::Demon::*;
use botc_core::Evil::*;
use botc_core::Good::*;
use botc_core::Minion::*;
use botc_core::Outsider::*;
use botc_core::Player::Seat;
use botc_core::Townsfolk::*;
use botc_core::{Claim, ReportLog, Time};
use botc_solver::constrain;

#[path = "define_solver.rs"]
mod define_solver;
use define_solver::define_solver_until;

#[test]
fn test_chef_spy_recluse_washerwoman_imp_empath_reports_chef_0() {
    // 6-player circle: CHEF - SPY - RECLUSE - WASHERWOMAN - IMP - EMPATH
    // The two fixed evils are isolated, so a truthful Chef can report 0 (when Recluse does
    // not register evil) or 1 (when it does). This test covers the 0 case.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Chef)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
            Good(Townsfolk(Empath)),
        ],
        Time::Night(1),
    );

    // Chef claims "0" — this is consistent with the seating under the full rules.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(0));
    solver.assert(constrain(&registry, &[], &chef_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_chef_spy_recluse_washerwoman_imp_empath_reports_chef_1() {
    // 6-player circle: CHEF - SPY - RECLUSE - WASHERWOMAN - IMP - EMPATH
    // Depending on how the Recluse registers, a Chef can sometimes learn "1".
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Chef)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
            Good(Townsfolk(Empath)),
        ],
        Time::Night(1),
    );

    // Chef claims "1" — this remains possible under the full rules.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(1));
    solver.assert(constrain(&registry, &[], &chef_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_chef_spy_recluse_washerwoman_imp_empath_reports_chef_2() {
    // 6-player circle: CHEF - SPY - RECLUSE - WASHERWOMAN - IMP - EMPATH
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Chef)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
            Good(Townsfolk(Empath)),
        ],
        Time::Night(1),
    );

    // Chef claims "2". With only two isolated evils there is no way to get a count of 2.
    // This remains correctly Unsat under the full rules.
    let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(2));
    solver.assert(constrain(&registry, &[], &chef_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
