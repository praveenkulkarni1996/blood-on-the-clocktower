#![warn(clippy::pedantic)]
use botc_core::Character::{Evil, Good};
use botc_core::Demon::Imp;
use botc_core::Evil::{Minion, Demon};
use botc_core::Good::{Outsider, Townsfolk};
use botc_core::Minion::{Baron, Poisoner, ScarletWoman, Spy};
use botc_core::Outsider::{Recluse, Saint, Butler, Drunk};
use botc_core::Townsfolk::{Washerwoman, Librarian, Investigator, Chef, Empath, FortuneTeller, Undertaker, Monk, Ravenkeeper, Virgin, Slayer, Soldier, Mayor};

#[must_use] 
pub fn must_evil(r: &super::Registry, p: &botc_core::Player) -> z3::ast::Bool {
    r.get(*p, Evil(Minion(Baron)))
        | r.get(*p, Evil(Minion(Poisoner)))
        | r.get(*p, Evil(Minion(ScarletWoman)))
        | r.get(*p, Evil(Demon(Imp)))
}

#[must_use] 
pub fn can_evil(r: &super::Registry, p: &botc_core::Player) -> z3::ast::Bool {
    must_evil(r, p) | r.get(*p, Good(Outsider(Recluse))) | r.get(*p, Evil(Minion(Spy)))
}

#[must_use] 
pub fn can_good(r: &super::Registry, p: &botc_core::Player) -> z3::ast::Bool {
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

#[must_use] 
pub fn must_townsfolk(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    r.get(p, Good(Townsfolk(Washerwoman)))
        | r.get(p, Good(Townsfolk(Librarian)))
        | r.get(p, Good(Townsfolk(Investigator)))
        | r.get(p, Good(Townsfolk(Chef)))
        | r.get(p, Good(Townsfolk(Empath)))
        | r.get(p, Good(Townsfolk(FortuneTeller)))
        | r.get(p, Good(Townsfolk(Undertaker)))
        | r.get(p, Good(Townsfolk(Monk)))
        | r.get(p, Good(Townsfolk(Ravenkeeper)))
        | r.get(p, Good(Townsfolk(Virgin)))
        | r.get(p, Good(Townsfolk(Slayer)))
        | r.get(p, Good(Townsfolk(Soldier)))
        | r.get(p, Good(Townsfolk(Mayor)))
}

#[must_use] 
pub fn can_townsfolk(r: &super::Registry, p: &botc_core::Player) -> z3::ast::Bool {
    must_townsfolk(r, *p) | r.get(*p, Evil(Minion(Spy)))
}

#[must_use] 
pub fn must_demon(r: &super::Registry, p: botc_core::Player) -> z3::ast::Bool {
    r.get(p, Evil(Demon(Imp))).clone()
}

#[must_use] 
pub fn can_demon(r: &super::Registry, p: &botc_core::Player) -> z3::ast::Bool {
    must_demon(r, *p) | r.get(*p, Good(Outsider(Recluse)))
}

#[must_use] 
pub fn as_token(
    r: &super::Registry,
    p: botc_core::Player,
    token: botc_core::Character,
) -> z3::ast::Bool {
    let is_token = r.get(p, token);
    let is_spy = r.get(p, Evil(Minion(Spy)));
    let is_recluse = r.get(p, Good(Outsider(Recluse)));

    match token {
        Good(_) => is_token | is_spy,
        Evil(_) => is_token | is_recluse,
    }
}
