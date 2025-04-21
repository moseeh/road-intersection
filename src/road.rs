use sdl2::rect::Rect;

pub struct Road {
    pub bounds: Rect,
    pub intersection_center: (i32, i32),
    pub intersection_size: (i32, i32),
}

impl Road {
    pub fn new() -> Self {
        Self {
            bounds: Rect::new(0, 0, 800, 600),
            intersection_center: (400, 300),
            intersection_size: (100, 100),
        }
    }
}