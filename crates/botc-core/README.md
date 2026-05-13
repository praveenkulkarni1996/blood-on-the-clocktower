# botc-core

`botc-core` is the foundational library for the [Blood on the Clocktower](https://bloodontheclocktower.com/) solver. It provides pure Rust enumerations and data types representing the state, roles, and events within a game of Blood on the Clocktower. It is specifically modeled with the ["Trouble Brewing"](https://wiki.bloodontheclocktower.com/Trouble_Brewing) script in mind. For more detailed rules and role interactions, reference the [official wiki](https://wiki.bloodontheclocktower.com/Main_Page).

## Overview

This crate defines the core domain models:
- **Characters:** Contains enumerations for `Townsfolk`, `Outsider`, `Minion`, and `Demon` roles, layered generically into `Good` and `Evil` alignments.
- **Time Representation:** Implements custom logic for navigating game phases (e.g. `Time::Night`, `Time::Day`), abstracting the standard 1-based indexing sequence utilized natively by Blood on the Clocktower gameplay.
- **Game Events and Claims (`ReportLog`):** Defines the vast array of different possibilities that can occur in the game, from individual claims made by specific roles (`WasherwomanSees`, `EmpathLearnsOne`, etc.) to publicly verifiable events like executions (`DayExecutes`) and night deaths (`NightKilled`).

## Architecture and Modeling

For detailed information regarding how these components map to Logical SAT formulas used by solver engines, please review the mathematical models defined in [`src/plan.md`](src/plan.md).

## Usage

This crate is meant to be consumed by logic engines—such as the Z3 solver implemented in `botc-solver`—that can map these strictly-defined enumerations and events to logical constraints to compute possible world states and deduce valid assignments.
