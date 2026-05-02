use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

use z3::ast::Bool;
use z3::{Context, Solver};

use crate::Player::Seat;
use crate::{Character, Claim, Time, TimeIterator};
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

fn must_register_evil(r: &Registry, p: &Player) -> z3::ast::Bool {
    use crate::Character::*;
    use crate::Demon::*;
    use crate::Evil::*;
    use crate::Minion::*;

    r.get(*p, Evil(Minion(Baron)))
        | r.get(*p, Evil(Minion(Poisoner)))
        | r.get(*p, Evil(Minion(ScarletWoman)))
        | r.get(*p, Evil(Demon(Imp)))
}

fn can_register_evil(r: &Registry, p: &Player) -> z3::ast::Bool {
    use crate::Character::*;
    use crate::Demon::*;
    use crate::Evil::*;
    use crate::Good::*;
    use crate::Minion::*;
    use crate::Outsider::*;

    r.get(*p, Evil(Minion(Baron)))
        | r.get(*p, Evil(Minion(Poisoner)))
        | r.get(*p, Evil(Minion(ScarletWoman)))
        | r.get(*p, Evil(Minion(Spy)))
        | r.get(*p, Evil(Demon(Imp)))
        | r.get(*p, Good(Outsider(Recluse)))
}

fn must_evil_pair(r: &Registry, p1: &Player, p2: &Player) -> z3::ast::Bool {
    must_register_evil(r, p1) & must_register_evil(r, p2)
}

fn can_evil_pair(r: &Registry, p1: &Player, p2: &Player) -> z3::ast::Bool {
    can_register_evil(r, p1) & can_register_evil(r, p2)
}

impl<'ctx> Registry<'ctx> {
    /// Create a new registry of variables for the given context.
    pub fn new(context: &Context, num_players: usize, until: Time) -> Registry {
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

        let is_alive_map = {
            let mut is_alive = BTreeMap::new();
            // Populate is_alive[player][time] variable maps.
            for time in TimeIterator::new(until) {
                for seat in 0..num_players {
                    let player = Player::Seat(seat.try_into().unwrap());
                    is_alive.entry(player).or_insert_with(HashMap::new).insert(
                        time,
                        Bool::new_const(format!("is_alive_{:?}_{:?}", player, time)),
                    );
                }
            }
            is_alive
        };

        let is_poisoned_map = {
            let mut is_poisoned = BTreeMap::new();
            // Populate is_poisoned[player][time] variable maps.
            for time in TimeIterator::new(until) {
                for seat in 0..num_players {
                    let player = Player::Seat(seat.try_into().unwrap());
                    is_poisoned
                        .entry(player)
                        .or_insert_with(HashMap::new)
                        .insert(
                            time,
                            Bool::new_const(format!("is_poisoned_{:?}_{:?}", player, time)),
                        );
                }
            }
            is_poisoned
        };

        let is_red_herring_map = {
            let mut is_red_herring = HashMap::new();
            for seat in 0..num_players {
                let player = Player::Seat(seat.try_into().unwrap());
                is_red_herring.insert(
                    player,
                    Bool::new_const(format!("is_red_herring_{:?}", player)),
                );
            }
            is_red_herring
        };

        Registry {
            context,
            num_players: num_players.try_into().unwrap(),
            is_character: is,

            is_alive: is_alive_map,
            is_poisoned: is_poisoned_map,
            is_red_herring: is_red_herring_map,
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

        // Investigator |alpha| sees that either |bravo| OR |charlie| is the Minion |minion|.
        OnTime(t, alpha, InvestigatorSees(bravo, charlie, minion)) => {
            let alpha_is_investigator = r.get(*alpha, Good(Townsfolk(Investigator)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[&alpha][&t];

            let bravo_is_correct = r.get(*bravo, Evil(Minion(*minion)));
            let charlie_is_correct = r.get(*charlie, Evil(Minion(*minion)));

            let bravo_is_sober_recluse =
                r.get(*bravo, Good(Outsider(Recluse))) & r.is_poisoned[&bravo][&t].not();
            let charlie_is_sober_recluse =
                r.get(*charlie, Good(Outsider(Recluse))) & r.is_poisoned[&charlie][&t].not();

            (alpha_is_investigator & !alpha_is_poisoned).implies(
                alpha_is_drunk
                    | bravo_is_correct
                    | charlie_is_correct
                    | bravo_is_sober_recluse
                    | charlie_is_sober_recluse,
            )
        }

        // Chef |alpha| gets the number |num|. This means that |num| players (other than |alpha|)
        OnTime(t, alpha, ChefGets(num)) => {
            // Let us assume that the recluse and the spy are not poisoned.
            // In general, assuming that the Spy is
            // logic from the Good team's perspective, since every world in which the recluse is poisoned is equivalent to a world where the poisoner self-poisons.

            let zero = z3::ast::Int::from_i64(0);
            let one = z3::ast::Int::from_i64(1);

            let must_pairs: Vec<z3::ast::Int> = (0..r.num_players)
                .circular_tuple_windows::<(_, _)>()
                .map(|(p1, p2)| must_evil_pair(r, &Seat(p1), &Seat(p2)).ite(&one, &zero))
                .collect();

            let can_pairs: Vec<z3::ast::Int> = (0..r.num_players)
                .circular_tuple_windows::<(_, _)>()
                .map(|(p1, p2)| can_evil_pair(r, &Seat(p1), &Seat(p2)).ite(&one, &zero))
                .collect();

            let chef_num = z3::ast::Int::from_i64(*num as i64);
            let chef_min = z3::ast::Int::add(&must_pairs.iter().collect::<Vec<_>>()).le(&chef_num);
            let chef_max = z3::ast::Int::add(&can_pairs.iter().collect::<Vec<_>>()).ge(&chef_num);
            let chef_correct = chef_min & chef_max;

            let alpha_is_chef = r.get(*alpha, Good(Townsfolk(Chef)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[&alpha][&t];

            (alpha_is_chef & !alpha_is_poisoned).implies(alpha_is_drunk | chef_correct)
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
#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use crate::Character::*;
    use crate::Evil::*;
    use crate::Good::*;
    use crate::Minion::*;
    use crate::Outsider::*;
    use crate::Player::Seat;
    use crate::Time;
    use crate::Townsfolk::*;
    use z3::Solver;

    #[test]
    fn test_chef_spy_recluse_washerwoman_empath_gets_chef_0() {
        let solver = Solver::new();
        let registry = Registry::new(solver.get_context(), 5, Time::Night(1));

        // --- Add General Constraints ---
        player_has_exactly_one_character(&solver, &registry);
        character_has_at_most_one_player(&solver, &registry);

        // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
        solver.assert(&registry.is_character[&(Seat(0), Good(Townsfolk(Chef)))]);
        solver.assert(&registry.is_character[&(Seat(1), Evil(Minion(Spy)))]);
        solver.assert(&registry.is_character[&(Seat(2), Good(Outsider(Recluse)))]);
        solver.assert(&registry.is_character[&(Seat(3), Good(Townsfolk(Washerwoman)))]);
        solver.assert(&registry.is_character[&(Seat(4), Good(Townsfolk(Empath)))]);

        // --- Initialize is_poisoned map for relevant players/times ---
        solver.assert(!&registry.is_poisoned[&Seat(0)][&Time::Night(1)]); // Assert Chef is not poisoned

        // --- Add Chef's Claim Log ---
        // Chef (Player 0) claims a chef number of 1 at Time 0.
        let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(0));
        let chef_constraint = constrain(&registry, &vec![], &chef_claim);
        solver.assert(&chef_constraint);

        assert_eq!(solver.check(), z3::SatResult::Sat);
    }

    #[test]
    fn test_chef_spy_recluse_washerwoman_empath_gets_chef_1() {
        let solver = Solver::new();
        let registry = Registry::new(solver.get_context(), 5, Time::Night(1));

        // --- Add General Constraints ---
        player_has_exactly_one_character(&solver, &registry);
        character_has_at_most_one_player(&solver, &registry);

        // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
        solver.assert(&registry.is_character[&(Seat(0), Good(Townsfolk(Chef)))]);
        solver.assert(&registry.is_character[&(Seat(1), Evil(Minion(Spy)))]);
        solver.assert(&registry.is_character[&(Seat(2), Good(Outsider(Recluse)))]);
        solver.assert(&registry.is_character[&(Seat(3), Good(Townsfolk(Washerwoman)))]);
        solver.assert(&registry.is_character[&(Seat(4), Good(Townsfolk(Empath)))]);

        // --- Initialize is_poisoned map for relevant players/times ---
        solver.assert(!&registry.is_poisoned[&Seat(0)][&Time::Night(1)]); // Assert Chef is not poisoned

        // --- Add Chef's Claim Log ---
        // Chef (Player 0) claims a chef number of 1 at Time 0.
        let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(1));
        let chef_constraint = constrain(&registry, &vec![], &chef_claim);
        solver.assert(&chef_constraint);

        assert_eq!(solver.check(), z3::SatResult::Sat);
    }

    fn test_chef_spy_recluse_washerwoman_empath_cannot_get_chef2() {
        let solver = Solver::new();
        let registry = Registry::new(solver.get_context(), 5, Time::Night(1));

        // --- Add General Constraints ---
        player_has_exactly_one_character(&solver, &registry);
        character_has_at_most_one_player(&solver, &registry);

        // Round-Robin: CHEF - SPY - RECLUSE - WASHERWOMAN - EMPATH
        solver.assert(&registry.is_character[&(Seat(0), Good(Townsfolk(Chef)))]);
        solver.assert(&registry.is_character[&(Seat(1), Evil(Minion(Spy)))]);
        solver.assert(&registry.is_character[&(Seat(2), Good(Outsider(Recluse)))]);
        solver.assert(&registry.is_character[&(Seat(3), Good(Townsfolk(Washerwoman)))]);
        solver.assert(&registry.is_character[&(Seat(4), Good(Townsfolk(Empath)))]);

        // --- Initialize is_poisoned map for relevant players/times ---
        solver.assert(!&registry.is_poisoned[&Seat(0)][&Time::Night(1)]); // Assert Chef is not poisoned

        // --- Add Chef's Claim Log ---
        // Chef (Player 0) claims a chef number of 1 at Time 0.
        let chef_claim = ReportLog::OnTime(Time::Night(1), Seat(0), Claim::ChefGets(2));
        let chef_constraint = constrain(&registry, &vec![], &chef_claim);
        solver.assert(&chef_constraint);

        assert_eq!(solver.check(), z3::SatResult::Unsat);
    }
}
