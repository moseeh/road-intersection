# ROAD INTERSECTION

This is a realistic traffic intersection simulation built with Rust and SDL2, featuring vehicle spawning, traffic light control, and collision avoidance.

## Features

### Core Components
- **Road System**:
  - Vertical (North-South) and horizontal (East-West) roads
  - 4-lane intersection with proper dimensions (800x800 window)
- **Traffic Lights**:
  - Four lights (N, S, E, W) with Red/Green states
  - Cyclic switching every 150 frames (≈2.5 seconds at 60fps)
  - All-way red phase during transitions
- **Vehicles**:
  - Direction-based spawning (North, South, East, West)
  - Color-coded by direction
  - Fixed velocity and safe following distance (50px)
  - Automatic stopping for red lights and preceding vehicles

### Advanced Systems
- **Collision Prevention**:
  - Tentative movement prediction
  - Intersection zone handling
  - Direction-specific distance checks
- **Safe Spawning**:
  - Minimum 25-frame cooldown between spawns
  - Distance checks from existing vehicles
  - Direction-specific spawn points

## Installation

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs/))
- SDL2 development libraries

### Build & Run
```bash
cargo build --release
cargo run --release
```

## Controls

| Key       | Action                          |
|-----------|---------------------------------|
| ↑         | Spawn northbound vehicle        |
| ↓         | Spawn southbound vehicle        |
| ←         | Spawn westbound vehicle         |
| →         | Spawn eastbound vehicle         |
| R         | Spawn random-direction vehicle  |
| Esc       | Quit simulation                 |

## Technical Implementation

### Core Modules

```rust
src/
├── main.rs          # SDL setup, game loop, input handling
├── intersection.rs  # Intersection logic and rendering
├── road.rs          # Road geometry and drawing
├── traffic_light.rs # Light states and timing
└── vehicle.rs       # Vehicle behavior and physics
```

## Configuration

| Constant            | Value | Description                      |
|---------------------|-------|----------------------------------|
| `SAFE_DISTANCE`     | 50    | Minimum distance between vehicles |
| `LIGHT_SWITCH_FREQ` | 150   | Frames between light changes      |

## Future Improvements

- [ ] Add turning animations
- [ ] Implement dynamic light timing
- [ ] Add pedestrian crossings
- [ ] Support emergency vehicles

## Dependencies

```toml
[dependencies]
sdl2 = "0.37.0"
rand = "0.9"
```
## Resources

- [SDL2 Documentation](https://docs.rs/sdl2/latest/sdl2/)
- [Rust-SDL2 Guide](https://github.com/Rust-SDL2/rust-sdl2)
- [Traffic Simulation Theory (Wikipedia)](https://en.wikipedia.org/wiki/Traffic_simulation)

## Contributors

-  [Moses Onyango](https://github.com/moonyango)
- [Alice Okingo](https://github.com/aokingo)
- [Doreen Onyango](https://github.com/doonyango)
