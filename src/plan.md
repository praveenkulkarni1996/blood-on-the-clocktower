# Trouble Brewing SAT Modeling Plan

> **Note:** This model currently does not support the **Scarlet Woman** role, as it is the only role in Trouble Brewing that can cause the player-to-character mapping to change during the game (when the Imp dies and the Scarlet Woman becomes the new Imp).

This document outlines the strategy for modeling the "Trouble Brewing" edition of Blood on the Clocktower using a SAT solver. The goal is to determine the set of possible game states (who is which character) that are consistent with a set of claims.

## 1. Core Variables

For each player $p \in P$ and each character $c \in C$:

-   **`X[p][c]`**: Boolean variable. True if player $p$ is character $c$.
-   **`Good[p]`**: True if player $p$ is on the Good team.
    -   `Good[p]` $\iff \bigvee_{c \in GoodCharacters}$ `X[p][c]`
-   **`Evil[p]`**: True if player $p$ is on the Evil team.
    -   `Evil[p]` $\iff \bigvee_{c \in EvilCharacters}$ `X[p][c]`
-   **`Healthy[p]`**: True if player $p$ is not poisoned or drunk.
    -   This is a dynamic state that depends on other variables (e.g., Poisoner's target, being the Drunk).

## 2. Global Constraints

### Unique Role
Each player must have exactly one role:
$$\forall p \in P, \sum_{c \in C} X[p][c] = 1$$

### Role Distribution (Setup)
The number of players in each category must match the game setup based on the total player count $N$. The **Baron** modifies this by replacing 2 Townsfolk with 2 Outsiders.

Let $B = \sum_{p \in P} X[p][Baron]$ (which is either 0 or 1).
-   $\sum_{p \in P} \sum_{c \in Townsfolk} X[p][c] = CountTownsfolk(N) - 2B$
-   $\sum_{p \in P} \sum_{c \in Outsider} X[p][c] = CountOutsider(N) + 2B$
-   $\sum_{p \in P} \sum_{c \in Minion} X[p][c] = CountMinion(N)$
-   $\sum_{p \in P} \sum_{c \in Demon} X[p][c] = 1$

## 3. Modeling Character Logic

Claims made by players are only guaranteed to be true if the player is **Good** and **Healthy**.

### Investigator Example
"The Investigator ($p$) claims that either Adam ($a$) or Eve ($e$) is the Poisoner ($M$)."

If we assume player $p$ is the Investigator and is healthy:
$$(X[p][Investigator] \land Healthy[p]) \implies (X[a][M] \lor X[e][M] \lor X[a][Recluse] \lor X[e][Recluse])$$
*(Note: Recluse can register as a Minion)*

### Washerwoman Example
"The Washerwoman ($p$) claims that either Alice ($a$) or Bob ($b$) is the Empath ($T$)."
$$(X[p][Washerwoman] \land Healthy[p]) \implies (X[a][T] \lor X[b][T] \lor X[a][Spy] \lor X[b][Spy])$$
*(Note: Spy can register as Townsfolk)*

### Chef Example
"The Chef ($p$) claims they see $N$ pairs of evil players."
$$(X[p][Chef] \land Healthy[p]) \implies \text{CountPairs}(Evil) = N$$

## 4. Drunkenness and Poisoning

-   **The Drunk (Outsider)**: Thinks they are a Townsfolk.
    -   If `X[p][Drunk]`, then `Healthy[p]` is False.
    -   $p$ will believe they are some $c \in Townsfolk$.
-   **Poisoner**:
    -   If `X[p][Poisoner]` and $p$ poisons $target$, then `Healthy[target]` is False for that duration.

## 5. Proposed Implementation Steps

1.  **Variable Registry**: Create a system to map `X[p][c]` pairs and auxiliary states (like `Healthy[p]`) to unique integer IDs for a SAT solver (e.g., `varisat`).
2.  **Constraint Generator**:
    -   Implement "At Least One" and "Exactly One" constraints using CNF.
    -   Translate the high-level `Claim` enum and `CompoundConstraint` structures into SAT clauses.
3.  **Solver Integration**:
    -   Feed generated clauses into the solver.
    -   Provide methods to query if a specific $(p, c)$ assignment is possible.
4.  **Refining "Healthy"**:
    -   Model night-by-night interactions to track the $Healthy$ state accurately, especially for the Poisoner and characters like the Virgin or Monk.

## 6. Goal
The final system should allow us to add "ReportLogs" (Claims) and then ask the solver:
-   "Is it possible that Adam is the Imp?"
-   "Given these claims, who are the guaranteed Good players?"
-   "List all possible game states (assignments of characters to players)."

## Appendix A: Character Distribution

The following table shows the standard character distribution based on the number of players ($N$).

| Players | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
| :--- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| **Townsfolk** | 3 | 3 | 5 | 5 | 5 | 7 | 7 | 7 | 9 | 9 | 9 |
| **Outsiders** | 0 | 1 | 0 | 1 | 2 | 0 | 1 | 2 | 0 | 1 | 2 |
| **Minions** | 1 | 1 | 1 | 1 | 1 | 2 | 2 | 2 | 3 | 3 | 3 |
| **Demons** | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
