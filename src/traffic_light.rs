use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::time::{Instant, Duration};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { North, South, East, West }

pub struct TrafficSystem {
    pub current: Direction,
    pub last_change: Instant,
    pub interval: Duration,
}

impl TrafficSystem {
    pub fn new() -> Self {
        TrafficSystem {
            current: Direction::North,
            last_change: Instant::now(),
            interval: Duration::from_secs(4),
        }
    }

    pub fn update(&mut self) {
        if self.last_change.elapsed() >= self.interval {
            self.current = match self.current {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            self.last_change = Instant::now();
        }
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        self.current == direction
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        // Draw 4 traffic lights depending on direction
        let light_color = Color::GREEN;
        let red_color = Color::RED;

        let lights = [
            (Direction::North, Rect::new(390, 300, 20, 20)),
            (Direction::South, Rect::new(390, 480, 20, 20)),
            (Direction::East, Rect::new(300, 390, 20, 20)),
            (Direction::West, Rect::new(480, 390, 20, 20)),
        ];

        for (dir, rect) in lights.iter() {
            canvas.set_draw_color(if *dir == self.current { light_color } else { red_color });
            let _ = canvas.fill_rect(*rect);
        }
    }
}
