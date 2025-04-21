use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::vehicle::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub position: (i32, i32),
    pub direction: Direction,
    pub state: LightState,
    pub timer: u32,
    pub cycle_time: u32,
    pub bounding_box: Rect,
}

impl TrafficLight {
    pub const WIDTH: i32 = 20;
    pub const HEIGHT: i32 = 40;

    pub fn create_intersection_lights() -> Vec<Self> {
        vec![
            Self::new((400, 250), Direction::North, LightState::Red, 100),
            Self::new((400, 350), Direction::South, LightState::Red, 100),
            Self::new((350, 300), Direction::East, LightState::Green, 100),
            Self::new((450, 300), Direction::West, LightState::Green, 100),
        ]
    }

    pub fn new(position: (i32, i32), direction: Direction, initial_state: LightState, cycle_time: u32) -> Self {
        Self {
            position,
            direction,
            state: initial_state,
            timer: 0,
            cycle_time,
            bounding_box: Rect::new(position.0, position.1, Self::WIDTH as u32, Self::HEIGHT as u32),
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        if self.timer >= self.cycle_time {
            self.toggle();
            self.timer = 0;
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            LightState::Red => LightState::Green,
            LightState::Green => LightState::Red,
        };
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let color = match self.state {
            LightState::Red => (255, 0, 0),
            LightState::Green => (0, 255, 0),
        };
        
        canvas.set_draw_color(color);
        canvas.fill_rect(self.bounding_box)?;
        
        // Draw pole
        canvas.set_draw_color((100, 100, 100));
        let pole_rect = match self.direction {
            Direction::North => Rect::new(self.position.0 + 5, self.position.1 + Self::HEIGHT, 10, 30),
            Direction::South => Rect::new(self.position.0 + 5, self.position.1 - 30, 10, 30),
            Direction::East => Rect::new(self.position.0 + Self::WIDTH, self.position.1 + 5, 30, 10),
            Direction::West => Rect::new(self.position.0 - 30, self.position.1 + 5, 30, 10),
        };
        canvas.fill_rect(pole_rect)?;
        
        Ok(())
    }
}