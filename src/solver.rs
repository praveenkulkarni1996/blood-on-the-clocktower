use std::collections::HashMap;
use std::ops::Not;
use strum::EnumIter;
use strum::IntoEnumIterator;

use z3::ast::{Bool, Int};
use z3::{Context, Solver};

// Internal representations of players and characters for the solver.
#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum InternalPlayer {
    Adam,
    Eve,
}

#[derive(Hash, Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum InternalChar {
    // Good characters
    Investigator,
    Imp,
}

struct Registry<'ctx> {
    context: &'ctx Context,

    /// Boolean variables that track "Is player X character Y?"
    is_character: HashMap<(InternalPlayer, InternalChar), Bool>,
}

impl<'ctx> Registry<'ctx> {
    /// Create a new registry of variables for the given context.
    pub fn new(context: &Context) -> Registry {
        let mut is = HashMap::new();
        for p in InternalPlayer::iter() {
            for c in InternalChar::iter() {
                let var_name = is.insert((p, c), Bool::new_const(format!("is_{:?}_{:?}", p, c)));
            }
        }
        Registry {
            context,
            is_character: is,
        }
    }

    /// Get the variable that tracks "Is player X character Y?"
    pub fn get(&self, p: InternalPlayer, c: InternalChar) -> &Bool {
        &self.is_character[&(p, c)]
    }
}

pub fn foo() {
    let solver = Solver::new();

    let registry = Registry::new(&solver.get_context());

    let adam_investigator: &Bool = registry.get(InternalPlayer::Adam, InternalChar::Investigator);
    let eve_investigator: &Bool = registry.get(InternalPlayer::Eve, InternalChar::Investigator);
    solver.assert(adam_investigator);

    // Adam as Investigator IMPLIES Eve is NOT the Investigator
    solver.assert(&adam_investigator.implies(&eve_investigator.not()));

    // run the solver
    _ = solver.check();
    let model = solver.get_model().unwrap();

    println!("{model:?}");
}
