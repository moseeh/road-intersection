use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::road::Road;

#[allow(dead_code)]
pub struct Intersection {
    pub road_ns: Road, // vertical road
    pub road_ew: Road, // horizontal road
}

impl Intersection {
    pub fn new(road_ns: Road, road_ew: Road) -> Self {
        Intersection { road_ns, road_ew }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let crossing_color = Color::RGB(100, 100, 100);
        canvas.set_draw_color(crossing_color);
        // Intersection square
        let square = Rect::new(350, 350, 100, 100);
        let _ = canvas.fill_rect(square);
    }
}
