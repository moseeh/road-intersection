use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Turn {
    Straight,
    Right,
    Left,
}

pub struct Vehicle {
    rect: Rect,
    direction: Direction,
    velocity: i32,
    color: Color,
    turn: Turn,
}

impl Vehicle {
    pub fn new(direction: Direction) -> Self {
        let velocity = 2;
        let rect = match direction {
            Direction::North => Rect::new(415, 800, 20, 40),
            Direction::South => Rect::new(365, -40, 20, 40),
            Direction::East => Rect::new(-40, 415, 40, 20),
            Direction::West => Rect::new(800, 365, 40, 20),
        };
        let mut rng = rand::rng();
        let (turn, color)  = match  rng.random_range(0..3) {
            0 => (Turn::Straight, Color::CYAN), 
            1 => (Turn::Right,Color::YELLOW),
            2 => (Turn::Left, Color::BLUE),
            _ => unreachable!()
        };
        Vehicle {
            rect,
            direction,
            velocity,
            color,
            turn,
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
    // Get the vehicle's direction
    pub fn direction(&self) -> Direction {
        self.direction
    }

    // Get a reference to the vehicle's rectangle
    pub fn rect(&self) -> Rect {
        self.rect
    }

    // Check if the vehicle is at a safe distance from its spawn point
    // This helps prevent immediately spawning vehicles on top of each other
    pub fn is_safe_distance(&self) -> bool {
        match self.direction {
            Direction::North => self.rect.y() < 750, // Vehicle has moved away from spawn point
            Direction::South => self.rect.y() > 10,
            Direction::East => self.rect.x() > 10,
            Direction::West => self.rect.x() < 750,
        }
    }
}
