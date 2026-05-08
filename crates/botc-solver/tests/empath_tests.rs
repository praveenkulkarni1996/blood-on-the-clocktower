#![deny(clippy::pedantic)]

use botc_core::Character::{Evil, Good};
use botc_core::Demon::Imp;
use botc_core::Evil::{Demon, Minion};
use botc_core::Good::{Outsider, Townsfolk};
use botc_core::Minion::{ScarletWoman, Spy};
use botc_core::Outsider::{Drunk, Recluse};
use botc_core::Player::Seat;
use botc_core::Townsfolk::{Empath, FortuneTeller, Washerwoman};
use botc_core::{Claim, ReportLog, Time};
use botc_solver::constrain;

#[path = "define_solver.rs"]
mod define_solver;
use define_solver::define_solver_until;

#[test]
fn test_empath_learns_zero() {
    // 6-player circle: EMPATH(0) - FT(1) - DRUNK(2) - POISONER(3) - IMP(4) -
    // WASHERWOMAN(5). 3 Townsfolk, 1 Outsider, 1 Minion, 1 Demon.
    // Neighbors of 0 are 5 (Washerwoman - Good) and 1 (FT - Good). So Empath learns
    // 0.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Empath)),
            Good(Townsfolk(FortuneTeller)),
            Good(Outsider(Drunk)),
            Evil(Minion(ScarletWoman)),
            Evil(Demon(Imp)),
            Good(Townsfolk(Washerwoman)),
        ],
        Time::Night(1),
    );

    let empath_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsZero(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &empath_claim));
    assert_eq!(solver.check(), z3::SatResult::Sat);

    let empath_claim_wrong = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsOne(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &empath_claim_wrong));
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_empath_learns_one() {
    // 6-player circle: EMPATH(0) - POISONER(1) - FT(2) - DRUNK(3) - IMP(4) -
    // WASHERWOMAN(5). 3 Townsfolk, 1 Outsider, 1 Minion, 1 Demon.
    // Neighbors of 0 are 5 (Washerwoman - Good) and 1 (Poisoner - Evil). So Empath
    // learns 1.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Empath)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(FortuneTeller)),
            Good(Outsider(Drunk)),
            Evil(Demon(Imp)),
            Good(Townsfolk(Washerwoman)),
        ],
        Time::Night(1),
    );

    let empath_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsOne(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &empath_claim));
    assert_eq!(solver.check(), z3::SatResult::Sat);

    let empath_claim_wrong = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsZero(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &empath_claim_wrong));
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_empath_learns_two() {
    // 6-player circle: EMPATH(0) - POISONER(1) - FT(2) - DRUNK(3) - WASHERWOMAN(4)
    // - IMP(5). 3 Townsfolk, 1 Outsider, 1 Minion, 1 Demon.
    // Neighbors of 0 are 5 (Imp - Evil) and 1 (Poisoner - Evil). So Empath learns
    // 2.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Empath)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(FortuneTeller)),
            Good(Outsider(Drunk)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let empath_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsTwo(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &empath_claim));
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_empath_spy_misregistration() {
    // 6-player circle: EMPATH(0) - SPY(1) - FT(2) - DRUNK(3) - WASHERWOMAN(4) -
    // IMP(5). 3 Townsfolk, 1 Outsider, 1 Minion, 1 Demon.
    // Neighbors of 0 are 5 (Imp - Evil) and 1 (Spy - Evil, can misregister as
    // Good).
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Empath)),
            Evil(Minion(Spy)),
            Good(Townsfolk(FortuneTeller)),
            Good(Outsider(Drunk)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    // Spy can register as Good (Empath sees 1) or Evil (Empath sees 2).
    solver.push();
    let claim_one = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsOne(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &claim_one));
    assert_eq!(solver.check(), z3::SatResult::Sat);
    solver.pop(1);

    solver.push();
    let claim_two = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsTwo(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &claim_two));
    assert_eq!(solver.check(), z3::SatResult::Sat);
    solver.pop(1);

    solver.push();
    let claim_zero = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsZero(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &claim_zero));
    // Cannot be zero because Imp is permanently evil.
    assert_eq!(solver.check(), z3::SatResult::Unsat);
    solver.pop(1);
}

#[test]
fn test_empath_recluse_misregistration() {
    // 6-player circle: EMPATH(0) - RECLUSE(1) - FT(2) - POISONER(3) -
    // WASHERWOMAN(4) - IMP(5). 3 Townsfolk, 1 Outsider, 1 Minion, 1 Demon.
    // Neighbors of 0 are 5 (Imp - Evil) and 1 (Recluse - Good, can misregister as
    // Evil).
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Empath)),
            Good(Outsider(Recluse)),
            Good(Townsfolk(FortuneTeller)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(Washerwoman)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    solver.push();
    let claim_one = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsOne(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &claim_one));
    assert_eq!(solver.check(), z3::SatResult::Sat);
    solver.pop(1);

    solver.push();
    let claim_two = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::EmpathLearnsTwo(Seat(5), Seat(1)),
    );
    solver.assert(constrain(&registry, &[], &claim_two));
    assert_eq!(solver.check(), z3::SatResult::Sat);
    solver.pop(1);
}
