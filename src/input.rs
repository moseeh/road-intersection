use sdl2::keyboard::Keycode;
use rand::seq::SliceRandom;
use crate::{vehicle::Vehicle, traffic_light::Direction};

pub struct InputManager;

impl InputManager {
    pub fn new() -> Self { Self }

    pub fn handle_key(&self, key: Keycode) -> Option<Vehicle> {
        match key {
            Keycode::Up => Some(Vehicle::new(Direction::South)),
            Keycode::Down => Some(Vehicle::new(Direction::North)),
            Keycode::Left => Some(Vehicle::new(Direction::East)),
            Keycode::Right => Some(Vehicle::new(Direction::West)),
            Keycode::R => {
                let dirs = [Direction::North, Direction::South, Direction::East, Direction::West];
                let dir = *dirs.choose(&mut rand::thread_rng()).unwrap();
                Some(Vehicle::new(dir))
            },
            _ => None
        }
    }
}