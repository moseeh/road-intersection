use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy)]
pub struct Road {
    rect: Rect,
    color: Color,
    vertical: bool,
}

impl Road {
    pub fn new_vertical(x: i32, y: i32, w: u32, h: u32) -> Self {
        Road {
            rect: Rect::new(x, y, w, h),
            color: Color::RGB(50, 50, 50),
            vertical: true,
        }
    }
    pub fn new_horizontal(x: i32, y: i32, w: u32, h: u32) -> Self {
        Road {
            rect: Rect::new(x, y, w, h),
            color: Color::RGB(50, 50, 50),
            vertical: false,
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.rect);
        canvas.set_draw_color(Color::RGB(200, 200, 200));

        if self.vertical {
            let line = Rect::new(
                self.rect.x() + (self.rect.width() / 2) as i32 - 2,
                self.rect.y(),
                4,
                self.rect.height(),
            );
            let _ = canvas.fill_rect(line);
        } else {
            let line = Rect::new(
                self.rect.x(),
                self.rect.y() + (self.rect.height() / 2) as i32 - 2,
                self.rect.width(),
                4,
            );
            let _ = canvas.fill_rect(line);
        }
    }
}
