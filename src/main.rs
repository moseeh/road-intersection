// src/main.rs
mod intersection;
mod road;
mod traffic_light;
mod vehicle;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::time::Duration;

use intersection::Intersection;
use traffic_light::{LightState, TrafficLight};
use vehicle::{Direction, Vehicle};

use road::Road;

const SAFE_DISTANCE: i32 = 50; // Minimum safe distance between vehicles

fn main() -> Result<(), String> {
    // Initialize SDL2 context and video subsystem
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window and canvas
    let window = video_subsystem
        .window("Road Intersection", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?; // <-- map_err here

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Instantiate our modular components
    let road_ns = Road::new_vertical(350, 0, 100, 800); // North‑South road 
    let road_ew = Road::new_horizontal(0, 350, 800, 100); // East‑West road 
    let intersection = Intersection::new(road_ns, road_ew);

    let mut light_s = TrafficLight::new(460, 460, 20, 20, LightState::Red);
    let mut light_w = TrafficLight::new(320, 460, 20, 20, LightState::Green);
    let mut light_n = TrafficLight::new(320, 320, 20, 20, LightState::Red);
    let mut light_e = TrafficLight::new(460, 320, 20, 20, LightState::Green);
    let mut vehicles = Vec::new();

    // Track the last spawn time for each direction to enforce safe distance
    let mut last_spawn = HashMap::new();
    last_spawn.insert(Direction::North, 0);
    last_spawn.insert(Direction::South, 0);
    last_spawn.insert(Direction::East, 0);
    last_spawn.insert(Direction::West, 0);

    let mut rng = rand::rng();
    let mut event_pump = sdl_context.event_pump()?;
    let mut n = 0;
    let mut frame_count = 0;
    let mut is_green = false;

    'running: loop {
        frame_count += 1;

        // Input handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let direction = match keycode {
                        Keycode::Up => Some(Direction::North), // From south heading north
                        Keycode::Down => Some(Direction::South), // From north heading south
                        Keycode::Left => Some(Direction::West), // From east heading west
                        Keycode::Right => Some(Direction::East), // From west heading east
                        Keycode::R => {
                            // Random direction
                            let random_dir = match rng.random_range(0..4) {
                                0 => Direction::North,
                                1 => Direction::South,
                                2 => Direction::East,
                                3 => Direction::West,
                                _ => unreachable!(),
                            };
                            Some(random_dir)
                        }
                        Keycode::Escape => break 'running, // Exit the simulation
                        _ => None,
                    };

                    // If a valid direction key was pressed, try to spawn a vehicle
                    if let Some(dir) = direction {
                        // Check if it's safe to spawn a new vehicle (not too close to existing ones)
                        if is_safe_to_spawn(&vehicles, dir, &last_spawn, frame_count) {
                            vehicles.push(Vehicle::new(dir));
                            last_spawn.insert(dir, frame_count);
                        }
                    }
                }
                _ => {}
            }
        }

        // Update logic
        n += 1;
        if n > 500 {
            is_green = !is_green;
            light_n.update(is_green);
            light_s.update(is_green);
            light_e.update(!is_green);
            light_w.update(!is_green);
            n = 0;
        }
        // Inside the main loop's update section:

        // Compute tentative positions (with traffic light checks)
        let  tentatives: Vec<Vehicle> = vehicles
            .iter()
            .map(|v| {
                let mut tentative_v = v.clone();
                let light_state = match tentative_v.direction() {
                    Direction::North => light_s.state(),
                    Direction::South => light_n.state(),
                    Direction::East => light_w.state(),
                    Direction::West => light_e.state(),
                };

                // Only move if not stopped by light or vehicle ahead
                tentative_v.update(light_state);
                tentative_v
            })
            .collect();

        // Check safe distances between vehicles in the same direction
        let mut safe_to_move = vec![true; tentatives.len()];
        for i in 0..tentatives.len() {
            let current = &tentatives[i];
            let dir = current.direction();

            // Find closest vehicle ahead in the same direction
            let closest_ahead = tentatives
                .iter()
                .enumerate()
                .filter(|(j, other)| *j != i && other.direction() == dir)
                .filter_map(|(_, other)| {
                    let distance = match dir {
                        Direction::North => {
                            current.rect.y() - (other.rect.y() + other.rect.height() as i32)
                        }
                        Direction::South => {
                            other.rect.y() - (current.rect.y() + current.rect.height() as i32)
                        }
                        Direction::East => {
                            other.rect.x() - (current.rect.x() + current.rect.width() as i32)
                        }
                        Direction::West => {
                            current.rect.x() - (other.rect.x() + other.rect.width() as i32)
                        }
                    };
                    if distance >= 0 { Some(distance) } else { None }
                })
                .min();

            // Block movement if vehicle is too close
            if let Some(distance) = closest_ahead {
                if distance < SAFE_DISTANCE {
                    safe_to_move[i] = false;
                }
            }
        }

        // Also check for collisions
        let mut collisions = vec![false; tentatives.len()];
        for i in 0..tentatives.len() {
            for j in (i + 1)..tentatives.len() {
                if tentatives[i].rect().has_intersection(tentatives[j].rect()) {
                    collisions[i] = true;
                    collisions[j] = true;
                }
            }
        }

        // Update original vehicles only if safe
        for (i, vehicle) in vehicles.iter_mut().enumerate() {
            if safe_to_move[i] && !collisions[i] {
                *vehicle = tentatives[i].clone();
            }
        }
        // Remove vehicles that have left the screen
        vehicles.retain(|v| is_on_screen(v));

        // Drawing
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        road_ns.draw(&mut canvas);
        road_ew.draw(&mut canvas);
        intersection.draw(&mut canvas);
        // Draw lights on top
        light_n.draw(&mut canvas);
        light_s.draw(&mut canvas);
        light_e.draw(&mut canvas);
        light_w.draw(&mut canvas);
        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn is_safe_to_spawn(
    vehicles: &[Vehicle],
    direction: Direction,
    last_spawn: &HashMap<Direction, i32>,
    current_frame: i32,
) -> bool {
    // Enforce minimum time between spawns in the same direction
    if let Some(last_frame) = last_spawn.get(&direction) {
        if current_frame - last_frame < SAFE_DISTANCE / 2 {
            return false;
        }
    }

    // Calculate spawn point based on direction
    let (spawn_coord, is_vertical) = match direction {
        Direction::North => (800, true), // Spawn at bottom (y=800)
        Direction::South => (-40, true), // Spawn at top (y=-40)
        Direction::East => (-40, false), // Spawn at left (x=-40)
        Direction::West => (800, false), // Spawn at right (x=800)
    };

    // Check distance from existing vehicles in the same direction
    for vehicle in vehicles.iter().filter(|v| v.direction() == direction) {
        let vehicle_pos = if is_vertical {
            vehicle.rect.y() + vehicle.rect.height() as i32
        } else {
            vehicle.rect.x() + vehicle.rect.width() as i32
        };

        let distance = (vehicle_pos - spawn_coord).abs();
        if distance < SAFE_DISTANCE {
            return false; // Existing vehicle too close to spawn point
        }
    }

    true
}

// Function to check if a vehicle is still on screen
fn is_on_screen(vehicle: &Vehicle) -> bool {
    let rect = vehicle.rect();
    match vehicle.direction() {
        Direction::North => rect.y() > -50, // Allow vehicle to fully leave the screen
        Direction::South => rect.y() < 850,
        Direction::East => rect.x() < 850,
        Direction::West => rect.x() > -50,
    }
}
