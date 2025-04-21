use sdl2::{render::Canvas, video::Window, VideoSubsystem, Sdl};

pub struct Renderer {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}

impl Renderer {
    pub fn new(window: sdl2::video::Window) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        
        Ok(Self {
            canvas,
            sdl_context,
        })
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(0, 0, 0);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}