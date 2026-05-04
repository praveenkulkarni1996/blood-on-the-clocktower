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
fn test_10_player_baron_2_outsiders() {
    // NRB Live and Imp-Person
    // https://www.youtube.com/watch?v=m14N28Lq-jM
    let solver = define_solver(
        vec![
            Evil(Demon(Imp)),
            Evil(Minion(Poisoner)),
            Good(Outsider(Drunk)),
            Good(Outsider(Recluse)),
            Good(Townsfolk(Chef)),
            Good(Townsfolk(FortuneTeller)),
            Good(Townsfolk(Librarian)),
            Good(Townsfolk(Mayor)),
            Good(Townsfolk(Monk)),
        ]
        .as_slice(),
    );
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_nrb_live_and_imp_person_expect_fail() {
    // Same as above, but with Scarlet Woman instead of Baron.
    // Now the outsider count is invalid.
    let solver = define_solver(
        vec![
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
        ]
        .as_slice(),
    );
    assert_eq!(solver.check(), z3::SatResult::Unsat);
}

/// Taken from Reddit, based on this puzzle by /u/Not_Quite_Vertical.
/// https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/
#[test]
fn test_not_throwing_away_my_shot() {
    let solver = define_solver(
        vec![
            Evil(Demon(Imp)),              // bluffing as Good(Townsfolk(Washerwoman))
            Good(Outsider(Drunk)),         // appearing as Good(Townsfolk(Librarian)),
            Good(Townsfolk(Empath)),       // -
            Good(Townsfolk(Slayer)),       // -
            Evil(Minion(Baron)),           // bluffing as Good(Townsfolk(Chef))
            Good(Outsider(Recluse)),       // -
            Good(Townsfolk(Investigator)), // -
        ]
        .as_slice(),
    );
    assert_eq!(solver.check(), z3::SatResult::Sat);
}

#[test]
fn test_seven_player_base_zero_outsiders() {
    let solver = define_solver(
        vec![
            Evil(Demon(Imp)),               //
            Evil(Minion(Poisoner)),         //
            Good(Townsfolk(Empath)),        //
            Good(Townsfolk(FortuneTeller)), //
            Good(Townsfolk(Investigator)),  //
            Good(Townsfolk(Slayer)),        //
            Good(Townsfolk(Soldier)),       //
        ]
        .as_slice(),
    );
    assert_eq!(solver.check(), z3::SatResult::Sat);
}
