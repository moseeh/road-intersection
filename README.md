# Traffic Intersection Simulation

A traffic simulation program that models vehicles navigating through an intersection controlled by traffic lights.

## Features

- Intersection with two roads, each having two lanes (one in each direction)
- Traffic lights that automatically cycle between red and green to control traffic flow
- Vehicles that follow predefined routes (left, right, straight)
- Collision avoidance system
- Color-coded vehicles based on their routes
- Smooth animations and turning mechanics

## Controls

- **Up Arrow:** Spawn a vehicle from the south heading north
- **Down Arrow:** Spawn a vehicle from the north heading south  
- **Left Arrow:** Spawn a vehicle from the east heading west
- **Right Arrow:** Spawn a vehicle from the west heading east
- **R:** Spawn a vehicle from a random direction
- **Esc:** Close the simulation

## Requirements

- Rust (latest stable version)
- SDL2 libraries

## Installation

### 1. Install Rust
If you don't have Rust installed, follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### 2. Install SDL2 dependencies

#### On Ubuntu/Debian:
```bash
sudo apt-get install libsdl2-dev libsdl2-image-dev
```

#### On macOS:
```bash
brew install sdl2 sdl2_image
```

#### On Windows:
Follow the instructions at [SDL2 crate documentation](https://github.com/Rust-SDL2/rust-sdl2#windows).

### 3. Run the project
```bash
cargo run --release
```

## Adding assets (optional)

For enhanced graphics, you can add image assets to the `assets/` directory:
- `car.png` - Vehicle texture
- `traffic_light_red.png` - Red traffic light texture
- `traffic_light_green.png` - Green traffic light texture

If no assets are found, the program will fall back to simple shapes.

## How It Works

The simulation uses a traffic light algorithm that alternates between north-south and east-west traffic flows. Vehicles are color-coded:
- Red: Left-turning vehicles
- Green: Straight-going vehicles
- Blue: Right-turning vehicles

Vehicles maintain a safe distance from each other and avoid collisions by properly responding to traffic lights and other vehicles ahead of them.