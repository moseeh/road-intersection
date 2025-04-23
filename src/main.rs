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
use traffic_light::{TrafficLight, LightState};
use vehicle::{Direction, Vehicle};


use road::Road;

const SAFE_DISTANCE: i32 = 100; // Minimum safe distance between vehicles

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

        for vehicle in vehicles.iter_mut() {
            vehicle.update();
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

// Function to check if a new vehicle can be safely spawned without being too close to existing vehicles
fn is_safe_to_spawn(
    vehicles: &[Vehicle],
    direction: Direction,
    last_spawn: &HashMap<Direction, i32>,
    current_frame: i32,
) -> bool {
    // Check if enough frames have passed since the last spawn in this direction
    if let Some(last_frame) = last_spawn.get(&direction) {
        if current_frame - last_frame < SAFE_DISTANCE / 2 {
            return false;
        }
    }

    // Check that the spawn location isn't too close to other vehicles
    for vehicle in vehicles {
        if vehicle.direction() == direction && !vehicle.is_safe_distance() {
            return false;
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
