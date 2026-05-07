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
fn test_washerwoman_sees_first_correctly_registers() {
    // 6-player circle: WASHERWOMAN - CHEF - EMPATH - SPY - RECLUSE - IMP
    // WasherwomanSees(first, second, ...) works because |first| is correctly registering.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Washerwoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let washerwoman_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::WasherwomanSees(Seat(1), Seat(2), Chef),
    );
    solver.assert(constrain(&registry, &[], &washerwoman_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_washerwoman_sees_second_correctly_registers() {
    // 6-player circle: WASHERWOMAN - CHEF - EMPATH - SPY - RECLUSE - IMP
    // WasherwomanSees(first, second, ...) works because |second| is correctly registering.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Washerwoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let washerwoman_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::WasherwomanSees(Seat(2), Seat(1), Chef),
    );
    solver.assert(constrain(&registry, &[], &washerwoman_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_washerwoman_sees_via_spy_misregistration() {
    // 6-player circle: WASHERWOMAN - CHEF - EMPATH - SPY - RECLUSE - IMP
    // WasherwomanSees(first, second, ...) works because either |first| or |second| is a Spy who can misregister.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Washerwoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Evil(Minion(Spy)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let washerwoman_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::WasherwomanSees(Seat(2), Seat(3), Chef),
    );
    solver.assert(constrain(&registry, &[], &washerwoman_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}
