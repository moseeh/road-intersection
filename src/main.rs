// src/main.rs
mod intersection;
mod road;
mod traffic_light;
mod vehicle;

use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;

use intersection::Intersection;
use traffic_light::TrafficLight;
use vehicle::{Direction, Vehicle};

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

    // Just outside the vertical road’s right edge (north-bound traffic)
    let mut light_n = TrafficLight::new(460, 460, 20, 20);
    // Just outside the vertical road’s left edge (south-bound traffic)
    let mut light_s = TrafficLight::new(320, 460, 20, 20);
    // Just outside the horizontal road’s bottom edge (east-bound traffic)
    let mut light_e = TrafficLight::new(320, 320, 20, 20);
    // Just outside the horizontal road’s top edge (west-bound traffic)
    let mut light_w = TrafficLight::new(460, 320, 20, 20);
    // Vehicle container and initial spawns
    let mut vehicles = vec![
        Vehicle::new(Direction::North),
        Vehicle::new(Direction::South),
        Vehicle::new(Direction::East),
        Vehicle::new(Direction::West),
    ];

    let mut event_pump = sdl_context.event_pump()?;
    let mut n = 0;
    'running: loop {
        // Input handling
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }
        n += 1;

        if n > 100 {
            // Update lights
            light_n.update();
            light_s.update();
            light_e.update();
            light_w.update();
            n = 0;
        }
        for vehicle in vehicles.iter_mut() {
            vehicle.update();
        }

        // Drawing
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        road_ns.draw(&mut canvas);
        road_ew.draw(&mut canvas);
        intersection.draw(&mut canvas);
        // Draw lights on top
        light_n.draw(&mut canvas);
        light_s.draw(&mut canvas);
        light_e.draw(&mut canvas);
        light_w.draw(&mut canvas);
        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
