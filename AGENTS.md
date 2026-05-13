# BotC Solver: AI Agent Instructions

Welcome, Agent. This codebase is a Rust-based tool that uses the Z3 theorem prover to solve the social deduction game "Blood on the Clocktower" (specifically for the "Trouble Brewing" edition).

## Project Overview
This repository uses a Cargo workspace and contains two primary crates:
- `botc-core/`: Contains the pure Rust enumerations and data models defining the game's Time, Roles, Characters (Townsfolk, Outsiders, Minions, Demons) and Claims. It provides the `ReportLog` definitions representing in-game events.
- `botc-solver/`: Uses the `z3` crate to compile the `ReportLog` sequences into logical SAT/SMT constraints. It bounds `botc-core` states to Z3 boolean variables inside a `Registry`.

## Architectural Notes
- The mathematical logic, assumptions, and constraint mappings for the SAT solver are thoroughly documented in `crates/botc-core/src/plan.md`. This is the single most important architectural document for the logic, review it before making changes to any constraint mappings.
- Game state variables are dimensioned through the `Registry` (e.g., `is_character[player][character]`, `is_alive[player][time]`, `is_poisoned[player][time]`).
- The Z3 constraint bridging code lives primarily in `botc-solver/src/lib.rs`.

## Useful Commands
- **Linting:** Make sure to run `cargo clippy`. The codebase adheres to `clippy::pedantic` which is denied globally using `#![deny(clippy::pedantic)]` in root source files. Avoid triggering pedantic warnings.
- **Testing Check:** Run `cargo test` from the root workspace.
- **Run Solver Examples:** `cargo run -p botc-solver --example reddit`

## Guidelines for Modifying Code
1. **Adding New Roles/Claims:** If you have to support a new script or role, start by adding it to the enums in `botc-core`. Afterward, translate the ability's logic into Z3 constraints in the `constrain` match block found in `botc-solver`.
2. **Night vs Day Iteration:** Time relies on 1-based indices (e.g., Night-1, Day-1, Night-2). The execution of the game happens linearly through these phases.
3. **Rust Best Practices:** Continue to leverage Cargo's robust ecosystems. Rely heavily on strong typing since Z3 uses algebraic abstractions not typical runtime assertions. 
4. **Panic Assertions:** Some logical flow errors natively `panic!` (e.g., trying to process a "DayExecutes" event mathematically during a "Night" phase). This is by design to catch solver misconfigurations early.
