use std::char;
use std::collections::{BTreeMap, HashMap};

use z3::ast::Bool;
use z3::{Context, Solver};

use crate::Player::Seat;
use crate::{Character, Claim, Time};
use crate::{Player, ReportLog};

struct Registry<'ctx> {
    context: &'ctx Context,

    num_players: i32,

    /// Boolean variables that track "Is player X character Y?"
    is_character: HashMap<(Player, Character), Bool>,

    /// Boolean variables that track "Is player X alive at the start of time Y?"
    is_alive: BTreeMap<Player, HashMap<Time, Bool>>,

    /// Is player X poisoned at the start of time Y?
    is_poisoned: BTreeMap<Player, HashMap<Time, Bool>>,

    /// Is player X a red herring?
    is_red_herring: HashMap<Player, Bool>,
}

impl<'ctx> Registry<'ctx> {
    /// Create a new registry of variables for the given context.
    pub fn new(context: &Context, num_players: usize) -> Registry {
        let mut is = HashMap::new();
        for seat in 0..num_players {
            let player = Player::Seat(seat.try_into().unwrap());
            for c in Character::iter() {
                is.insert(
                    (player, c),
                    Bool::new_const(format!("is_{:?}_{:?}", player, c)),
                );
            }
        }
        Registry {
            context,
            num_players: num_players.try_into().unwrap(),
            is_character: is,

            // TODO: actually populate this.
            is_alive: BTreeMap::new(),
            is_poisoned: BTreeMap::new(),
            is_red_herring: HashMap::new(),
        }
    }

    /// Get the variable that tracks "Is player X character Y?"
    pub fn get(&self, p: Player, c: Character) -> &Bool {
        &self.is_character[&(p, c)]
    }
}

pub fn constrain(r: &Registry, history: &Vec<ReportLog>, log: &ReportLog) -> z3::ast::Bool {
    use crate::Evil::*;
    use crate::Good::*;
    use crate::Minion::*;
    use crate::Outsider::*;
    use crate::ReportLog::OnTime;
    use crate::Townsfolk::*;
    use Character::*;
    use Claim::*;

    match log {
        // Washerwoman |alpha| sees |bravo| OR |charlie| as character |townsfolk|.
        OnTime(t, alpha, WasherwomanSees(bravo, charlie, townsfolk)) => {
            let alpha_is_washerwoman = r.get(*alpha, Good(Townsfolk(Washerwoman)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[&alpha][&t];

            let bravo_is_correct = r.get(*bravo, Good(Townsfolk(*townsfolk)));
            let charlie_is_correct = r.get(*charlie, Good(Townsfolk(*townsfolk)));

            let bravo_is_sober_spy =
                r.get(*bravo, Evil(Minion(Spy))) & r.is_poisoned[&bravo][&t].not();
            let charlie_is_sober_spy =
                r.get(*charlie, Evil(Minion(Spy))) & r.is_poisoned[&charlie][&t].not();

            (alpha_is_washerwoman & !alpha_is_poisoned).implies(
                alpha_is_drunk
                    | bravo_is_correct
                    | charlie_is_correct
                    | bravo_is_sober_spy
                    | charlie_is_sober_spy,
            )
        }

        // Librarian |alpha| sees that either |bravo| OR |charlie| is the Outsider |outsider|.
        OnTime(t, alpha, LibrarianSees(bravo, charlie, outsider)) => {
            let alpha_is_librarian = r.get(*alpha, Good(Townsfolk(Librarian)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[&alpha][&t];

            let bravo_is_correct = r.get(*bravo, Good(Outsider(*outsider)));
            let charlie_is_correct = r.get(*charlie, Good(Outsider(*outsider)));

            let bravo_is_sober_spy =
                r.get(*bravo, Evil(Minion(Spy))) & r.is_poisoned[&bravo][&t].not();

            let charlie_is_sober_spy =
                r.get(*charlie, Evil(Minion(Spy))) & r.is_poisoned[&charlie][&t].not();

            (alpha_is_librarian & !alpha_is_poisoned).implies(
                alpha_is_drunk
                    | bravo_is_correct
                    | charlie_is_correct
                    | bravo_is_sober_spy
                    | charlie_is_sober_spy,
            )
        }

        _ => todo!(), // TODO: implement the rest of the claim types.
    }
}

/// Every player has exactly one character.
/// TODO(proof): We have not yet modelled starpassing or ScarletWoman.
fn player_has_exactly_one_character(solver: &Solver, registry: &Registry) {
    for seat in 0..registry.num_players {
        {
            let _characters = Character::iter().map(|c| registry.get(Seat(seat), c));
            solver.assert(z3::ast::atleast(_characters, 1));
        }
        {
            let _characters = Character::iter().map(|c| registry.get(Seat(seat), c));
            solver.assert(z3::ast::atmost(_characters, 1));
        }
    }
}

/// Every character has at most one player.
/// NOTE: When the Scarlet Woman / Imp-Starpass mechanic is implemented, this will need to be updated.
fn character_has_at_most_one_player(solver: &Solver, registry: &Registry) {
    for c in Character::iter() {
        let _players = (0..registry.num_players).map(|seat| registry.get(Player::Seat(seat), c));
        solver.assert(z3::ast::atmost(_players, 1));
    }
}

pub fn foo() -> String {
    return String::from("hello world");
}
