#![deny(clippy::pedantic)]

use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

use z3::ast::Bool;

use botc_core::Player::Seat;
use botc_core::{Character, Time, TimeIterator};
use botc_core::{Player, ReportLog};

pub mod life;
pub mod registers;
pub mod setup;

/// Debugging / exploration helpers (e.g. model inspection).
/// Not part of the stable public API.
pub mod debugging;

pub struct Registry {
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

#[must_use]
pub fn game_setup(r: &Registry) -> z3::ast::Bool {
    // players and characters
    setup::assert_player_count_rules(r)
        & setup::assert_unique_player_tokens(r)
        // life-and-death
        & life::assert_life_until_death(r)
        // poisoning and red-herring
        & poisoner_can_poison_one_person_only_if_alive(r)
        & poisoning_does_not_move_during_the_day(r)
        & atmost_one_player_can_be_poisoned(r)
        & atmost_one_player_can_be_red_herringed(r)
}

fn can_lie(r: &Registry, p: Player) -> z3::ast::Bool {
    use botc_core::Character::{Evil, Good};
    use botc_core::Demon::Imp;
    use botc_core::Evil::{Demon, Minion};
    use botc_core::Good::Outsider;
    use botc_core::Minion::{Baron, Poisoner, ScarletWoman, Spy};
    use botc_core::Outsider::Drunk;

    // Roles that are allowed to lie (Drunk is poisoned, minions + demons
    // deliberately deceive).
    r.get(p, Good(Outsider(Drunk)))
        | r.get(p, Evil(Minion(Baron)))
        | r.get(p, Evil(Minion(Poisoner)))
        | r.get(p, Evil(Minion(ScarletWoman)))
        | r.get(p, Evil(Minion(Spy)))
        | r.get(p, Evil(Demon(Imp)))
}

fn must_evil_pair(r: &Registry, p1: Player, p2: Player) -> z3::ast::Bool {
    registers::must_evil(r, p1) & registers::must_evil(r, p2)
}

fn can_evil_pair(r: &Registry, p1: Player, p2: Player) -> z3::ast::Bool {
    registers::can_evil(r, p1) & registers::can_evil(r, p2)
}

impl Registry {
    /// Create a new registry of variables.
    ///
    /// # Panics
    ///
    /// Panics if `num_players` is greater than `i32::MAX`.
    #[must_use]
    pub fn new(num_players: usize, until: Time) -> Registry {
        let mut is = HashMap::new();
        for seat in 0..num_players {
            let player = Player::Seat(seat.try_into().unwrap());
            for c in Character::iter() {
                is.insert((player, c), Bool::new_const(format!("is_{player:?}_{c:?}")));
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
                        Bool::new_const(format!("is_alive_{player:?}_{time:?}")),
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
                            Bool::new_const(format!("is_poisoned_{player:?}_{time:?}")),
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
                    Bool::new_const(format!("is_red_herring_{player:?}")),
                );
            }
            is_red_herring
        };

        Registry {
            num_players: num_players.try_into().unwrap(),
            until,

            is_character: is,
            is_alive: is_alive_map,
            is_poisoned: is_poisoned_map,
            is_red_herring: is_red_herring_map,
        }
    }

    /// Get the variable that tracks "Is player X character Y?"
    #[must_use]
    pub fn get(&self, p: Player, c: Character) -> &Bool {
        &self.is_character[&(p, c)]
    }

    /// Get the variable that tracks if player X is the red herring.
    #[must_use]
    pub fn is_red_herring(&self, p: Player) -> &Bool {
        &self.is_red_herring[&p]
    }
}

/// # Panics
///
/// Panics if a `DayExecutes` event is reported during the night.
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn constrain(r: &Registry, _history: &[ReportLog], log: &ReportLog) -> z3::ast::Bool {
    use botc_core::Character::{Evil, Good};
    use botc_core::Claim::{
        Am, ChefGets, EmpathLearnsOne, EmpathLearnsTwo, EmpathLearnsZero, FortuneTellerNo,
        FortuneTellerYes, InvestigatorSees, LibrarianSees, LibrarianZero, MonkProtectedNightKilled,
        PoisonerPoisons, RavenkeeperSees, SaintExecutedWithoutDefeat, SlayerKillsDemon,
        SlayerMisses, SoldierNightKilled, UndertakerSees, VirginKillsTownsfolk, VirginMisses,
        WasherwomanSees,
    };
    use botc_core::Demon::Imp;
    use botc_core::Evil::{Demon, Minion};
    use botc_core::Good::{Outsider, Townsfolk};
    use botc_core::Minion::Poisoner;
    use botc_core::Outsider::Saint;
    use botc_core::ReportLog::OnTime;
    use botc_core::Townsfolk::{
        Chef, Empath, FortuneTeller, Investigator, Librarian, Monk, Ravenkeeper, Slayer, Soldier,
        Undertaker, Virgin, Washerwoman,
    };

    match log {
        // Character |alpha| claims to be |character|.
        OnTime(_t, alpha, Am(character)) => {
            let alpha_can_lie = &can_lie(r, *alpha);
            let alpha_is_character = r.get(*alpha, *character);

            alpha_can_lie | alpha_is_character
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

        // Librarian |alpha| is told no players are registering as outsiders.
        OnTime(t, alpha, LibrarianZero) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Librarian)))
                & is_effective(r, *alpha, *t).implies(setup::assert_player_count_by_predicate(
                    r,
                    registers::must_outsider,
                    0,
                ))
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
            let zero = z3::ast::Int::from_i64(0);
            let one = z3::ast::Int::from_i64(1);

            let must_pairs: Vec<z3::ast::Int> = (0..r.num_players)
                .circular_tuple_windows::<(_, _)>()
                .map(|(p1, p2)| must_evil_pair(r, Seat(p1), Seat(p2)).ite(&one, &zero))
                .collect();

            let can_pairs: Vec<z3::ast::Int> = (0..r.num_players)
                .circular_tuple_windows::<(_, _)>()
                .map(|(p1, p2)| can_evil_pair(r, Seat(p1), Seat(p2)).ite(&one, &zero))
                .collect();

            let chef_num = z3::ast::Int::from_i64(i64::from(*num));
            let chef_min = z3::ast::Int::add(&must_pairs.iter().collect::<Vec<_>>()).le(&chef_num);
            let chef_max = z3::ast::Int::add(&can_pairs.iter().collect::<Vec<_>>()).ge(&chef_num);
            let chef_correct = chef_min & chef_max;

            player_claims_character(r, *alpha, Good(Townsfolk(Chef)))
                & is_effective(r, *alpha, *t).implies(chef_correct)
        }

        // Empath |alpha| gets a ZERO on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsZero(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            player_claims_character(r, *alpha, Good(Townsfolk(Empath)))
                & is_effective(r, *alpha, *t)
                    .implies(registers::can_good(r, *bravo) & registers::can_good(r, *charlie))
        }

        // Empath |alpha| gets a ONE on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsOne(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            player_claims_character(r, *alpha, Good(Townsfolk(Empath)))
                & is_effective(r, *alpha, *t).implies(
                    (registers::can_good(r, *bravo) & registers::can_evil(r, *charlie))
                        | (registers::can_evil(r, *bravo) & registers::can_good(r, *charlie)),
                )
        }

        // Empath |alpha| gets a TWO on their two alive neighbors: |bravo| and |charlie|.
        OnTime(t, alpha, EmpathLearnsTwo(bravo, charlie)) => {
            // TODO: Add checks to ensure that bravo and charlie are actually alive
            // neighbors.
            player_claims_character(r, *alpha, Good(Townsfolk(Empath)))
                & is_effective(r, *alpha, *t)
                    .implies(registers::can_evil(r, *bravo) & registers::can_evil(r, *charlie))
        }

        // FortuneTeller |alpha| gets a YES on |bravo| and |charlie|.
        OnTime(t, alpha, FortuneTellerYes(bravo, charlie)) => {
            let bravo_is_red_herring = &r.is_red_herring[bravo];
            let charlie_is_red_herring = &r.is_red_herring[charlie];

            player_claims_character(r, *alpha, Good(Townsfolk(FortuneTeller)))
                & is_effective(r, *alpha, *t).implies(
                    Bool::from_bool(false)
                        | bravo_is_red_herring
                        | charlie_is_red_herring
                        | registers::can_demon(r, *bravo)
                        | registers::can_demon(r, *charlie),
                )
        }

        // FortuneTeller |alpha| gets a NO on |bravo| and |charlie|.
        OnTime(t, alpha, FortuneTellerNo(bravo, charlie)) => {
            let bravo_is_red_herring = &r.is_red_herring[bravo];
            let charlie_is_red_herring = &r.is_red_herring[charlie];

            player_claims_character(r, *alpha, Good(Townsfolk(FortuneTeller)))
                & is_effective(r, *alpha, *t).implies(
                    Bool::from_bool(true)
                        & !bravo_is_red_herring
                        & !charlie_is_red_herring
                        & !registers::must_demon(r, *bravo)
                        & !registers::must_demon(r, *charlie),
                )
        }

        // Undertaker |alpha| sees the previously executed player |bravo| as the |character|.
        OnTime(t, alpha, UndertakerSees(bravo, character)) => {
            // TODO: Add checks that |bravo| is the previously executed player from
            // |history|. If we do not do that, then this essentially becomes a
            // RavenKeeper.
            player_claims_character(r, *alpha, Good(Townsfolk(Undertaker)))
                & player_sees_other_players_character(r, *alpha, *bravo, *character, *t)
        }

        // Ravenkeeper |alpha| sees the |bravo| as the |character|.
        OnTime(t, alpha, RavenkeeperSees(bravo, character)) => {
            // TODO: Add checks that |alpha| has JUST died.
            player_claims_character(r, *alpha, Good(Townsfolk(Ravenkeeper)))
                & player_sees_other_players_character(r, *alpha, *bravo, *character, *t)
        }

        // The player |alpha| claims soldier, and is killed in the night.
        // TODO: Add checks that the victim is actually dead.
        OnTime(t, alpha, SoldierNightKilled) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Soldier)))
                & !is_effective(r, *alpha, *t)
        }
        // The player |alpha| claims monk, and his protected player died in the night.
        // TODO: Add checks that the victim is actually dead.
        OnTime(t, alpha, MonkProtectedNightKilled) => {
            player_claims_character(r, *alpha, Good(Townsfolk(Monk))) & !is_effective(r, *alpha, *t)
        }

        // The virgin |alpha| kills the first |nominator|.
        // NOTE: The exact details about nominating are modelled at a higher level.
        OnTime(t, alpha, VirginKillsTownsfolk(nominator)) => {
            player_must_character(r, *alpha, Good(Townsfolk(Virgin)))
                & is_effective(r, *alpha, *t)
                & registers::can_townsfolk(r, *nominator)
        }

        // The supposed-virgin |virgin| is unable to kill the first |nominator|.
        OnTime(t, virgin, VirginMisses(nominator)) => {
            player_claims_character(r, *virgin, Good(Townsfolk(Virgin)))
                & (!is_effective(r, *virgin, *t) ^ !registers::must_townsfolk(r, *nominator))
        }

        // The player |slayer| is able to kill the |target|.
        OnTime(t, slayer, SlayerKillsDemon(target)) => {
            player_must_character(r, *slayer, Good(Townsfolk(Slayer)))
                & is_effective(r, *slayer, *t)
                & registers::can_demon(r, *target)
        }

        // The supposed-slayer |slayer| is unable to kill their |target|.
        OnTime(t, slayer, SlayerMisses(target)) => {
            player_claims_character(r, *slayer, Good(Townsfolk(Slayer)))
                & (!is_effective(r, *slayer, *t) ^ !registers::must_demon(r, *target))
        }

        // The supposed-saint |saint| does not end the game when executed.
        OnTime(t, saint, SaintExecutedWithoutDefeat) => {
            player_claims_character(r, *saint, Good(Outsider(Saint))) & !is_effective(r, *saint, *t)
        }

        // NOTE: This is the only explicit evil-player action - meant for debugging use only.
        // We force POISONER role.
        OnTime(t, poisoner, PoisonerPoisons(victim)) => {
            player_must_character(r, *poisoner, Evil(Minion(Poisoner)))
                & r.is_poisoned[victim][t].clone()
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
    }
}

#[must_use]
pub fn atmost_one_player_can_be_poisoned(registry: &Registry) -> z3::ast::Bool {
    let constraints: Vec<_> = TimeIterator::new(registry.until)
        .map(|time| {
            let poisoned_vars = (0..registry.num_players)
                .map(|seat| &registry.is_poisoned[&Player::Seat(seat)][&time]);
            z3::ast::atmost(poisoned_vars, 1)
        })
        .collect();
    z3::ast::Bool::and(constraints.iter().collect_vec().as_slice())
}

#[must_use]
pub fn atmost_one_player_can_be_red_herringed(r: &Registry) -> z3::ast::Bool {
    let red_herringed = (0..r.num_players).map(|seat| &r.is_red_herring[&Player::Seat(seat)]);
    let atmost_one = z3::ast::atmost(red_herringed, 1);

    let red_herring_must_be_good = (0..r.num_players)
        .map(|seat| {
            let p = Player::Seat(seat);
            r.is_red_herring(p).implies(registers::can_good(r, p))
        })
        .collect_vec();

    atmost_one & z3::ast::Bool::and(red_herring_must_be_good.iter().collect_vec().as_slice())
}

/// We do not yet support the ability for a player to change characters.
/// There are cases when a Poisoner could become the next Imp, and that is not
/// yet handled in this case.
#[must_use]
pub fn poisoner_can_poison_one_person_only_if_alive(r: &Registry) -> z3::ast::Bool {
    use botc_core::Character::Evil;
    use botc_core::Evil::Minion;
    use botc_core::Minion::Poisoner;

    let constraints: Vec<_> = TimeIterator::new(r.until)
        .map(|time| {
            let is_poisoner_alive = assert_character_is_alive(r, Evil(Minion(Poisoner)), time);
            let is_someone_poisoned =
                util::player_any(r, |r, p, t| r.is_poisoned[&p][&t].clone(), time);
            is_poisoner_alive.iff(is_someone_poisoned)
        })
        .collect();
    z3::ast::Bool::and(constraints.iter().collect_vec().as_slice())
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
/// But, as of right now - we have modelled this `Time::{Night, Day}`, and we
/// can reconsider simplifying or extending this in the future.
pub fn poisoning_does_not_move_during_the_day(registry: &Registry) -> z3::ast::Bool {
    let days: Vec<i32> = TimeIterator::new(registry.until)
        .filter_map(|time| match time {
            Time::Day(x) => Some(x),
            Time::Night(_) => None,
        })
        .collect_vec();

    let players = (0..registry.num_players).map(Seat).collect_vec();

    let constraints: Vec<_> = players
        .iter()
        .flat_map(|player| {
            days.iter().map(move |&day| {
                let is_player_poisoned_day = &registry.is_poisoned[player][&Time::Day(day)];
                let is_player_poisoned_night = &registry.is_poisoned[player][&Time::Night(day)];
                is_player_poisoned_day.implies(is_player_poisoned_night)
            })
        })
        .collect();

    z3::ast::Bool::and(constraints.iter().collect_vec().as_slice())
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

#[must_use]
pub fn mark_characters_not_in_play(registry: &Registry, characters: &[Character]) -> z3::ast::Bool {
    let constraints: Vec<_> = characters
        .iter()
        .map(|c| {
            let players =
                (0..registry.num_players).map(|seat| registry.get(Player::Seat(seat), *c));
            z3::ast::atmost(players, 0)
        })
        .collect();
    z3::ast::Bool::and(constraints.iter().collect_vec().as_slice())
}

/// A player claims to be a character.
/// Valid if they really are that character, *or* they are a role that is
/// allowed to lie.
fn player_claims_character(r: &Registry, claimant: Player, character: Character) -> Bool {
    r.get(claimant, character) | can_lie(r, claimant)
}

fn player_must_character(r: &Registry, claimant: Player, character: Character) -> Bool {
    r.get(claimant, character).clone()
}

/// A player (e.g. Ravenskeeper, Undertaker) sees another player's token.
fn player_sees_other_players_character(
    r: &Registry,
    seer: Player,
    target: Player,
    token: Character,
    t: Time,
) -> Bool {
    is_effective(r, seer, t).implies(registers::as_token(r, target, token))
}

// Validates that a player's ablity should correctly have the right effect.
fn is_effective(r: &Registry, p: Player, t: Time) -> Bool {
    let player_can_lie: Bool = can_lie(r, p);
    let player_is_poisoned: &Bool = &r.is_poisoned[&p][&t];

    (!player_can_lie) & (!player_is_poisoned)
}

fn see_character_between_player_pair(
    r: &Registry,
    seer: Player,
    a: Player,
    b: Player,
    token: Character,
    time: Time,
) -> Bool {
    is_effective(r, seer, time)
        .implies(registers::as_token(r, a, token) | registers::as_token(r, b, token))
}

mod util {
    use botc_core::Player::Seat;
    use botc_core::{Player, Time};
    use itertools::Itertools;
    use z3::ast::Bool;

    // Returns true if a |predicate| holds true for at-least one player at time
    // |time|.
    pub fn player_any<F>(r: &super::Registry, predicate: F, time: Time) -> Bool
    where
        F: Fn(&super::Registry, Player, Time) -> Bool,
    {
        let players = (0..r.num_players).map(Seat);
        let is_player_matching = players.map(|player| predicate(r, player, time));
        Bool::or(is_player_matching.collect_vec().as_slice())
    }
}
