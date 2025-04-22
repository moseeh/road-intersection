use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq)] // for easy comparisons/logging :contentReference[oaicite:4]{index=4}
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Vehicle {
    rect: Rect,
    direction: Direction,
    velocity: i32,
    color: Color,
}

impl Vehicle {
    pub fn new(direction: Direction) -> Self {
        let velocity = 2;
        let (rect, color) = match direction {
            Direction::North => (Rect::new(415, 800, 20, 40), Color::BLUE),
            Direction::South => (Rect::new(365, -40, 20, 40), Color::YELLOW),
            Direction::East => (Rect::new(-40, 415, 40, 20), Color::MAGENTA),
            Direction::West => (Rect::new(800, 365, 40, 20), Color::CYAN),
        };
        Vehicle {
            rect,
            direction,
            velocity,
            color,
        }
    }
    pub fn update(&mut self) {
        match self.direction {
            Direction::North => self.rect.offset(0, -self.velocity),
            Direction::South => self.rect.offset(0, self.velocity),
            Direction::East => self.rect.offset(self.velocity, 0),
            Direction::West => self.rect.offset(-self.velocity, 0),
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.rect);
    }
}
