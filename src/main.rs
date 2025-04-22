// src/main.rs
mod intersection;
mod road;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

use intersection::Intersection;
use road::Road;

fn main() -> Result<(), String> {
    // Initialize SDL2 context and video subsystem
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window and canvas
    let window = video_subsystem
        .window("Road Intersection", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?; // <-- map_err here

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Instantiate our modular components
    let road_ns = Road::new_vertical(350, 0, 100, 800); // North‑South road 
    let road_ew = Road::new_horizontal(0, 350, 800, 100); // East‑West road 
    let intersection = Intersection::new(road_ns, road_ew);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Input handling
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        // Drawing
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        road_ns.draw(&mut canvas);
        road_ew.draw(&mut canvas);
        intersection.draw(&mut canvas);
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
