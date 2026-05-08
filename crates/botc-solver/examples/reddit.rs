use botc_core::Time::{Day, Night};
use botc_core::{Character, Claim, Good, Minion, Outsider, Player, ReportLog, Townsfolk};

/// Taken from Reddit, based on this puzzle by `/u/ExcessiveUsernames`.
/// <https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/>
///
/// This seems to allow for many different solutions.
#[allow(unused_variables)]
fn setup_game() -> Vec<ReportLog> {
    use Claim::{
        Am, ChefGets, EmpathLearnsOne, FortuneTellerYes, InvestigatorSees, LibrarianSees,
        RavenkeeperSees, SlayerMisses, VirginKillsTownsfolk, WasherwomanSees,
    };
    use Minion::Poisoner;
    use Outsider::{Drunk, Saint};
    use ReportLog::OnTime;
    use Townsfolk::{Empath, Investigator};

    // Cyclic seating order starting from Erika
    let erika = Player::Seat(0);
    let ailidh = Player::Seat(1);
    let john = Player::Seat(2);
    let rachael = Player::Seat(3);
    let you = Player::Seat(4);
    let edward = Player::Seat(5);
    let derek = Player::Seat(6);
    let kyle = Player::Seat(7);
    let linda = Player::Seat(8);
    let diane = Player::Seat(9);

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

    for &log in &claim_logs {
        let ast_constraint = botc_solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }

    solver.assert(botc_solver::game_setup(&registry));

    dbg!(solver.check());
    let model = solver.get_model().expect("Failed to retrieve model");
    dbg!(model);
}
