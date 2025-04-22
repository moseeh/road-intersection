use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use crate::traffic_light::{TrafficSystem, Direction};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Turn { Left, Right, Straight }

#[derive(Clone)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    // pub _turn: Turn,
    pub color: Color,
    pub speed: i32,
}

impl Vehicle {
    pub fn new(direction: Direction) -> Self {
        let mut rng = rand::thread_rng();
        let turn = match rng.gen_range(0..3) {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => Turn::Straight,
        };

        let color = match turn {
            Turn::Left => Color::YELLOW,
            Turn::Right => Color::GREEN,
            Turn::Straight => Color::RED,
        };

        let (x, y) = match direction {
            Direction::North => (390, 0),
            Direction::South => (410, 800),
            Direction::East => (0, 390),
            Direction::West => (800, 410),
        };

        Vehicle { x, y, direction, color, speed: 2 }
    }

    pub fn update(&mut self, traffic: &TrafficSystem, _vehicles: &[Vehicle]) {
        // Move if green or inside intersection (basic logic)
        if traffic.can_move(self.direction) {
            match self.direction {
                Direction::North => self.y += self.speed,
                Direction::South => self.y -= self.speed,
                Direction::East => self.x += self.speed,
                Direction::West => self.x -= self.speed,
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let rect = Rect::new(self.x, self.y, 10, 10);
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(rect);
    }
}