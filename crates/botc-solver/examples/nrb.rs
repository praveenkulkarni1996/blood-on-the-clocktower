use botc_core::Time::*;
use botc_core::*;

#[allow(unused_variables)]
fn setup_game() -> Vec<botc_core::ReportLog> {
    // Live and Imp-Person
    // https://www.youtube.com/watch?v=m14N28Lq-jM

    // Cyclic seating order starting from Ollie
    let ollie: Player = Player::Seat(0); // Baron
    let carly = Player::Seat(1); // Fortune Teller
    let john: Player = Player::Seat(2); // Librarian
    let brooke = Player::Seat(3); // Drunk / Undertaker
    let adam = Player::Seat(4); // Chef
    let isaac = Player::Seat(5); // Monk
    let sullivan = Player::Seat(6); // Poisoner
    let laurie = Player::Seat(7); // Recluse
    let dom = Player::Seat(8); // Mayor
    let blair = Player::Seat(9); // Imp

    use Claim::*;
    use Outsider::*;
    use ReportLog::*;

    vec![
        // NIGHT-1
        //
        // Sullivan (Poisoner) selects a player to neutralize.
        DocumentOnly(Night(1), sullivan, Claim::PoisonerPoisons(isaac)),
        //
        // Adam (Chef) learns how many pairs of evil players are sitting next to each other.
        // Could learn both 1 (BARON-IMP) or 2 (POISONER-RECLUSE).
        OnTime(Night(1), adam, ChefGets(1)),
        //
        // 3. John (Librarian) learns that either Adam or Laurie is the Recluse.
        OnTime(Night(1), john, LibrarianSees(adam, laurie, Recluse)),
        //
        // 4. Carly (Fortune Teller) selects adam and RECLUSE laurie, and gets YES.
        OnTime(Night(1), carly, Claim::FortuneTellerYes(adam, laurie)),
        // 5. Brooke (Drunk) believes she is the Undertaker.
        // 6. Blair (Imp) learns her Minions (Ollie, Sullivan) and Demon Bluffs (Investigator,
        //    Empath, Saint).

        // DAY-1
        // Adam (the Chef) is executed.
        DayExecutes(Time::Day(1), adam),
        // NIGHT-2
        // 1. Sullivan (Poisoner) poisons Brooke (the Undertaker).
        DocumentOnly(Night(2), sullivan, Claim::PoisonerPoisons(brooke)),
        // 2. Isaac (Monk) protects Carly (the Fortune Teller). (Invalidated by poison)
        // OnTime(Night(2), isaac, Claim::MonkProtects(carly)),
        NightKilled(Night(2), isaac),
        // Carly (Fortune Teller) selects herself (red-herring) and Sullivan.
        OnTime(Night(2), carly, Claim::FortuneTellerYes(carly, sullivan)),
        // 5. Brooke (Undertaker - Drunk) sees Adam as the chef.
        OnTime(
            Night(2),
            brooke,
            Claim::UndertakerSees(adam, Character::Good(Good::Townsfolk(Townsfolk::Chef))),
        ),
        // DAY-2
        // Brooke (Drunk-Undertaker) is nominated and executed.
        DayExecutes(Day(2), brooke),
        // NIGHT-3
        // 1. Sullivan (Poisoner) poisons Carly (the Fortune Teller). (Disables Red Herring)
        DocumentOnly(Night(3), sullivan, Claim::PoisonerPoisons(carly)),
        // John : Librarian is killed in the night.
        NightKilled(Night(3), john),
        // 3. Carly (Fortune Teller) selects herself and Dom. (No - Poisoned)
        OnTime(Night(3), carly, Claim::FortuneTellerNo(carly, dom)),
        // DAY-3
        // Laurie (the RECLUSE) is nominated and executed.
        DayExecutes(Day(3), laurie),
        //
        // NIGHT-4
        // Carly is poisoned and killed.
        DocumentOnly(Night(4), sullivan, Claim::PoisonerPoisons(carly)),
        NightKilled(Night(3), carly),
        // NIGHT-5
        // 1. Sullivan (Poisoner) poisons Dom (the Mayor).
        // Consequence: Disables the "bounce" safety net.
        // 2. Blair (Imp) kills Dom (the Mayor).
        // Result: Dom dies, leaving only the 3 Evil players.
        DocumentOnly(Night(5), sullivan, Claim::PoisonerPoisons(dom)),
        NightKilled(Night(5), dom),
        // Claims
        OnTime(
            Night(1),
            ollie,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Soldier))), /* Lying. Ollie is
                                                                              * a Baron. */
        ),
        OnTime(
            Night(1),
            carly,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::FortuneTeller))),
        ),
        OnTime(
            Night(1),
            john,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Librarian))),
        ),
        OnTime(
            Night(1),
            brooke,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Undertaker))),
        ),
        OnTime(
            Night(1),
            adam,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Chef))),
        ),
        OnTime(
            Night(1),
            isaac,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Monk))),
        ),
        OnTime(
            Night(1),
            sullivan,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Empath))), /* Lying: Actually
                                                                             * poisoner */
        ),
        OnTime(
            Night(1),
            laurie,
            Claim::Am(Character::Good(Good::Outsider(Outsider::Recluse))),
        ),
        OnTime(
            Night(1),
            dom,
            Claim::Am(Character::Good(Good::Townsfolk(Townsfolk::Mayor))),
        ),
        OnTime(
            Night(1),
            blair,
            Claim::Am(Character::Good(Good::Outsider(Outsider::Saint))), // Lying
        ),
        // let ollie: Player = Player::Seat(0); // Baron
        // let carly = Player::Seat(1); // Fortune Teller
        // let john: Player = Player::Seat(2); // Librarian
        // let brooke = Player::Seat(3); // Drunk / Undertaker
        // let adam = Player::Seat(4); // Chef
        // let isaac = Player::Seat(5); // Monk
        // let sullivan = Player::Seat(6); // Poisoner
        // let laurie = Player::Seat(7); // Recluse
        // let dom = Player::Seat(8); // Mayor
        // let blair = Player::Seat(9); // Imp
    ]
}

fn main() {
    let claim_logs = setup_game();

    use botc_core::Player::Seat;
    use botc_core::{Character, Demon, Evil, Minion};

    // ------------------------------------------------------------------
    // Single-solver incremental solving using push/pop scopes.
    //
    // Instead of creating a fresh Solver + Registry for every possible
    // Demon candidate (which re-encodes the entire problem 10 times),
    // we create ONE solver and ONE registry, assert the permanent
    // constraints only once, and then use Z3 scopes (push/pop) to
    // temporarily force each player to be the Imp.
    //
    // This is much more efficient and demonstrates proper use of
    // Z3's incremental solving capabilities for model enumeration.
    // ------------------------------------------------------------------

    // Create solver and registry only ONCE
    let solver = z3::Solver::new();
    let registry = botc_solver::Registry::new(10, Day(6));

    // Assert the permanent part of the problem only ONCE
    // (all player claims + game setup rules + forbidden characters).
    let history = vec![];
    for &log in claim_logs.iter() {
        let ast_constraint = botc_solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }
    solver.assert(botc_solver::game_setup(&registry));
    solver.assert(botc_solver::mark_characters_not_in_play(
        &registry,
        &vec![
            Character::Evil(Evil::Minion(Minion::ScarletWoman)),
            Character::Evil(Evil::Minion(Minion::Spy)),
        ],
    ));

    // Now enumerate every possible Imp using lightweight scopes.
    // Each push/pop pair adds and then removes only the "this player
    // is the Imp" assertion. The heavy constraints stay in the solver.
    for seat in 0..10 {
        let demon_candidate = Seat(seat);

        solver.push(); // enter a new temporary scope

        // Force this specific player to be the Demon (Imp)
        let demon_imp = Character::Evil(Evil::Demon(Demon::Imp));
        let imp_var = registry.get(demon_candidate, demon_imp);
        solver.assert(imp_var.clone());

        match solver.check() {
            z3::SatResult::Sat => {
                println!("=== SAT WORLD: Seat({}) is the Imp ===", seat);
                if let Some(model) = solver.get_model() {
                    // Use the explicit path into the debugging module
                    botc_solver::debugging::print_true_variables(&model, &registry);
                }
            }
            z3::SatResult::Unsat => {
                println!("=== UNSAT: Seat({}) cannot be the Imp ===", seat);
            }
            other => {
                println!("=== UNKNOWN: Seat({}) => {:?}", seat, other);
            }
        }

        solver.pop(1); // backtrack — removes only the forcing literal above
    }
}
