use z3::Solver;

use botc::Time::*;
use botc::*;

#[allow(unused_variables)]
fn setup_game() -> Vec<botc::ReportLog> {
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
        // Sullivan (Poisoner) selects a player to neutralize.
        // OnTime(Night(1), sullivan, Claim::PoisonerPoisons(isaac)),
        // Adam (Chef) learns how many pairs of evil players are sitting next to each other.
        // Could learn both 1 or 2.
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
        // 1. Sullivan (Poisoner) poisons Isaac (the Monk).
        // OnTime(Night(2), sullivan, Claim::PoisonerPoisons(isaac)),
        // 2. Isaac (Monk) protects Carly (the Fortune Teller). (Invalidated by poison)
        // OnTime(Night(2), isaac, Claim::MonkProtects(carly)),
        NightKilled(Night(2), isaac),
        //
        // Carly (Fortune Teller) selects herself (red-herring) and Sullivan.
        OnTime(Night(2), carly, Claim::FortuneTellerYes(carly, sullivan)),
        // 5. Brooke (Undertaker - Drunk) sees Adam as the chef.
        OnTime(
            Night(2),
            brooke,
            Claim::UndertakerSees(adam, Character::Good(Good::Townsfolk(Townsfolk::Chef))),
        ),
        // DAY-2
        // Adam (the Chef) is nominated and executed.
        DayExecutes(Day(2), adam),
        // NIGHT-3
        // 1. Sullivan (Poisoner) poisons Carly (the Fortune Teller). (Disables Red Herring)
        // OnTime(Day(2), sullivan, Claim::PoisonerPoisons(carly)),
        // 2. Isaac (Monk) protects Adam. (Wait, Adam is dead - protecting a corpse/sinking kill?)
        // In the game, Isaac protected someone, but Blair killed Brooke.
        NightKilled(Night(3), brooke),
        // 3. Carly (Fortune Teller) selects herself and Dom. (No - Poisoned)
        OnTime(Night(3), carly, Claim::FortuneTellerNo(carly, dom)),
        // 4. Blair (Imp) kills Brooke (the Drunk).
        // OnTime(Night(3), blair, Claim::ImpKills(brooke)),
        // DAY-3
        // Isaac (the Monk) is nominated and executed.
        DayExecutes(Day(3), isaac),
        // NIGHT-4
        // The Final Five (Blair, Sullivan, Ollie, Carly, Dom) remain.
        // There were no deaths this night.

        // DAY-4
        // Carly (the Fortune Teller) is executed.
        // Result: Leaving only 4 alive (Blair, Sullivan, Ollie, Dom).
        DayExecutes(Day(4), carly),
        // NIGHT-5
        // 1. Sullivan (Poisoner) poisons Dom (the Mayor).
        // Consequence: Disables the "bounce" safety net.
        // OnTime(Night(5), sullivan, Claim::PoisonerPoisons(dom)),
        // 2. Blair (Imp) kills Dom (the Mayor).
        // Result: Dom dies, leaving only the 3 Evil players.
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

    let solver = Solver::new();
    let registry = solver::Registry::new(solver.get_context(), 10, Day(6));

    let history = vec![];

    for &log in claim_logs.iter() {
        let ast_constraint = solver::constrain(&registry, &history, &log);
        solver.assert(ast_constraint);
    }

    solver::player_has_exactly_one_character(&solver, &registry);
    solver::character_has_at_most_one_player(&solver, &registry);
    solver::atmost_one_player_can_be_red_herringed(&solver, &registry);
    solver::atmost_one_player_can_be_poisoned(&solver, &registry);
    solver::fix_minion_count(&solver, &registry, 2);
    solver::poisoner_can_poison_one_person_only_if_alive(&solver, &registry);
    solver::poisoning_does_not_move_during_the_day(&solver, &registry);
    solver::mark_characters_not_in_play(
        &solver,
        &registry,
        &vec![
            Character::Evil(Evil::Minion(Minion::ScarletWoman)),
            Character::Evil(Evil::Minion(Minion::Spy)),
        ],
    );

    dbg!(solver.check());
    let model = solver.get_model().expect("Failed to retrieve model");
    dbg!(model);
}
