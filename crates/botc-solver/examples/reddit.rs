use botc_core::Time::*;
use botc_core::*;

/// Taken from Reddit, based on this puzzle by /u/ExcessiveUsernames.
/// https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/
///
/// This seems to allow for many different solutions.
#[allow(unused_variables)]
fn setup_game() -> Vec<botc_core::ReportLog> {
    // Cyclic seating order starting from Erika
    let erika = botc_core::Player::Seat(0);
    let ailidh = botc_core::Player::Seat(1);
    let john = botc_core::Player::Seat(2);
    let rachael = botc_core::Player::Seat(3);
    let you = botc_core::Player::Seat(4);
    let edward = botc_core::Player::Seat(5);
    let derek = botc_core::Player::Seat(6);
    let kyle = botc_core::Player::Seat(7);
    let linda = botc_core::Player::Seat(8);
    let diane = botc_core::Player::Seat(9);

    use Claim::*;
    use Minion::*;
    use Outsider::*;
    use ReportLog::*;
    use Townsfolk::*;

    vec![
        OnTime(Night(1), erika, Am(Character::Good(Good::Outsider(Saint)))),
        OnTime(Night(1), ailidh, LibrarianSees(derek, rachael, Drunk)),
        OnTime(Night(1), rachael, FortuneTellerYes(diane, linda)),
        OnTime(Night(1), you, InvestigatorSees(edward, erika, Poisoner)),
        OnTime(Night(1), edward, ChefGets(1)),
        OnTime(Night(1), kyle, WasherwomanSees(ailidh, linda, Empath)),
        OnTime(Night(1), linda, EmpathLearnsOne(diane, kyle)),
        //
        OnTime(Day(1), john, VirginKillsTownsfolk(ailidh)),
        OnTime(Day(1), derek, SlayerMisses(diane)),
        //
        OnTime(
            Night(2),
            diane,
            RavenkeeperSees(you, Character::Good(Good::Townsfolk(Investigator))),
        ),
    ]
}

fn main() {
    let claim_logs = setup_game();

    let solver = z3::Solver::new();
    let registry = botc_solver::Registry::new(10, Day(2));

    let history = vec![];

    for &log in claim_logs.iter() {
        let ast_constraint = botc_solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }

    solver.assert(botc_solver::game_setup(&registry));

    dbg!(solver.check());
    let model = solver.get_model().expect("Failed to retrieve model");
    dbg!(model);
}
