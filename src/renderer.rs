use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use std::path::Path;

use crate::simulation::Simulation;
use crate::traffic_light::TrafficLightState;

pub struct Renderer<'a> {
    car_texture: Option<Texture<'a>>,
    traffic_light_red: Option<Texture<'a>>,
    traffic_light_green: Option<Texture<'a>>,
}

impl<'a> Renderer<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        // Try to load textures if available
        let car_texture = Self::load_texture_if_exists(texture_creator, "assets/car.png");
        let traffic_light_red = Self::load_texture_if_exists(texture_creator, "assets/traffic_light_red.png");
        let traffic_light_green = Self::load_texture_if_exists(texture_creator, "assets/traffic_light_green.png");
        
        Ok(Renderer {
            car_texture,
            traffic_light_red,
            traffic_light_green,
        })
    }
    
    fn load_texture_if_exists(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &str
    ) -> Option<Texture<'a>> {
        if Path::new(path).exists() {
            texture_creator.load_texture(path).ok()
        } else {
            None
        }
    }
    
    pub fn render(&self, canvas: &mut Canvas<Window>, simulation: &Simulation) -> Result<(), String> {
        self.render_road(canvas)?;
        self.render_traffic_lights(canvas, simulation)?;
        self.render_vehicles(canvas, simulation)?;
        
        Ok(())
    }
    
    fn render_road(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Draw asphalt background
        let road_color = Color::RGB(50, 50, 50); // Dark gray
        
        // Horizontal road
        canvas.set_draw_color(road_color);
        canvas.fill_rect(Rect::new(0, 275, 800, 50))?;
        
        // Vertical road
        canvas.fill_rect(Rect::new(375, 0, 50, 600))?;
        
        // Draw lane dividers
        let lane_divider_color = Color::RGB(255, 255, 0); // Yellow
        canvas.set_draw_color(lane_divider_color);
        
        // Horizontal road divider
        canvas.fill_rect(Rect::new(0, 299, 375, 2))?;
        canvas.fill_rect(Rect::new(425, 299, 375, 2))?;
        
        // Vertical road divider
        canvas.fill_rect(Rect::new(399, 0, 2, 275))?;
        canvas.fill_rect(Rect::new(399, 325, 2, 275))?;
        
        // Draw sidewalks
        let sidewalk_color = Color::RGB(200, 200, 200); // Light gray
        
        // Top sidewalk
        canvas.set_draw_color(sidewalk_color);
        canvas.fill_rect(Rect::new(0, 260, 375, 15))?;
        canvas.fill_rect(Rect::new(425, 260, 375, 15))?;
        
        // Bottom sidewalk
        canvas.fill_rect(Rect::new(0, 325, 375, 15))?;
        canvas.fill_rect(Rect::new(425, 325, 375, 15))?;
        
        // Left sidewalk
        canvas.fill_rect(Rect::new(360, 0, 15, 260))?;
        canvas.fill_rect(Rect::new(360, 325, 15, 275))?;
        
        // Right sidewalk
        canvas.fill_rect(Rect::new(425, 0, 15, 260))?;
        canvas.fill_rect(Rect::new(425, 325, 15, 275))?;
        
        Ok(())
    }
    
    fn render_traffic_lights(
        &self,
        canvas: &mut Canvas<Window>,
        simulation: &Simulation
    ) -> Result<(), String> {
        for traffic_light in &simulation.traffic_lights {
            let (x, y) = traffic_light.get_position();
            
            if let (Some(red_texture), Some(green_texture)) = (&self.traffic_light_red, &self.traffic_light_green) {
                // Use texture if available
                let texture = match traffic_light.state {
                    TrafficLightState::Red => red_texture,
                    TrafficLightState::Green => green_texture,
                };
                
                let dst = Rect::new(x - 10, y - 10, 20, 20);
                canvas.copy(texture, None, dst)?;
            } else {
                // Fallback to basic rectangle rendering
                let color = match traffic_light.state {
                    TrafficLightState::Red => Color::RGB(255, 0, 0),
                    TrafficLightState::Green => Color::RGB(0, 255, 0),
                };
                
                canvas.set_draw_color(color);
                canvas.fill_rect(Rect::new(x - 10, y - 10, 20, 20))?;
                
                // Draw a black border
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.draw_rect(Rect::new(x - 10, y - 10, 20, 20))?;
            }
        }
        
        Ok(())
    }
    
    fn render_vehicles(
        &self,
        canvas: &mut Canvas<Window>,
        simulation: &Simulation
    ) -> Result<(), String> {
        for vehicle in &simulation.vehicles {
            let x = vehicle.position.x;
            let y = vehicle.position.y;
            
            if let Some(car_texture) = &self.car_texture {
                // Use texture if available
                let car_width = 30;
                let car_height = 15;
                
                let dst = Rect::new(
                    x - car_width / 2,
                    y - car_height / 2,
                    car_width as u32,
                    car_height as u32
                );
                
                // Convert angle from degrees to radians and flip it as SDL2 uses clockwise rotation
                let angle = 360.0 - vehicle.angle;
                
                // Set color tint based on route
                let (r, g, b) = vehicle.color;
                car_texture.set_color_mod(r, g, b);
                
                // Render rotated vehicle
                canvas.copy_ex(
                    car_texture,
                    None,
                    dst,
                    angle.into(), // Convert f32 to f64 with .into()
                    None,
                    false,
                    false
                )?;
            } else {
                // Fallback to drawing colored rectangles
                let (r, g, b) = vehicle.color;
                canvas.set_draw_color(Color::RGB(r, g, b));
                
                // Create a rectangle for the vehicle
                let car_width = 30;
                let car_height = 15;
                
                // Apply rotation to the rectangle
                let angle_rad = vehicle.angle * std::f32::consts::PI / 180.0;
                let cos_a = angle_rad.cos();
                let sin_a = angle_rad.sin();
                
                // Base rectangle corners (centered at origin)
                let corners = [
                    (-car_width / 2, -car_height / 2),
                    (car_width / 2, -car_height / 2),
                    (car_width / 2, car_height / 2),
                    (-car_width / 2, car_height / 2),
                ];
                
                // Rotate and translate corners
                let rotated_corners: Vec<Point> = corners.iter()
                    .map(|&(cx, cy)| {
                        let rx = (cx as f32 * cos_a - cy as f32 * sin_a) as i32;
                        let ry = (cx as f32 * sin_a + cy as f32 * cos_a) as i32;
                        Point::new(x + rx, y + ry)
                    })
                    .collect();
                
                // Draw each edge of the rectangle
                for i in 0..4 {
                    let start = rotated_corners[i];
                    let end = rotated_corners[(i + 1) % 4];
                    canvas.draw_line(start, end)?;
                }
                
                // Fill the rectangle - simple implementation
                canvas.fill_rect(Rect::new(
                    x - car_width / 2,
                    y - car_height / 2,
                    car_width as u32,
                    car_height as u32
                ))?;
            }
        }
        
        Ok(())
    }
}