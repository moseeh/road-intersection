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

      /// Renders the road network to the screen
    /// Draws in three layers:
    /// 1. Dark gray background
    /// 2. Light gray roads
    /// 3. White intersection area
    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {

        canvas.set_draw_color(50, 50, 50);
        canvas.fill_rect(self.bounds)?;

        canvas.set_draw_color(100, 100, 100);
    
        canvas.fill_rect(Rect::new(
            0,                                   
            self.intersection_center.1 - 25,     
            self.bounds.width(),               
            50                                  
        ))?;
        
        canvas.fill_rect(Rect::new(
            self.intersection_center.0 - 25,     
            0,                                   
            50,                                   
            self.bounds.height()               
        ))?;

        canvas.set_draw_color(255, 255, 255);
        canvas.fill_rect(Rect::new(
            self.intersection_center.0 - self.intersection_size.0 / 2,  
            self.intersection_center.1 - self.intersection_size.1 / 2,  
            self.intersection_size.0 as u32,                            
            self.intersection_size.1 as u32                           
        ))?;

        Ok(())
    }
}