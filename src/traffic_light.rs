use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightState {
    Red,
    Green,
}
#[allow(dead_code)]
pub struct TrafficLight {
    rect: Rect,
    state: LightState,
}

impl TrafficLight {
    pub fn new(x: i32, y: i32, w: u32, h: u32, state: LightState) -> Self {
        TrafficLight {
            rect: Rect::new(x, y, w, h),
            state,
        }
    }
    pub fn update(&mut self, is_green: bool) {
        self.state = if is_green {
            LightState::Green
        } else {
            LightState::Red
        };
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let color = match self.state {
            LightState::Red => Color::RGB(200, 0, 0),
            LightState::Green => Color::RGB(0, 200, 0),
        };

        canvas.set_draw_color(color);
        let _ = canvas.fill_rect(self.rect);
    }
    pub fn state(&self) -> LightState {
        self.state
    }
}
