use std::collections::HashMap;
use strum::EnumIter;
use strum::IntoEnumIterator;

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

    // Adam as Investigator IMPLIES Eve is NOT the Investigator
    solver.assert(&adam_investigator.implies(&eve_investigator.not()));

    // run the solver
    _ = solver.check();
    let model = solver.get_model().unwrap();

    println!("{model:?}");

    for c in Character::iter() {
        println!("{:?}", c);
    }
}
