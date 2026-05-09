#![deny(clippy::pedantic)]

use botc_core::Character::{Evil, Good};
use botc_core::Demon::Imp;
use botc_core::Evil::{Demon, Minion};
use botc_core::Good::Outsider;
use botc_core::Good::Townsfolk;
use botc_core::Minion::Spy;
use botc_core::Outsider::{Drunk, Recluse};
use botc_core::Player::Seat;
use botc_core::Townsfolk::{Mayor, Monk, Slayer};
use botc_core::{Claim, ReportLog, Time};
use botc_solver::constrain;

#[path = "define_solver.rs"]
mod define_solver;
use define_solver::define_solver_until;

#[test]
fn test_one_slayer_shot_is_sat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerKillsDemon(Seat(4)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_slayer_cannot_kill_non_demonic_players() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerKillsDemon(Seat(2)));
    solver.assert(constrain(&registry, &[], &shot));
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_slayer_miss_is_sat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Slayer (Seat 0) shots Monk (Seat 2) and misses.
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerMisses(Seat(2)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_drunk_slayer_miss_on_nondemon_is_sat() {
    // 6 players: 3T, 1O, 1M, 1D
    let (solver, registry) = define_solver_until(
        &[
            Good(Outsider(Drunk)), // seat 0 - claims Slayer but is Drunk
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Good(Townsfolk(Slayer)), // seat 3
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Drunk (Seat 0) shots Monk (Seat 2) and misses.
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerMisses(Seat(2)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_slayer_kills_recluse_is_sat() {
    // 6 players: 3T, 1O, 1M, 1D
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Good(Outsider(Recluse)), // seat 3
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Slayer (Seat 0) shots Recluse (Seat 3) and kills them.
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerKillsDemon(Seat(3)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_slayer_misses_demon_when_drunk_is_sat() {
    // 6 players: 3T, 1O, 1M, 1D
    let (solver, registry) = define_solver_until(
        &[
            Good(Outsider(Drunk)), // seat 0 - claims Slayer but is Drunk
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Good(Townsfolk(Slayer)), // seat 3
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Drunk (Seat 0) shots Demon (Seat 5) and misses.
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerMisses(Seat(5)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_slayer_misses_recluse_is_sat() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Good(Outsider(Recluse)),
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Slayer (Seat 0) shots Recluse (Seat 3) and misses.
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerMisses(Seat(3)));
    solver.assert(constrain(&registry, &[], &shot));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_drunk_slayer_cannot_kill_demon() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Outsider(Drunk)), // seat 0
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Good(Townsfolk(Slayer)),
            Evil(Minion(Spy)),
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Drunk Slayer (Seat 0) attempts to kill Demon (Seat 5).
    let shot = ReportLog::OnTime(Time::Day(1), Seat(0), Claim::SlayerKillsDemon(Seat(5)));
    solver.assert(constrain(&registry, &[], &shot));

    // Should be UNSAT because SlayerKillsDemon requires is_effective.
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_spy_cannot_slayer_kill() {
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Slayer)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
            Evil(Minion(Spy)), // seat 3
            Evil(Demon(Imp)),
        ],
        Time::Day(2),
    );

    // Spy (Seat 3) attempts to kill Demon (Seat 4).
    let shot = ReportLog::OnTime(Time::Day(1), Seat(3), Claim::SlayerKillsDemon(Seat(4)));
    solver.assert(constrain(&registry, &[], &shot));

    // Should be UNSAT because SlayerKillsDemon requires
    // player_must_character(Slayer).
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
