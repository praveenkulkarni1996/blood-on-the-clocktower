use botc_core::Character::*;
use botc_core::Demon::*;
use botc_core::Evil::*;
use botc_core::Good::*;
use botc_core::Minion::*;
use botc_core::Outsider::*;
use botc_core::Player::Seat;
use botc_core::Time;
use botc_core::Townsfolk::*;
use botc_solver::Registry;
use z3::Solver;

fn define_solver(tokens: &[botc_core::Character]) -> Solver {
    let solver = Solver::new();
    let registry = Registry::new(solver.get_context(), tokens.len(), Time::Day(1));
    solver.assert(botc_solver::setup::assert_player_count_rules(&registry));
    solver.assert(botc_solver::setup::assert_unique_player_tokens(&registry));

    for index in 0..tokens.len() {
        solver.assert(registry.get(Seat(index as i32), tokens[index]));
    }

    solver
}

#[test]
fn test_10p_baron() {
    // NRB Live and Imp-Person
    // https://www.youtube.com/watch?v=m14N28Lq-jM
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(Poisoner)),
        Good(Outsider(Drunk)),
        Good(Outsider(Recluse)),
        Good(Townsfolk(Chef)),
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Librarian)),
        Good(Townsfolk(Mayor)),
        Good(Townsfolk(Monk)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_10p_no_baron() {
    // Same as above, but with Scarlet Woman instead of Baron.
    // Now the outsider count is invalid.
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(ScarletWoman)),
        Evil(Minion(Poisoner)),
        Good(Outsider(Drunk)),
        Good(Outsider(Recluse)),
        Good(Townsfolk(Chef)),
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Librarian)),
        Good(Townsfolk(Mayor)),
        Good(Townsfolk(Monk)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

/// Taken from Reddit, based on this puzzle by /u/Not_Quite_Vertical.
/// https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/
#[test]
fn test_7p_baron() {
    let solver = define_solver(&[
        Evil(Demon(Imp)),              // bluffing as Good(Townsfolk(Washerwoman))
        Good(Outsider(Drunk)),         // appearing as Good(Townsfolk(Librarian)),
        Good(Townsfolk(Empath)),       // -
        Good(Townsfolk(Slayer)),       // -
        Evil(Minion(Baron)),           // bluffing as Good(Townsfolk(Chef))
        Good(Outsider(Recluse)),       // -
        Good(Townsfolk(Investigator)), // -
    ]);
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_7p_zero_outsider() {
    let solver = define_solver(&[
        Evil(Demon(Imp)),               //
        Evil(Minion(Poisoner)),         //
        Good(Townsfolk(Empath)),        //
        Good(Townsfolk(FortuneTeller)), //
        Good(Townsfolk(Investigator)),  //
        Good(Townsfolk(Slayer)),        //
        Good(Townsfolk(Soldier)),       //
    ]);
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_7p_too_many_minions() {
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(Poisoner)),
        Evil(Minion(ScarletWoman)), // 2nd minion
        Good(Townsfolk(Empath)),
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Investigator)),
        Good(Townsfolk(Slayer)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_7p_too_many_demons() {
    // 7 players, 2 imps.
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Demon(Imp)), // This will actually fail unique token check too, but let's see.
        Evil(Minion(Poisoner)),
        Good(Townsfolk(Empath)),
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Investigator)),
        Good(Townsfolk(Slayer)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_8p_wrong_outsider_count() {
    // 8 players should have 1 outsider.
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(Poisoner)),
        Good(Townsfolk(Chef)),
        Good(Townsfolk(Empath)),
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Investigator)),
        Good(Townsfolk(Slayer)),
        Good(Townsfolk(Soldier)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_duplicate_townsfolk() {
    // 7 players, two Chefs.
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(Poisoner)),
        Good(Townsfolk(Chef)),
        Good(Townsfolk(Chef)), // Duplicate
        Good(Townsfolk(FortuneTeller)),
        Good(Townsfolk(Investigator)),
        Good(Townsfolk(Slayer)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

#[test]
fn test_4p_invalid() {
    // 4 players is not a valid game size in our BASE_SETUP.
    let solver = define_solver(&[
        Evil(Demon(Imp)),
        Evil(Minion(Poisoner)),
        Good(Townsfolk(Chef)),
        Good(Townsfolk(Empath)),
    ]);
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}
