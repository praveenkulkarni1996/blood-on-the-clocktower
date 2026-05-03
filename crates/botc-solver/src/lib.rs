use core::panic;
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

use z3::ast::Bool;
use z3::{Context, Solver};

use botc_core::Player::Seat;
use botc_core::{Character, Claim, Time, TimeIterator};
use botc_core::{Player, ReportLog};

pub mod life;
pub mod setup;

pub struct Registry<'ctx> {
    _context: &'ctx Context,

    num_players: i32,

    until: Time,

    /// Boolean variables that track "Is player X character Y?"
    is_character: HashMap<(Player, Character), Bool>,

    /// Boolean variables that track "Is player X alive at the start of time Y?"
    is_alive: BTreeMap<Player, HashMap<Time, Bool>>,

    /// Is player X poisoned at the start of time Y?
    is_poisoned: BTreeMap<Player, HashMap<Time, Bool>>,

    /// Is player X a red herring?
    is_red_herring: HashMap<Player, Bool>,
}

fn is_lying(r: &Registry, p: Player) -> z3::ast::Bool {
    use botc_core::Character::*;
    use botc_core::Demon::*;
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;

    // Only drunks + minions + demons will lie about their role.
    r.get(p, Good(Outsider(Drunk)))
        | r.get(p, Evil(Minion(Baron)))
        | r.get(p, Evil(Minion(Poisoner)))
        | r.get(p, Evil(Minion(ScarletWoman)))
        | r.get(p, Evil(Minion(Spy)))
        | r.get(p, Evil(Demon(Imp)))
}

fn must_register_evil(r: &Registry, p: &Player) -> z3::ast::Bool {
    use botc_core::Character::*;
    use botc_core::Demon::*;
    use botc_core::Evil::*;
    use botc_core::Minion::*;

    r.get(*p, Evil(Minion(Baron)))
        | r.get(*p, Evil(Minion(Poisoner)))
        | r.get(*p, Evil(Minion(ScarletWoman)))
        | r.get(*p, Evil(Demon(Imp)))
}

fn can_register_evil(r: &Registry, p: &Player) -> z3::ast::Bool {
    use botc_core::Character::*;
    use botc_core::Demon::*;
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;

    r.get(*p, Evil(Minion(Baron)))
        | r.get(*p, Evil(Minion(Poisoner)))
        | r.get(*p, Evil(Minion(ScarletWoman)))
        | r.get(*p, Evil(Minion(Spy)))
        | r.get(*p, Evil(Demon(Imp)))
        | r.get(*p, Good(Outsider(Recluse)))
}

fn can_register_good(r: &Registry, p: &Player) -> z3::ast::Bool {
    use botc_core::Character::*;
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;
    use botc_core::Townsfolk::*;

    r.get(*p, Good(Townsfolk(Washerwoman)))
        | r.get(*p, Good(Townsfolk(Librarian)))
        | r.get(*p, Good(Townsfolk(Investigator)))
        | r.get(*p, Good(Townsfolk(Chef)))
        | r.get(*p, Good(Townsfolk(Empath)))
        | r.get(*p, Good(Townsfolk(FortuneTeller)))
        | r.get(*p, Good(Townsfolk(Undertaker)))
        | r.get(*p, Good(Townsfolk(Monk)))
        | r.get(*p, Good(Townsfolk(Ravenkeeper)))
        | r.get(*p, Good(Townsfolk(Virgin)))
        | r.get(*p, Good(Townsfolk(Slayer)))
        | r.get(*p, Good(Townsfolk(Soldier)))
        | r.get(*p, Good(Townsfolk(Mayor)))
        | r.get(*p, Good(Outsider(Recluse)))
        | r.get(*p, Good(Outsider(Saint)))
        | r.get(*p, Good(Outsider(Butler)))
        | r.get(*p, Good(Outsider(Drunk)))
        | r.get(*p, Evil(Minion(Spy)))
}

fn must_evil_pair(r: &Registry, p1: &Player, p2: &Player) -> z3::ast::Bool {
    must_register_evil(r, p1) & must_register_evil(r, p2)
}

fn can_evil_pair(r: &Registry, p1: &Player, p2: &Player) -> z3::ast::Bool {
    can_register_evil(r, p1) & can_register_evil(r, p2)
}

impl<'ctx> Registry<'ctx> {
    /// Create a new registry of variables for the given context.
    pub fn new(context: &Context, num_players: usize, until: Time) -> Registry<'_> {
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
            _context: context,
            num_players: num_players.try_into().unwrap(),
            until,

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

pub fn constrain(r: &Registry, _history: &Vec<ReportLog>, log: &ReportLog) -> z3::ast::Bool {
    use Character::*;
    use Claim::*;
    use botc_core::Demon::*;
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Outsider::*;
    use botc_core::ReportLog::OnTime;
    use botc_core::Townsfolk::*;

    match log {
        // Character |alpha| claims to be |character|.
        OnTime(_t, alpha, Am(character)) => {
            let alpha_is_lying = &is_lying(r, *alpha);
            let alpha_is_character = r.get(*alpha, *character);

            alpha_is_lying | alpha_is_character
        }
        // Washerwoman |alpha| sees |bravo| OR |charlie| as character |townsfolk|.
        OnTime(t, alpha, WasherwomanSees(bravo, charlie, townsfolk)) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Washerwoman)))
                & see_character_between_player_pair(
                    r,
                    *alpha,
                    *bravo,
                    *charlie,
                    Good(Townsfolk(*townsfolk)),
                    *t,
                )
        }

        // Librarian |alpha| sees that either |bravo| OR |charlie| is the Outsider |outsider|.
        OnTime(t, alpha, LibrarianSees(bravo, charlie, outsider)) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Librarian)))
                & see_character_between_player_pair(
                    r,
                    *alpha,
                    *bravo,
                    *charlie,
                    Good(Outsider(*outsider)),
                    *t,
                )
        }

        // Investigator |alpha| sees that either |bravo| OR |charlie| is the Minion |minion|.
        OnTime(t, alpha, InvestigatorSees(bravo, charlie, minion)) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Investigator)))
                & see_character_between_player_pair(
                    r,
                    *alpha,
                    *bravo,
                    *charlie,
                    Evil(Minion(*minion)),
                    *t,
                )
        }

        // Chef |alpha| gets the number |num|. This means that |num| players (other than |alpha|)
        OnTime(t, alpha, ChefGets(num)) => {
            // Let us assume that the recluse and the spy are not poisoned.
            // In general, assuming that the Spy is
            // logic from the Good team's perspective, since every world in which the
            // recluse is poisoned is equivalent to a world where the poisoner self-poisons.

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
            let alpha_is_lying = &is_lying(r, *alpha);
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            (alpha_is_lying | alpha_is_chef)
                & alpha_is_chef.implies(alpha_is_poisoned | chef_correct)
        }

        // Empath |alpha| gets a ZERO on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsZero(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            let alpha_is_empath: &Bool = r.get(*alpha, Good(Townsfolk(Empath)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            alpha_is_empath.implies(
                alpha_is_poisoned
                    | alpha_is_drunk
                    | (can_register_good(r, bravo) & can_register_good(r, charlie)),
            )
        }

        // Empath |alpha| gets a ONE on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsOne(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            let alpha_is_empath: &Bool = r.get(*alpha, Good(Townsfolk(Empath)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            alpha_is_empath.implies(
                alpha_is_poisoned
                    | alpha_is_drunk
                    | (can_register_good(r, bravo) & can_register_evil(r, charlie))
                    | (can_register_evil(r, bravo) & can_register_good(r, charlie)),
            )
        }

        // Empath |alpha| gets a TWO on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsTwo(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            let alpha_is_empath: &Bool = r.get(*alpha, Good(Townsfolk(Empath)));
            let alpha_is_drunk = r.get(*alpha, Good(Outsider(Drunk)));
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            alpha_is_empath.implies(
                alpha_is_poisoned
                    | alpha_is_drunk
                    | (can_register_evil(r, bravo) & can_register_evil(r, charlie)),
            )
        }

        // FortuneTeller |alpha| gets a YES on |bravo| and |charlie|.
        OnTime(t, alpha, FortuneTellerYes(bravo, charlie)) => {
            let alpha_is_fortune_teller: &Bool = r.get(*alpha, Good(Townsfolk(FortuneTeller)));
            let alpha_is_lying = &is_lying(r, *alpha);
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            let bravo_is_demon = r.get(*bravo, Evil(Demon(Imp)));
            let charlie_is_demon = r.get(*charlie, Evil(Demon(Imp)));

            let bravo_is_red_herring = &r.is_red_herring[bravo];
            let charlie_is_red_herring = &r.is_red_herring[charlie];

            let bravo_is_sober_recluse =
                r.get(*bravo, Good(Outsider(Recluse))) & r.is_poisoned[bravo][t].not();
            let charlie_is_sober_recluse =
                r.get(*charlie, Good(Outsider(Recluse))) & r.is_poisoned[charlie][t].not();

            (alpha_is_lying | alpha_is_fortune_teller)
                & alpha_is_fortune_teller.implies(
                    alpha_is_poisoned
                        | bravo_is_demon
                        | charlie_is_demon
                        | bravo_is_red_herring
                        | charlie_is_red_herring
                        | bravo_is_sober_recluse
                        | charlie_is_sober_recluse,
                )
        }

        // FortuneTeller |alpha| gets a NO on |bravo| and |charlie|.
        OnTime(t, alpha, FortuneTellerNo(bravo, charlie)) => {
            let alpha_is_fortune_teller: &Bool = r.get(*alpha, Good(Townsfolk(FortuneTeller)));
            let alpha_is_lying = &is_lying(r, *alpha);
            let alpha_is_poisoned = &r.is_poisoned[alpha][t];

            let bravo_is_demon = r.get(*bravo, Evil(Demon(Imp)));
            let charlie_is_demon = r.get(*charlie, Evil(Demon(Imp)));

            let bravo_is_red_herring = &r.is_red_herring[bravo];
            let charlie_is_red_herring = &r.is_red_herring[charlie];

            (alpha_is_lying | alpha_is_fortune_teller)
                & alpha_is_fortune_teller.implies(
                    alpha_is_poisoned
                        | !bravo_is_demon
                        | !charlie_is_demon
                        | !bravo_is_red_herring
                        | !charlie_is_red_herring,
                )
        }

        // Undertaker |alpha| sees the previously executed player |bravo| as the |character|.
        OnTime(t, alpha, UndertakerSees(bravo, character)) => {
            // TODO: Add checks that |bravo| is the previously executed player from
            // |history|. If we do not do that, then this essentially becomes a
            // RavenKeeper.
            player_claims_character(r, *alpha, Good(Townsfolk(Undertaker)))
                & player_sees_other_players_character(r, alpha, bravo, *character, t)
        }

        // Ravenkeeper |alpha| sees the |bravo| as the |character|.
        OnTime(t, alpha, RavenkeeperSees(bravo, character)) => {
            // TODO: Add checks that |alpha| has JUST died.
            player_claims_character(r, *alpha, Good(Townsfolk(Ravenkeeper)))
                & player_sees_other_players_character(r, alpha, bravo, *character, t)
        }

        // The player |alpha| claims soldier, and is killed in the night.
        ReportLog::OnTime(t, alpha, Claim::SoldierNightKilled) => {
            player_unexpected_night_killed(r, alpha, t)
        }
        // The player |alpha| claims monk, and his protected player died in the night.
        ReportLog::OnTime(t, alpha, Claim::MonkProtectedNightKilled) => {
            player_unexpected_night_killed(r, alpha, t)
        }

        // The town executes the |player| at time |t|.
        ReportLog::DayExecutes(t, player) => {
            let day = match t {
                Time::Day(day) => day,
                Time::Night(_) => panic!("cannot have night execution"),
            };

            // A list of variables "Player P is dead at future time T"
            let player_is_not_alive: Vec<Bool> =
                TimeIterator::new_with_start(Time::Night(day + 1), r.until)
                    .map(|time| !r.is_alive[player][&time].clone())
                    .collect_vec();

            z3::ast::Bool::and(player_is_not_alive.iter().collect_vec().as_slice())
                & assert_character_is_alive(r, Evil(Demon(Imp)), *t)
        }

        // The player dies at night.
        // Killed by the Imp who picked them, or Mayor-bounced to them.
        ReportLog::NightKilled(t, player) => {
            let night = match t {
                Time::Day(_) => panic!("cannot have day killing"),
                Time::Night(night) => night,
            };

            // A list of variables "Player P is dead at future time T"
            let player_is_not_alive: Vec<Bool> =
                TimeIterator::new_with_start(Time::Night(*night), r.until)
                    .map(|time| !r.is_alive[player][&time].clone())
                    .collect_vec();

            z3::ast::Bool::and(player_is_not_alive.iter().collect_vec().as_slice())
                & assert_character_is_alive(r, Evil(Demon(Imp)), *t)
        }

        // Documentation only fields are not used for solving.
        ReportLog::DocumentOnly(_, _, _) => z3::ast::Bool::from_bool(true),

        _other => todo!("pending implementation: {:?}", _other), /* TODO: implement the rest of
                                                                  * the claim types. */
    }
}

/// Every player has exactly one character.
/// TODO(proof): We have not yet modelled starpassing or ScarletWoman.
pub fn player_has_exactly_one_character(solver: &Solver, registry: &Registry) {
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
/// NOTE: When the Scarlet Woman / Imp-Starpass mechanic is implemented, this
/// will need to be updated.
pub fn character_has_at_most_one_player(solver: &Solver, registry: &Registry) {
    for c in Character::iter() {
        let _players = (0..registry.num_players).map(|seat| registry.get(Player::Seat(seat), c));
        solver.assert(z3::ast::atmost(_players, 1));
    }
}

pub fn atmost_one_player_can_be_poisoned(solver: &Solver, registry: &Registry) {
    for time in TimeIterator::new(registry.until) {
        let _poisoned =
            (0..registry.num_players).map(|seat| &registry.is_poisoned[&Player::Seat(seat)][&time]);
        solver.assert(z3::ast::atmost(_poisoned, 1));
    }
}

pub fn atmost_one_player_can_be_red_herringed(solver: &Solver, registry: &Registry) {
    let _red_herringed =
        (0..registry.num_players).map(|seat| &registry.is_red_herring[&Player::Seat(seat)]);
    solver.assert(z3::ast::atmost(_red_herringed, 1));
}

/// We do not yet support the ability for a player to change characters.
/// There are cases when a Poisoner could become the next Imp, and that is not
/// yet handled in this case.
pub fn poisoner_can_poison_one_person_only_if_alive(solver: &Solver, r: &Registry) {
    use botc_core::Character::*;
    use botc_core::Evil::*;
    use botc_core::Minion::*;

    let is_alive_poisoner = |player: &Player, t: &Time| -> z3::ast::Bool {
        let is_alive = &r.is_alive[player][t];
        let is_poisoner = r.get(*player, Evil(Minion(Poisoner)));
        is_alive & is_poisoner
    };

    for time in TimeIterator::new(r.until) {
        let is_someone_alive_poisoner = {
            let is_player_alive_poisoner = (0..r.num_players)
                .map(Seat)
                .map(|player| is_alive_poisoner(&player, &time))
                .collect_vec();

            z3::ast::atleast(is_player_alive_poisoner.iter(), 1)
        };

        let is_someone_poisoned = {
            let is_player_poisoned = (0..r.num_players)
                .map(Seat)
                .map(|player| &r.is_poisoned[&player][&time]);

            z3::ast::atmost(is_player_poisoned, 1)
        };

        let is_no_one_poisoned = {
            let _is_player_poisoned = (0..r.num_players)
                .map(Seat)
                .map(|player| &r.is_poisoned[&player][&time]);
            z3::ast::atmost(_is_player_poisoned, 0)
        };

        let alive_poisoner_rule = is_someone_alive_poisoner.implies(is_someone_poisoned);
        let dead_poisoner_rule = !is_someone_alive_poisoner.implies(is_no_one_poisoned);

        solver.assert(alive_poisoner_rule);
        solver.assert(dead_poisoner_rule);
    }
}

/// A poisoned player during DAY(x) must be NIGHT(x) as well.
///
/// In Trouble Brewing, it is (likely) possible to model deaths has taking place
/// at "dusk", the interplay between NIGHT and DAY. That is because:
///   (1) Day deaths (i.e. executions) trigger the end of the day.
///   (2) Night deaths (i.e. demon kills) happen at the start of the night.
///
/// In more complicated scripts, it might be the case that we cannot model the
/// night order in a very black-and-white way without painstakingly modelling
/// the "night order" explicitly. For example in "Bad Moon Rising" script, the
/// Goon is turned good over evil depending upon the exact character
/// actions - and the night order is extremely important.
///
/// But, as of right now - we have modelled this Time::{Night, Day}, and we can
/// reconsider simplifying or extending this in the future.
pub fn poisoning_does_not_move_during_the_day(solver: &Solver, registry: &Registry) {
    let days: Vec<i32> = TimeIterator::new(registry.until)
        .filter_map(|time| match time {
            Time::Day(x) => Some(x),
            _ => None,
        })
        .collect_vec();

    let players = (0..registry.num_players).map(Seat).collect_vec();

    for player in players {
        for &day in days.iter() {
            let is_player_poisoned_day = &registry.is_poisoned[&player][&Time::Day(day)];
            let is_player_poisoned_night = &registry.is_poisoned[&player][&Time::Night(day)];
            solver.assert(is_player_poisoned_day.implies(is_player_poisoned_night));
        }
    }
}

// Assert that the character is still alive.
fn assert_character_is_alive(r: &Registry, c: Character, time: Time) -> z3::ast::Bool {
    let is_alive_character = |player: Player| -> z3::ast::Bool {
        let is_alive = &r.is_alive[&player][&time];
        let is_character = &r.is_character[&(player, c)];
        is_alive & is_character
    };

    let is_player_alive_character: Vec<z3::ast::Bool> = (0..r.num_players)
        .map(Seat)
        .map(is_alive_character)
        .collect_vec();

    z3::ast::Bool::or(is_player_alive_character.iter().collect_vec().as_slice())
}

pub fn mark_characters_not_in_play(solver: &Solver, registry: &Registry, characters: &[Character]) {
    for c in characters.iter() {
        let _players = (0..registry.num_players).map(|seat| registry.get(Player::Seat(seat), *c));
        solver.assert(z3::ast::atmost(_players, 0));
    }
}

// A supposedly-immune player has un-expectedly died in the night.
fn player_unexpected_night_killed(r: &Registry, defender: &Player, time: &Time) -> Bool {
    let is_defender_lying: Bool = is_lying(r, *defender);
    let is_defender_poisoned: &Bool = &r.is_poisoned[defender][time];

    is_defender_lying | is_defender_poisoned
}

/// A player claims to be a character.
fn player_claims_character(r: &Registry, claimant: Player, character: Character) -> Bool {
    is_lying(r, claimant) ^ r.get(claimant, character)
}

/// A player (e.g. Ravenskeeper, Undertaker) sees another player's token.
fn player_sees_other_players_character(
    r: &Registry,
    seer: &Player,
    target: &Player,
    token: Character,
    t: &Time,
) -> Bool {
    use botc_core::Character::{Evil, Good};
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;

    let seer_lied: Bool = is_lying(r, *seer);
    let seer_is_poisoned: &Bool = &r.is_poisoned[seer][t];
    let target_is_correct: &Bool = r.get(*target, token);

    let target_is_spy = r.get(*target, Evil(Minion(Spy)));
    let target_is_recluse = r.get(*target, Good(Outsider(Recluse)));

    match token {
        Good(_) => (!seer_is_poisoned & !seer_lied).implies(target_is_correct | target_is_spy),
        Evil(_) => (!seer_is_poisoned & !seer_lied).implies(target_is_correct | target_is_recluse),
    }
}

fn see_character_between_player_pair(
    r: &Registry,
    seer: Player,
    a: Player,
    b: Player,
    token: Character,
    time: Time,
) -> Bool {
    use botc_core::Character::{Evil, Good};
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;

    let seer_lied: Bool = is_lying(r, seer);
    let seer_is_poisoned: &Bool = &r.is_poisoned[&seer][&time];

    let a_is_correct = r.get(a, token);
    let b_is_correct = r.get(b, token);

    let a_is_spy = r.get(a, Evil(Minion(Spy)));
    let b_is_spy = r.get(b, Evil(Minion(Spy)));

    let a_is_recluse = r.get(a, Good(Outsider(Recluse)));
    let b_is_recluse = r.get(b, Good(Outsider(Recluse)));

    let valid: Bool = match token {
        Good(_) => a_is_correct | b_is_correct | a_is_spy | b_is_spy,
        Evil(_) => a_is_correct | b_is_correct | a_is_recluse | b_is_recluse,
    };

    (!seer_lied & !seer_is_poisoned).implies(valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use botc_core::Character::*;
    use botc_core::Evil::*;
    use botc_core::Good::*;
    use botc_core::Minion::*;
    use botc_core::Outsider::*;
    use botc_core::Player::Seat;
    use botc_core::Time;
    use botc_core::Townsfolk::*;
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

    #[test]
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
