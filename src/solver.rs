use std::collections::HashMap;
use strum::EnumIter;
use strum::IntoEnumIterator;

use z3::ast;
use z3::ast::Bool;
use z3::{Context, Solver};

use crate::Character;

// Internal representations of players and characters for the solver.
#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum InternalPlayer {
    Adam,
    Eve,
}

struct Registry<'ctx> {
    context: &'ctx Context,

    /// Boolean variables that track "Is player X character Y?"
    is_character: HashMap<(InternalPlayer, Character), Bool>,
}

impl<'ctx> Registry<'ctx> {
    /// Create a new registry of variables for the given context.
    pub fn new(context: &Context) -> Registry {
        let mut is = HashMap::new();
        for p in InternalPlayer::iter() {
            for c in Character::iter() {
                is.insert((p, c), Bool::new_const(format!("is_{:?}_{:?}", p, c)));
            }
        }
        Registry {
            context,
            is_character: is,
        }
    }

    /// Get the variable that tracks "Is player X character Y?"
    pub fn get(&self, p: InternalPlayer, c: Character) -> &Bool {
        &self.is_character[&(p, c)]
    }
}

/// Every player has exactly one character.
/// TODO(proof): We have not yet modelled starpassing or ScarletWoman.
fn player_has_exactly_one_character(solver: &Solver, registry: &Registry) {
    for p in InternalPlayer::iter() {
        {
            let _characters = Character::iter().map(|c| registry.get(p, c));
            solver.assert(z3::ast::atleast(_characters, 1));
        }
        {
            let _characters = Character::iter().map(|c| registry.get(p, c));
            solver.assert(z3::ast::atmost(_characters, 1));
        }
    }
}

fn character_has_at_most_one_player(solver: &Solver, registry: &Registry) {
    for c in Character::iter() {
        let _players = InternalPlayer::iter().map(|p| registry.get(p, c));
        solver.assert(z3::ast::atmost(_players, 1));
    }
}

pub fn foo() {
    let solver = Solver::new();

    let registry = Registry::new(&solver.get_context());

    use crate::{Good, Townsfolk};
    let adam_investigator: &Bool = registry.get(
        InternalPlayer::Adam,
        Character::Good(Good::Townsfolk(Townsfolk::Investigator)),
    );
    let eve_investigator: &Bool = registry.get(
        InternalPlayer::Eve,
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
