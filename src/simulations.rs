use sdl2::{event::Event, keyboard::Keycode};
use std::time::{Duration, Instant};
use crate::{vehicle::Vehicle, traffic_light::TrafficLight, road::Road, input::InputHandler, render::Render};

pub struct Simulation {
    pub vehicles: Vec<Vehicle>,
    pub traffic_lights: Vec<TrafficLight>,
    pub road: Road,
    pub input_handler: InputHandler,
    pub render: Rende,
    pub is_running: bool,
    pub last_spawn_time: Instant,
    pub spawn_cooldown: Duration,
}

impl Simulation {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        
        let window = video_subsystem.window("Traffic Intersection", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let render = Render::new(window)?;
        let road = Road::new();
        let traffic_lights = TrafficLight::create_intersection_lights();
        
        Ok(Self {
            vehicles: Vec::new(),
            traffic_lights,
            road,
            input_handler: InputHandler::new(),
            render,
            is_running: true,
            last_spawn_time: Instant::now(),
            spawn_cooldown: Duration::from_secs(1),
        })
    }

        /// Handles events, updates game state, and renders the scene
     pub fn run(&mut self) -> Result<(), String> {
        // Get SDL event pump for handling input
        let mut event_pump = self.renderer.sdl_context.event_pump()?;
        
        // Main game loop
        while self.is_running {
    
            for event in event_pump.poll_iter() {
                match event {
                    // Handle window close and ESC key for quitting
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.is_running = false;
                    },
                    _ => {}
                }
                // Handle other events like key presses
                self.input_handler.handle_event(event, self);
            }

            self.update()?;

            self.render()?;

            // Cap at ~60 FPS
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

}