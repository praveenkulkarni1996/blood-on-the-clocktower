use botc::*;

#[allow(unused_variables)]
fn setup_game() {
    // Live and Imp-Person
    // https://www.youtube.com/watch?v=m14N28Lq-jM

    // Cyclic seating order starting from Ollie
    let ollie = Player::Seat(1); // Baron
    let carly = Player::Seat(2); // Fortune Teller
    let john = Player::Seat(3); // Librarian
    let brooke = Player::Seat(4); // Drunk / Undertaker
    let adam = Player::Seat(5); // Chef
    let isaac = Player::Seat(6); // Monk
    let sullivan = Player::Seat(7); // Poisoner
    let laurie = Player::Seat(8); // Recluse
    let dom = Player::Seat(9); // Mayor
    let blair = Player::Seat(10); // Imp

    let logs = vec![
        // NIGHT-1
        // 1. Sullivan (Poisoner) selects a player to neutralize.
        ReportLog::OnTime(sullivan, Claim::PoisonerPoisons(isaac)),
        // 2. Adam (Chef) learns how many pairs of evil players are sitting next to each other.
        // Could learn both 1 or 2.
        ReportLog::OnTime(adam, Claim::Chef(1)),
        // 3. John (Librarian) learns that either Adam or Laurie is the Recluse.
        ReportLog::OnTime(john, Claim::LibrarianSees(adam, laurie, Outsider::Recluse)),
        // 4. Carly (Fortune Teller) selects herself and another player.
        // She receives a "Yes" because she is her own Red Herring.
        ReportLog::OnTime(carly, Claim::FortuneTellerLearns(adam, laurie, true)),
        // 5. Brooke (Drunk) believes she is the Undertaker.
        // 6. Blair (Imp) learns her Minions (Ollie, Sullivan) and Demon Bluffs (Investigator, Empath, Saint).

        // DAY-1
        // Laurie (the Recluse) is executed.
        ReportLog::OnTime(Player::Unresolved, Claim::Executes(laurie)),
        // NIGHT-2
        // 1. Sullivan (Poisoner) poisons Isaac (the Monk).
        ReportLog::OnTime(sullivan, Claim::PoisonerPoisons(isaac)),
        // 2. Isaac (Monk) protects Carly (the Fortune Teller). (Invalidated by poison)
        ReportLog::OnTime(isaac, Claim::MonkProtects(carly)),
        // 3. Carly (Fortune Teller) selects herself and Dom. (Yes - Red Herring)
        ReportLog::OnTime(carly, Claim::FortuneTellerLearns(carly, dom, true)),
        // 4. Blair (Imp) kills John (the Librarian).
        ReportLog::OnTime(blair, Claim::ImpKills(john)),
        // 5. Brooke (Undertaker - Drunk) sees Laurie as the Recluse. (Truth)
        ReportLog::OnTime(
            brooke,
            Claim::UndertakerSees(laurie, Character::Good(Good::Outsider(Outsider::Recluse))),
        ),
        // DAY-2
        // Adam (the Chef) is nominated and executed.
        ReportLog::OnTime(Player::Unresolved, Claim::Executes(adam)),
        // NIGHT-3
        // 1. Sullivan (Poisoner) poisons Carly (the Fortune Teller). (Disables Red Herring)
        ReportLog::OnTime(sullivan, Claim::PoisonerPoisons(carly)),
        // 2. Isaac (Monk) protects Adam. (Wait, Adam is dead - protecting a corpse/sinking kill?)
        // In the game, Isaac protected someone, but Blair killed Brooke.
        ReportLog::OnTime(isaac, Claim::MonkProtects(adam)),
        // 3. Carly (Fortune Teller) selects herself and Dom. (No - Poisoned)
        ReportLog::OnTime(carly, Claim::FortuneTellerLearns(carly, dom, false)),
        // 4. Blair (Imp) kills Brooke (the Drunk).
        ReportLog::OnTime(blair, Claim::ImpKills(brooke)),
        // DAY-3
        // Isaac (the Monk) is nominated and executed.
        ReportLog::OnTime(Player::Unresolved, Claim::Executes(isaac)),
        // NIGHT-4
        // The Final Five (Blair, Sullivan, Ollie, Carly, Dom) remain.
        // There were no deaths this night.

        // DAY-4
        // Carly (the Fortune Teller) is executed.
        // Result: Leaving only 4 alive (Blair, Sullivan, Ollie, Dom).
        ReportLog::OnTime(Player::Unresolved, Claim::Executes(carly)),
        // NIGHT-5
        // 1. Sullivan (Poisoner) poisons Dom (the Mayor).
        // Consequence: Disables the "bounce" safety net.
        ReportLog::OnTime(sullivan, Claim::PoisonerPoisons(dom)),
        // 2. Blair (Imp) kills Dom (the Mayor).
        // Result: Dom dies, leaving only the 3 Evil players.
        ReportLog::OnTime(blair, Claim::ImpKills(dom)),
    ];

    dbg!(logs);
}

fn main() {
    println!("Hello, world!");
    let alice = Player::Seat(1);
    let bob = Player::Seat(2);
    let log = Claim::FortuneTellerLearns(alice, bob, true);
    println!("{:?}", &log);
    setup_game();
}

