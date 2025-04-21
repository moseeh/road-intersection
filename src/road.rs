use sdl2::rect::Rect;

pub struct Road {
    pub bounds: Rect,
    pub intersection_center: (i32, i32),
    pub intersection_size: (i32, i32),
}