# Lifetime — Spawn Orchestration

**Responsibility:** Responsible for "first generation of this level": issuing player generation request; collecting player generation receipts; issuing rope generation request between adjacent players when all the players are present.

**NOT:** Not responsible for the specific construction, rendering, and animation of the player/rope; not responsible for physics and collision.

## Public API

* **Events**
  * `Lifetime2PlayerSpawn { node, texture, position, controls, mass }`
  * `Lifetime2RopeSpawn { head_entity, tail_entity }`
* **Plugins**
  * `LifetimePlugin`
* **Components**: None
* **Resources**: None

## Data model

* **Component/Resource**

  * `SpawnTrack`:

    * `expected_players: usize`
    * `spawned_players: usize`
    * `node_to_entity: Vec<Option<Entity>>`
    * `is_rope: bool`

* **Event flow**

  * Lifetime `queue_for_player_setup_event` → `Lifetime2PlayerSpawn`
  * Player → `Player2LifetimeSpawned`
  * Lifetime `wait_for_player_spawn`→ `Lifetime2RopeSpawn`

## Scheduling diagram

### Plugins

* **ControlPlugin** — Reads and aggregates input, producing intent events like `PlayerIntent*` (for physics/player use).
* **PlayerPlugin** — Receives `Lifetime2PlayerSpawn`, actually spawns the player entity, and sends back `Player2LifetimeSpawned`.
* **RopePlugin** — Receives `Lifetime2RopeSpawn`, spawns and maintains the rope entity (physics/visual updates, etc.).

### Systems

* **Startup → `queue_for_player_setup_event`**
At startup, issues the `Lifetime2PlayerSpawn` request required for this level (configured by node/position/map/control), and initializes the `SpawnTrack` counter.

* **Update → `wait_for_player_spawn`**
Continuously monitors `Player2LifetimeSpawned` receipts and populates `SpawnTrack`. When all players arrive, issue `Lifetime2RopeSpawn` (sent only once) based on adjacent node pairings.

> Key execution sequence:
>
> 1. `Startup` first issues a player spawn request → 2) `PlayerPlugin` spawns and receives a receipt → 3) `Update` receives all receipts and then issues a rope spawn request, which is then taken over by `RopePlugin`.

## Invariants & constraints

* `node_to_entity.len() == expected_players`
* Each `node` is registered only once; duplicate receipts are ignored.
* Rope events are only emitted if `spawned_players == expected_players` and `is_rope == false`.
* Ropes connect **adjacent nodes** `(0,1)…(n-2,n-1)` by default.

## Test checklist

* Initialization: `SpawnTrack` field is correct
* 2/3 person scenario: Only send the correct number of `Lifetime2RopeSpawn` once receipts are complete
* Exception: Duplicate/out-of-bounds `node` does not break the count; ropes are not sent if all receipts are complete

## Common change flows

* **Change the number of players**: Set `expected_players` and reset `node_to_entity`, then add or remove characters in `player_spawn`.
* **Change pairing**: Change adjacent pairings to custom edge sets.
