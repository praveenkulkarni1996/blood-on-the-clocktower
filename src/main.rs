use botc::{Log, Player};

fn main() {
    println!("Hello, world!");
    let alice = Player::Seat(1);
    let bob = Player::Seat(2);
    let log = Log::FortuneTellerLearns(alice, bob, true);
    println!("{:?}", &log);
}
