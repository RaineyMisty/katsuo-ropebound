# Rope Module

## Features

### Create Rope `spawn`

* Create a rope entity
* Send rope info to request physics attachment

### Move Rope `move_rope`

* Update rope position coordinates

## API

* **Event**: `Lifetime2RopeSpawn`, `Rope2PhysicsAttach`
* **Plugin** `RopePlugin`
* **Comopnent** `EndPoint`, `EndPoints`, `Rope`
  * We'd better not use component here. However, saving EndPoints twice is a worse solution. Waiting for better refactoring.

## Changes

* `rope`
* `physics/rope.rs`

## Bug

* **[20251024]** The force of the rope will increase infinitely as the number of times it is pulled increases; **[20251030]** Fixed.
  * NEVER use `GlobalTranslation` in physics calculation.
  * **Reason**: `GlobalTranslation` will be update in the PostUpdate process, so the rope system will read previous position to solve the rope force.
  * There is also a remainer that we didn't check if the energy will increace in the system.
* **[20251026]** The rope’s physics effect is applied twice during runtime; Ignored.

## To-Do

* Implement rope visualization mounting