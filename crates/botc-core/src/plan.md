# Trouble Brewing SAT Modeling Plan

> **Note:** This model currently does not support:
> 1. The **Scarlet Woman** role, as it causes the player-to-character mapping to change.
> 2. The **Imp's "Star-pass"** ability (the Imp killing themselves to pass the demonhood to a Minion), as it also requires dynamic role mapping.

This document outlines the strategy for modeling the "Trouble Brewing" edition of Blood on the Clocktower using a SAT solver. The goal is to determine the set of possible game states (who is which character) that are consistent with a set of claims.

## 1. Core Variables

These are the primary variables the solver will manipulate to find valid game states.

### Static Variables
-   **`Is[p][c]`**: Boolean variable. True if player $p$ is character $c$ (one of the 25 base roles).
-   **`IsRedHerring[p]`**: True if player $p$ is the good player who registers as a Demon to the Fortune Teller.
    -   *Constraint: Exactly one Good player is the Red Herring.*

### Temporal Variables (Indexed by time $t$)
Time $t$ follows the sequence: $N_1$ (Night 1), $D_1$ (Day 1), $N_2$ (Night 2), $D_2$ (Day 2)...

-   **`Poisoned[p][t]`**: True if player $p$ is currently poisoned (by a Poisoner) or is the Drunk.
-   **`Alive[p][t]`**: True if player $p$ is alive at time $t$.

## 2. Helper Definitions

These are not independent variables but are defined by the core variables or game events.

-   **`Good[p]`**: $\bigvee_{c \in GoodCharacters} Is[p][c]$
-   **`Evil[p]`**: $\bigvee_{c \in EvilCharacters} Is[p][c]$
-   **`Executed[p][t]`**: Boolean constant/input. True if player $p$ was executed at time $t$ (provided as game history).
-   **`Healthy[p][t]`**: $\neg Poisoned[p][t]$

## 3. Global Constraints

### Unique Role
$$\forall p \in P, \sum_{c \in C} Is[p][c] = 1$$

### Role Distribution (Setup)
The number of players in each category must match the game setup based on the total player count $N$. The **Baron** modifies this by replacing 2 Townsfolk with 2 Outsiders.

Let $B = \sum_{p \in P} Is[p][Baron]$.
-   $\sum_{p \in P} \sum_{c \in Townsfolk} Is[p][c] = CountTownsfolk(N) - 2B$
-   $\sum_{p \in P} \sum_{c \in Outsider} Is[p][c] = CountOutsider(N) + 2B$
-   $\sum_{p \in P} \sum_{c \in Minion} Is[p][c] = CountMinion(N)$
-   $\sum_{p \in P} \sum_{c \in Demon} Is[p][c] = 1$

### Drunkenness & Poisoning
-   **Drunk**: $\forall t, Is[p][Drunk] \implies Poisoned[p][t]$
-   **Poisoner**: $(Is[p][Poisoner] \land \neg Poisoned[p][N_n] \land PoisonerChoice[p][N_n] == target) \implies (Poisoned[target][N_n] \land Poisoned[target][D_n])$

## 4. Character Logic

Claims are made at a specific time $t$. Information is only binding if the player is the **Real Role** and **Not Poisoned**.

| Role | Claim / Ability | Variable Implications |
| :--- | :--- | :--- |
| **Washerwoman** | sees $a, b$ as Townsfolk $T$ | $(Is[p][Washerwoman] \land \neg Poisoned[p][N_1]) \implies (Is[a][T] \lor Is[b][T] \lor Is[a][Spy] \lor Is[b][Spy])$ |
| **Librarian** | sees $a, b$ as Outsider $O$ | $(Is[p][Librarian] \land \neg Poisoned[p][N_1]) \implies (Is[a][O] \lor Is[b][O] \lor Is[a][Spy] \lor Is[b][Spy])$ |
| **Investigator** | sees $a, b$ as Minion $M$ | $(Is[p][Investigator] \land \neg Poisoned[p][N_1]) \implies (Is[a][M] \lor Is[b][M] \lor Is[a][Recluse] \lor Is[b][Recluse])$ |
| **Chef** | learns $N$ pairs | $(Is[p][Chef] \land \neg Poisoned[p][N_1]) \implies \exists \text{ registrations s.t. } \text{CountPairs}(\text{Evil}) = N$ |
| **Empath** | learns $N$ at $N_n$ | $(Is[p][Empath] \land \neg Poisoned[p][N_n]) \implies \exists \text{ registrations s.t. } \text{CountEvil}(\text{Neighbors}) = N$ |
| **Fortune Teller** | picks $a, b$ at $N_n$ | $(Is[p][FortuneTeller] \land \neg Poisoned[p][N_n]) \implies (Y \iff (Is[a][Imp] \lor Is[b][Imp] \lor IsRedH[a] \lor IsRedH[b] \lor Is[a][Recluse] \lor Is[b][Recluse]))$ |
| **Undertaker** | learns $a$ is character $C$ | $(Is[p][Undertaker] \land \neg Poisoned[p][N_n] \land Executed[a][D_{n-1}]) \implies (Is[a][C] \lor Is[a][Spy] \lor Is[a][Recluse])$ |
| **Monk** | protects $a$ at $N_n$ | $(Is[p][Monk] \land \neg Poisoned[p][N_n]) \implies \neg \text{ImpKills}(a, N_n)$ |
| **Ravenkeeper** | sees $a$ as character $C$ | $(Is[p][Ravenkeeper] \land \neg Poisoned[p][N_n] \land \neg Alive[p][D_n]) \implies (Is[a][C] \lor Is[a][Spy] \lor Is[a][Recluse])$ |
| **Virgin** | $a$ nominates $p$ | $(Is[p][Virgin] \land \neg Poisoned[p][D_n] \land \text{NominatedBy}(p, a, D_n)) \implies (Executed[a][D_n] \iff (Is[a][Townsfolk] \lor (Is[a][Spy] \land \text{ST\_Decides})))$ |
| **Slayer** | shoots $a$ | $(Is[p][Slayer] \land \neg Poisoned[p][D_n] \land \text{Shoots}(p, a, D_n) \land Is[a][Imp]) \implies \neg Alive[a][D_n+1]$ |
| **Soldier** | (Passive) | $Is[p][Soldier] \implies \forall n, \neg \text{ImpKills}(p, N_n)$ |
| **Mayor** | (Passive/Win) | $\text{GoodWins} \iff (\text{AliveCount} \le 3 \land \neg \text{Execution} \land Is[Mayor][Alive])$ |
| **Mayor** | Bounce | $(Is[p][Mayor] \land \neg Poisoned[p][N_n] \land \text{ImpKills}(p, N_n)) \implies \exists a \neq p, \text{ImpKills}(a, N_n)$ |
| **Butler** | Vote constraint | (Not modeled in information constraints) |
| **Drunk** | (Passive) | See Section 3 (Drunkenness & Poisoning) |
| **Recluse** | (Passive) | $Is[p][Recluse] \implies \text{CanRegisterAs}(\text{Evil, Minion, Demon})$ |
| **Saint** | Execution | $(Is[p][Saint] \land Executed[p][D_n]) \implies \text{EvilWins}$ |
| **Poisoner** | poisons $a$ at $N_n$ | See Section 3 (Drunkenness & Poisoning) |
| **Spy** | (Passive) | $Is[p][Spy] \implies \text{CanRegisterAs}(\text{Good, Townsfolk, Outsider})$ |
| **Baron** | Setup | See Section 3 (Role Distribution) |
| **Imp** | kills $a$ at $N_n$ | $Is[p][Imp] \implies \text{ImpKills}(a, N_n)$ |
| **Execution** | $a$ is executed at $D_n$ | $Executed[a][D_n] \implies \neg Alive[a][N_{n+1}]$ |
| **Night Death** | $a$ dies at $N_n$ | $\text{Death}(a, N_n) \implies \neg Alive[a][D_n]$ |

> **Note on Alive state:** If $\neg Alive[p][t]$, then $\neg Alive[p][t+1]$ (Death is permanent).

> **Note on mis-registration:** In the table above, `CountPairs` and `CountEvil` should be treated as "Satisfiable" if there exists any valid assignment of "registers as evil/good" for the Spy and Recluse that matches the claimed count.

## 5. Proposed Implementation Steps

1.  **Variable Registry**: Map `Is[p][c]` (25 base roles) and temporal states (`Poisoned[p][t]`, `Alive[p][t]`) to SAT IDs.
2.  **Constraint Generator**: Implement "Exactly One" for roles and translate the "Binding" logic into clauses.
3.  **Solver Integration**: Use Z3 to handle counting constraints (Chef, Empath, Role Distribution) natively.

## 6. Goal
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
