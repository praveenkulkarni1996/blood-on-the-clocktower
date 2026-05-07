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
fn test_investigator_sees_scarlet_woman_first() {
    // 6-player game (valid 3T/1O/1M/1D):
    // INVESTIGATOR - SCARLET_WOMAN - CHEF - EMPATH - RECLUSE - IMP
    // InvestigatorSees sees ScarletWoman as the *first* argument (correct registration).
    // Only one minion (ScarletWoman). No Poisoner is ever used in these tests.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Investigator)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let investigator_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::InvestigatorSees(Seat(1), Seat(2), ScarletWoman),
    );
    solver.assert(constrain(&registry, &[], &investigator_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_investigator_sees_baron_second() {
    // Valid 6-player *Baron* game (1T / 3O / 1M / 1D):
    // INVESTIGATOR - BUTLER - BARON - SAINT - RECLUSE - IMP
    //
    // Because Baron is present, the counts must be the baron-modified setup.
    // InvestigatorSees sees Baron as the *second* argument of the pair (seat 2).
    // No Poisoner is ever used in any of these tests.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Investigator)),
            Good(Outsider(Butler)),
            Evil(Minion(Baron)),
            Good(Outsider(Saint)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let investigator_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::InvestigatorSees(Seat(1), Seat(2), Baron),
    );
    solver.assert(constrain(&registry, &[], &investigator_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_investigator_sees_poisoner_recluse_misregistration_sat() {
    // 6-player game (valid 3T/1O/1M/1D):
    // INVESTIGATOR - SCARLET_WOMAN - CHEF - EMPATH - RECLUSE - IMP
    //
    // Investigator claims "I see seats 2 and 4 as Poisoner".
    // A Recluse exists at seat 4 (the *second* argument) and misregisters as Poisoner
    // via the `as_token` rule for Evil minions (Recluse counts as any Evil token).
    // We *never* include an actual Poisoner in any of these tests.
    // The claim is therefore possible precisely because of the Recluse → Sat.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Investigator)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let investigator_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::InvestigatorSees(Seat(2), Seat(4), Poisoner),
    );
    solver.assert(constrain(&registry, &[], &investigator_claim));

    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_investigator_sees_unsat_info() {
    // 6-player game (valid 3T/1O/1M/1D):
    // INVESTIGATOR - SCARLET_WOMAN - CHEF - EMPATH - RECLUSE - IMP
    //
    // Investigator claims "I see seats 2 and 3 as Poisoner".
    // A Recluse exists who *could* misregister as a Poisoner to an Investigator,
    // but the Recluse is at seat 4 (not part of the claimed pair).
    // Seats 2 (Chef) and 3 (Empath) are both good townsfolk — neither is a Poisoner
    // nor the Recluse. We *never* include an actual Poisoner in any of these tests.
    // The specific claim is therefore impossible → Unsat.
    let (solver, registry) = define_solver_until(
        &[
            Good(Townsfolk(Investigator)),
            Evil(Minion(ScarletWoman)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(Empath)),
            Good(Outsider(Recluse)),
            Evil(Demon(Imp)),
        ],
        Time::Night(1),
    );

    let investigator_claim = ReportLog::OnTime(
        Time::Night(1),
        Seat(0),
        Claim::InvestigatorSees(Seat(2), Seat(3), Poisoner),
    );
    solver.assert(constrain(&registry, &[], &investigator_claim));

    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
