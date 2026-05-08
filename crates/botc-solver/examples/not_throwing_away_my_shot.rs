use botc_core::Time::{Day, Night};
use botc_core::{Character, Claim, Good, Minion, Outsider, Player, ReportLog, Townsfolk};

/// Taken from Reddit, based on this puzzle by `/u/Not_Quite_Vertical`.
/// <https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/>
#[allow(unused_variables)]
fn setup_game() -> Vec<ReportLog> {
    use Claim::{
        Am, ChefGets, EmpathLearnsZero, InvestigatorSees, LibrarianZero, SlayerKillsDemon,
        WasherwomanSees,
    };
    use Minion::Baron;
    use Outsider::Recluse;
    use ReportLog::OnTime;
    use Townsfolk::Librarian;

    let matthew = Player::Seat(0); // Washerwoman
    let oscar = Player::Seat(1); // Librarian
    let josh = Player::Seat(2); // Empath
    let you = Player::Seat(3); // Slayer
    let aoife = Player::Seat(4); // Aoife
    let tom = Player::Seat(5); // Chef
    let sula = Player::Seat(6); // Investigator

    vec![
        OnTime(Night(1), matthew, WasherwomanSees(aoife, oscar, Librarian)),
        OnTime(Night(1), oscar, LibrarianZero),
        OnTime(Night(1), josh, EmpathLearnsZero(you, oscar)),
        OnTime(Day(1), you, SlayerKillsDemon(tom)),
        OnTime(Night(1), aoife, ChefGets(0)),
        OnTime(Night(1), tom, Am(Character::Good(Good::Outsider(Recluse)))),
        OnTime(Night(1), sula, InvestigatorSees(you, aoife, Baron)),
    ]
}

fn main() {
    let claim_logs = setup_game();

    let solver = z3::Solver::new();
    let registry = botc_solver::Registry::new(7, Day(2));

    let history = vec![];

    for &log in &claim_logs {
        let ast_constraint = botc_solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }

    solver.assert(botc_solver::game_setup(&registry));
    // Must demand that he is a recluse.
    solver.assert(registry.get(
        Player::Seat(5), /* tom */
        Character::Good(Good::Outsider(Outsider::Recluse)),
    ));

    dbg!(solver.check());
    let model = solver.get_model().expect("Failed to retrieve model");
    dbg!(model);
}
