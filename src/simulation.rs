use sdl2::{pixels::Color, event::Event, keyboard::Keycode};
use std::time::{Duration, Instant};
use crate::{vehicle::Vehicle, traffic_light::TrafficSystem, input::InputManager};

pub struct Simulation {
    context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    events: sdl2::EventPump,
    vehicles: Vec<Vehicle>,
    traffic_system: TrafficSystem,
    input: InputManager,
    last_spawn: Instant,
}

impl Simulation {
    pub fn new() -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem.window("Traffic Simulation", 800, 800)
            .position_centered()
            .build().unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();
        let events = context.event_pump().unwrap();

        Simulation {
            context,
            canvas,
            events,
            vehicles: Vec::new(),
            traffic_system: TrafficSystem::new(),
            input: InputManager::new(),
            last_spawn: Instant::now(),
        }
    }

    pub fn run(&mut self) {
        let mut running = true;

        while running {
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} => running = false,
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => running = false,
                    Event::KeyDown { keycode: Some(key), .. } => {
                        if self.last_spawn.elapsed() > Duration::from_millis(500) {
                            if let Some(vehicle) = self.input.handle_key(key) {
                                self.vehicles.push(vehicle);
                                self.last_spawn = Instant::now();
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.traffic_system.update();
            let other_vehicles = self.vehicles.clone();
            for vehicle in &mut self.vehicles {
                vehicle.update(&self.traffic_system, &other_vehicles);
            }

            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            self.traffic_system.render(&mut self.canvas);
            for vehicle in &self.vehicles {
                vehicle.render(&mut self.canvas);
            }
            self.canvas.present();
            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
