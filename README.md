# ROAD INTERSECTION

This project simulates a traffic intersection using the Rust programming language and the `sdl2` graphics library.
It reduces traffic congestion and avoids vehicle collisions by implementing a simple traffic control strategy with traffic lights and vehicle behavior modeling.

## Overview

The simulation includes:
- Two intersecting roads (vertical and horizontal), each with one lane per direction.
- Vehicles that spawn from any direction (North, South, East, or West).
- Vehicles that follow a predetermined route: left turn, right turn, or straight.
- Traffic lights with red and green signals.
- Safety rules to avoid collisions and maintain safe distances.
- Keyboard controls to spawn vehicles interactively.

```
                North
            |  ↓  |  ↑  |
            |  ↓  |  ↑  |
            |     |     |
            |     |     |
            |     |     |
            |     |     |
 ___________|     |     |__________
 ← ←                             ← ←
East -----------     ----------- West
 → →                             → →
 ___________     ___________
            |     |     |
            |     |     |
            |     |     |
            |     |     |
            |     |     |
            |  ↓  |  ↑  |
            |  ↓  |  ↑  |
                South
```

## Controls

| Key       | Action                                                                 |
|-----------|------------------------------------------------------------------------|
| ↑         | Spawn a vehicle from the **South** towards North                       |
| ↓         | Spawn a vehicle from the **North** towards South                       |
| ←         | Spawn a vehicle from the **East** towards West                         |
| →         | Spawn a vehicle from the **West** towards East                         |
| `r`       | Spawn a vehicle from a **random direction and random route**           |
| `Esc`     | Quit the simulation                                                    |

**Note** Vehicle spamming is disabled. Vehicles are only added if there's a safe gap between existing ones.

## Vehicle Behavior

- Vehicles maintain **fixed speed** and keep a **safe distance** from the car in front.
- Each vehicle follows a route it is assigned at spawn time (left, straight, or right).
- Vehicles will **obey traffic lights**: stop at red, move at green.
- Color codes differentiate vehicle routes (e.g. right-turn = Yellow, left-turn = Blue, etc).

## Traffic Lights

- Positioned at the entrance of each lane at the intersection.
- Alternate red and green lights using a simple timer-based algorithm.
- Ensure **no more than one non-conflicting direction proceeds at once**.
- Primary goal: **avoid intersection collisions and reduce congestion**.

## Requirements

- Rust (latest stable)
- SDL2 development libraries

### Linux / Mac:
```bash
sudo apt install libsdl2-dev
```

## Build & Run

```bash
cargo build 
cargo run
```
