# Trouble Brewing SAT Modeling Plan

> **Note:** This model currently does not support the **Scarlet Woman** role, as it is the only role in Trouble Brewing that can cause the player-to-character mapping to change during the game (when the Imp dies and the Scarlet Woman becomes the new Imp).

This document outlines the strategy for modeling the "Trouble Brewing" edition of Blood on the Clocktower using a SAT solver. The goal is to determine the set of possible game states (who is which character) that are consistent with a set of claims.

## 1. Core Variables

### Static Variables (Fixed during setup)
-   **`Is[p][c]`**: Boolean variable. True if player $p$ is character $c$.
    -   *Note: Includes 13 Townsfolk, 3 "normal" Outsiders (Butler, Saint, Recluse), 4 Minions, 1 Demon, and **13 Drunk-Townsfolk** (e.g., `Drunk_Slayer`).*
-   **`Good[p]`**: True if player $p$ is on the Good team.
-   **`Evil[p]`**: True if player $p$ is on the Evil team.

### Temporal Variables (Indexed by time $t$)
Time $t$ follows the sequence: $N_1$ (Night 1), $D_1$ (Day 1), $N_2$ (Night 2), $D_2$ (Day 2)...

-   **`Poisoned[p][t]`**: True if player $p$ is currently poisoned (e.g., by the Poisoner).
-   **`Alive[p][t]`**: True if player $p$ is alive at time $t$.
-   **`Executed[p][t]`**: True if player $p$ is executed at time $t$.

## 2. Global Constraints

### Unique Role
$$\forall p \in P, \sum_{c \in C} Is[p][c] = 1$$

### Role Properties
-   **Outsiders**: $Is[p][c]$ where $c \in \{Saint, Butler, Recluse, Drunk\_T_1, \dots, Drunk\_T_{13}\}$.
-   **Drunk Status**: If $Is[p][Drunk\_T]$, the player is functionally identical to a poisoned Townsfolk $T$.

### Poisoning (The Poisoner)
If player $p$ is the Poisoner and is not poisoned at $N_n$, and chooses $target$:
$$(Is[p][Poisoner] \land \neg Poisoned[p][N_n] \land PoisonerChoice[p][N_n] == target) \implies (Poisoned[target][N_n] \land Poisoned[target][D_n])$$

### Role Distribution (Setup)
The number of players in each category must match the game setup based on the total player count $N$. The **Baron** modifies this by replacing 2 Townsfolk with 2 Outsiders.

Let $B = \sum_{p \in P} Is[p][Baron]$.
-   $\sum_{p \in P} \sum_{c \in Townsfolk} Is[p][c] = CountTownsfolk(N) - 2B$
-   $\sum_{p \in P} \sum_{c \in Outsider} Is[p][c] = CountOutsider(N) + 2B$
-   $\sum_{p \in P} \sum_{c \in Minion} Is[p][c] = CountMinion(N)$
-   $\sum_{p \in P} \sum_{c \in Demon} Is[p][c] = 1$

## 3. Modeling Character Logic

Claims are made at a specific time $t$. Information is only binding if the player is the **Real Townsfolk** and **Not Poisoned**.

### Universal Claim Logic
If player $p$ claims to be Townsfolk $T$ and reports information $I$ at time $t$:
$$(Is[p][T] \land \neg Poisoned[p][t]) \implies I$$

### Townsfolk

-   **Washerwoman** (Claim at $N_1$):
    $$(Is[p][Washerwoman] \land \neg Poisoned[p][N_1]) \implies (Is[a][T] \lor Is[b][T] \lor Is[a][Spy] \lor Is[b][Spy])$$
-   **Librarian** (Claim at $N_1$):
    $$(Is[p][Librarian] \land \neg Poisoned[p][N_1]) \implies (Is[a][O] \lor Is[b][O] \lor Is[a][Spy] \lor Is[b][Spy])$$
-   **Investigator** (Claim at $N_1$):
    $$(Is[p][Investigator] \land \neg Poisoned[p][N_1]) \implies (Is[a][M] \lor Is[e][M] \lor Is[a][Recluse] \lor Is[e][Recluse])$$
-   **Chef** (Claim at $N_1$):
    $$(Is[p][Chef] \land \neg Poisoned[p][N_1]) \implies \text{CountPairs}(Evil) = N$$
-   **Empath** (Claim at $N_n$):
    $$(Is[p][Empath] \land \neg Poisoned[p][N_n]) \implies \sum_{n \in Neighbors(p, N_n)} (Evil[n] \lor RegisterEvil[n]) = N$$
-   **Fortune Teller** (Claim at $N_n$):
    $$(Is[p][FortuneTeller] \land \neg Poisoned[p][N_n]) \implies (Y \iff (Evil[a] \lor Evil[b] \lor RedHerring[a] \lor RedHerring[b]))$$
-   **Undertaker** (Claim at $N_n$ about execution at $D_{n-1}$):
    $$(Is[p][Undertaker] \land \neg Poisoned[p][N_n] \land Executed[a][D_{n-1}]) \implies (TrueCharacter(Is[a]) == C \lor RegisterAs[a][C])$$
    *(Note: $TrueCharacter(Drunk\_T) = Drunk$)*
-   **Virgin** (Day $n$):
    $$(Is[p][Virgin] \land \neg Poisoned[p][D_n] \land NominatedBy[p][a][D_n] \land Is[a][Townsfolk]) \implies Executed[a][D_n]$$
-   **Slayer** (Day $n$):
    $$(Is[p][Slayer] \land \neg Poisoned[p][D_n] \land Shoots[p][a][D_n] \land Is[a][Demon]) \implies \neg Alive[a][D_n+1]$$
-   **Soldier**:
    $$\forall n, Is[p][Soldier] \implies \neg (ImpKills[p][N_n])$$

### Outsiders

-   **Recluse**:
    -   $Is[p][Recluse] \implies (RegisterEvil[p] \lor RegisterMinion[p] \lor RegisterDemon[p])$
-   **Saint**:
    -   $(Is[p][Saint] \land Executed(p)) \implies EvilWins$

### Minions

-   **Poisoner**: 
    -   See Section 2 for core logic.
-   **Spy**:
    -   $Is[p][Spy] \implies (RegisterGood[p] \lor RegisterTownsfolk[p])$
    -   Sees the Grimoire (Knows all `Is[i][j]`).

## 4. Proposed Implementation Steps

1.  **Variable Registry**: Map `Is[p][c]` (including Drunk variants) and temporal states (`Poisoned[p][t]`) to SAT IDs.
2.  **Constraint Generator**: Implement "Exactly One" for roles and translate the "Binding" logic into clauses.
3.  **Solver Integration**: Use Z3 to handle counting constraints (Chef, Empath, Role Distribution) natively.

## 5. Goal
The final system should allow us to add "ReportLogs" (Claims) and then ask the solver:
-   "Is it possible that Adam is the Imp?"
-   "Given these claims, who are the guaranteed Good players?"
-   "List all possible game states (assignments of characters to players)."

## Appendix A: Character Distribution

| Players | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
| :--- | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: | :-: |
| **Townsfolk** | 3 | 3 | 5 | 5 | 5 | 7 | 7 | 7 | 9 | 9 | 9 |
| **Outsiders** | 0 | 1 | 0 | 1 | 2 | 0 | 1 | 2 | 0 | 1 | 2 |
| **Minions** | 1 | 1 | 1 | 1 | 1 | 2 | 2 | 2 | 3 | 3 | 3 |
| **Demons** | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
