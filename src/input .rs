use sdl2::keyboard::Keycode;
use crate::simulation::Simulation;
use crate::vehicle::VehicleDirection;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {}
    }
    
    pub fn handle_key_press(&self, keycode: Keycode, simulation: &mut Simulation) {
        match keycode {
            Keycode::Up => {
                // Spawn vehicle from South heading North
                simulation.spawn_vehicle(VehicleDirection::South);
            },
            Keycode::Down => {
                // Spawn vehicle from North heading South
                simulation.spawn_vehicle(VehicleDirection::North);
            },
            Keycode::Left => {
                // Spawn vehicle from East heading West
                simulation.spawn_vehicle(VehicleDirection::East);
            },
            Keycode::Right => {
                // Spawn vehicle from West heading East
                simulation.spawn_vehicle(VehicleDirection::West);
            },
            Keycode::R => {
                // Spawn from random direction
                simulation.spawn_random_vehicle();
            },
            _ => {}
        }
    }
}