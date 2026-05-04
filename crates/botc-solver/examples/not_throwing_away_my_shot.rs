// WARNING: This code is not yet working.
use botc_core::Time::*;
use botc_core::*;

/// Taken from Reddit, based on this puzzle by /u/Not_Quite_Vertical.
/// https://www.reddit.com/r/BloodOnTheClocktower/comments/1f6lgjv/trouble_brewing_puzzle/
#[allow(unused_variables)]
fn setup_game() -> Vec<botc_core::ReportLog> {
    use Claim::*;
    use Minion::*;
    use Outsider::*;
    use ReportLog::*;
    use Townsfolk::*;

    let matthew = botc_core::Player::Seat(0); // Washerwoman
    let oscar = botc_core::Player::Seat(1); // Librarian
    let josh = botc_core::Player::Seat(2); // Empath
    let you = botc_core::Player::Seat(3); // Slayer
    let aoife = botc_core::Player::Seat(4); // Aoife
    let tom = botc_core::Player::Seat(5); // Chef
    let sula = botc_core::Player::Seat(6); // Investigator

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
    let registry = botc_solver::Registry::new(solver.get_context(), 7, Day(2));

    let history = vec![];

    for &log in claim_logs.iter() {
        let ast_constraint = botc_solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }

    botc_solver::atmost_one_player_can_be_red_herringed(&solver, &registry);
    botc_solver::atmost_one_player_can_be_poisoned(&solver, &registry);
    botc_solver::poisoner_can_poison_one_person_only_if_alive(&solver, &registry);
    botc_solver::poisoning_does_not_move_during_the_day(&solver, &registry);
    solver.assert(botc_solver::setup::assert_player_count_rules(&registry));
    solver.assert(botc_solver::setup::assert_unique_player_tokens(&registry));
    solver.assert(botc_solver::life::assert_life_until_death(&registry));
    solver.assert(registry.get(
        botc_core::Player::Seat(5), /*tom*/
        Character::Good(Good::Outsider(Outsider::Recluse)),
    ));

    dbg!(solver.check());
    let model = solver.get_model().expect("Failed to retrieve model");
    dbg!(model);
}
