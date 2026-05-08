#![deny(clippy::pedantic)]
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
fn test_librarian_sees_drunk_first() {
    // 6 players: 3T, 1O, 1M, 1D. Librarian sees Drunk at seat 1.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Outsider(Drunk)),      // seat-1
            Good(Townsfolk(Chef)),      // seat-2
            Good(Townsfolk(Empath)),    // seat-3
            Evil(Minion(Spy)),          // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::LibrarianSees(Seat(1), Seat(2), Drunk),
    );
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_librarian_sees_drunk_second() {
    // Librarian sees Drunk at seat 2.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Townsfolk(Chef)),      // seat-1
            Good(Outsider(Drunk)),      // seat-2
            Good(Townsfolk(Empath)),    // seat-3
            Evil(Minion(Spy)),          // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::LibrarianSees(Seat(1), Seat(2), Drunk),
    );
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_librarian_sees_spy_misregistration() {
    // Spy (seat 1) misregisters as Drunk to the Librarian.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Evil(Minion(Spy)),          // seat-1
            Good(Townsfolk(Chef)),      // seat-2
            Good(Townsfolk(Empath)),    // seat-3
            Good(Outsider(Butler)),     // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::LibrarianSees(Seat(1), Seat(2), Drunk),
    );
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_librarian_sees_unsat() {
    // Unsat: neither Chef (1) nor Empath (2) can register as Drunk.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Townsfolk(Chef)),      // seat-1
            Good(Townsfolk(Empath)),    // seat-2
            Evil(Minion(ScarletWoman)), // seat-3
            Good(Outsider(Butler)),     // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::LibrarianSees(Seat(1), Seat(2), Drunk),
    );
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_librarian_zero_5_player() {
    // 5 players: 3T, 0O, 1M, 1D. Librarian Zero is possible.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Townsfolk(Chef)),      // seat-1
            Good(Townsfolk(Empath)),    // seat-2
            Evil(Minion(Spy)),          // seat-3
            Evil(Demon(Imp)),           // seat-4
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::LibrarianZero);
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_librarian_zero_6_player_recluse() {
    // 6 players: 3T, 1O (Recluse), 1M, 1D. Recluse can misregister as non-outsider.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Townsfolk(Chef)),      // seat-1
            Good(Townsfolk(Empath)),    // seat-2
            Good(Outsider(Recluse)),    // seat-3
            Evil(Minion(Spy)),          // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::LibrarianZero);
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_librarian_zero_6_player_saint_unsat() {
    // 6 players: 3T, 1O (Saint), 1M, 1D. Saint MUST register as outsider -> Unsat.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Librarian)), // seat-0
            Good(Townsfolk(Chef)),      // seat-1
            Good(Townsfolk(Empath)),    // seat-2
            Good(Outsider(Saint)),      // seat-3
            Evil(Minion(Spy)),          // seat-4
            Evil(Demon(Imp)),           // seat-5
        ],
        Time::Night(1),
    );

    let librarian_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::LibrarianZero);
    solver.assert(constrain(&registry, &[], &librarian_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
