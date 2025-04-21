use sdl2::{event::Event, keyboard::Keycode};
use crate::{simulation::Simulation, vehicle::Direction};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_event(&mut self, event: Event, simulation: &mut Simulation) {
        if let Event::KeyDown { keycode: Some(key), .. } = event {
            match key {
                Keycode::Up => {
                    simulation.spawn_vehicle(Direction::South).unwrap_or_default();
                },
                Keycode::Down => {
                    simulation.spawn_vehicle(Direction::North).unwrap_or_default();
                },
                Keycode::Left => {
                    simulation.spawn_vehicle(Direction::East).unwrap_or_default();
                },
                Keycode::Right => {
                    simulation.spawn_vehicle(Direction::West).unwrap_or_default();
                },
                Keycode::R => {
                    let dir = match rand::random::<u8>() % 4 {
                        0 => Direction::North,
                        1 => Direction::South,
                        2 => Direction::East,
                        _ => Direction::West,
                    };
                    simulation.spawn_vehicle(dir).unwrap_or_default();
                },
                _ => {}
            }
        }
    }
}