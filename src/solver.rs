use std::collections::{BTreeMap, HashMap};

use z3::ast::Bool;
use z3::{Context, Solver};

use crate::Player;
use crate::Player::Seat;
use crate::{Character, Time};

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

pub fn foo() {
    let solver = Solver::new();

    let registry = Registry::new(&solver.get_context(), 3);

    use crate::{Good, Townsfolk};
    let adam_investigator: &Bool = registry.get(
        Player::Seat(0),
        Character::Good(Good::Townsfolk(Townsfolk::Investigator)),
    );
    solver.assert(adam_investigator);
    player_has_exactly_one_character(&solver, &registry);
    character_has_at_most_one_player(&solver, &registry);

    // run the solver
    _ = solver.check();
    let model = solver.get_model().unwrap();

    println!("{model:?}");

    for c in Character::iter() {
        println!("{:?}", c);
    }
}
