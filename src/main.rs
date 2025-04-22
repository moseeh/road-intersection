mod simulation;
mod vehicle;
mod traffic_light;
mod road;
mod renderer;
mod input;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
    let window = video_subsystem.window("Traffic Intersection Simulation", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    
    // Load assets
    let mut renderer = renderer::Renderer::new(&texture_creator)?;
    
    // Initialize simulation
    let mut simulation = simulation::Simulation::new();
    
    // Initialize input handler
    let mut input_handler = input::InputHandler::new();
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_spawn_time = Instant::now();
    
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    // Check for minimum time between spawns to prevent spamming
                    if last_spawn_time.elapsed() > Duration::from_millis(1000) {
                        input_handler.handle_key_press(key, &mut simulation);
                        last_spawn_time = Instant::now();
                    }
                },
                _ => {}
            }
        }
        
        // Update simulation
        simulation.update();
        
        // Render
        canvas.set_draw_color(Color::RGB(100, 100, 100)); // Gray background
        canvas.clear();
        
        renderer.render(&mut canvas, &simulation)?;
        
        canvas.present();
        
        // Cap to ~60 FPS
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}