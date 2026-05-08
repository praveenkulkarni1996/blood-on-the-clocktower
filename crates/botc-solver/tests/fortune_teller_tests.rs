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
fn test_fortune_teller_yes_demon() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerYes(Seat(1), Seat(5)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_fortune_teller_yes_red_herring() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerYes(Seat(1), Seat(2)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_fortune_teller_yes_recluse() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Townsfolk(Empath)),        // seat-2
            Good(Outsider(Recluse)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // Force Recluse (3) and Chef (1) not to be Red Herring.
    solver.assert(registry.is_red_herring(Seat(3)).not());
    solver.assert(registry.is_red_herring(Seat(1)).not());

    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerYes(Seat(1), Seat(3)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_fortune_teller_yes_unsat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // Force Spy (4) to be Red Herring to ensure 1 and 2 aren't.
    solver.assert(registry.is_red_herring(Seat(4)));

    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerYes(Seat(1), Seat(2)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_fortune_teller_no_normal() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // Force Red Herring to be seat 4 (Spy).
    solver.assert(registry.is_red_herring(Seat(4)));

    // FT sees Chef (1) and Drunk (2). Gets NO.
    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerNo(Seat(1), Seat(2)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_fortune_teller_no_demon_unsat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // FT sees Chef (1) and Imp (5). Gets NO.
    // Since Imp is Demon, must get YES -> UNSAT.
    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerNo(Seat(1), Seat(5)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_fortune_teller_no_red_herring_unsat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Outsider(Drunk)),          // seat-2
            Good(Townsfolk(Empath)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // Force Chef (1) to be Red Herring.
    solver.assert(registry.is_red_herring(Seat(1)));

    // FT sees Chef (1) and Drunk (2). Gets NO.
    // Chef is Red Herring, must get YES -> UNSAT.
    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerNo(Seat(1), Seat(2)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_fortune_teller_no_recluse_sat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(FortuneTeller)), // seat-0
            Good(Townsfolk(Chef)),          // seat-1
            Good(Townsfolk(Empath)),        // seat-2
            Good(Outsider(Recluse)),        // seat-3
            Evil(Minion(Spy)),              // seat-4
            Evil(Demon(Imp)),               // seat-5
        ],
        Time::Night(1),
    );

    // Force Recluse (3) and Chef (1) not to be Red Herring.
    solver.assert(registry.is_red_herring(Seat(3)).not());
    solver.assert(registry.is_red_herring(Seat(1)).not());

    // FT sees Chef (1) and Recluse (3). Gets NO.
    // Recluse *might* register as Demon, so getting NO is perfectly fine (SAT).
    let ft_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::FortuneTellerNo(Seat(1), Seat(3)),
    );
    solver.assert(constrain(&registry, &[], &ft_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}
