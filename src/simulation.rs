use sdl2::{event::Event, keyboard::Keycode};
use std::time::{Duration, Instant};
use crate::{
    vehicle::{Vehicle, Direction, Route},
    traffic_light::{TrafficLight, LightState},
    road::Road,
    input::InputHandler,
    renderer::Renderer
};

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

     /// Updates all simulation state (called each frame)
     fn update(&mut self) -> Result<(), String> {
    
        for light in &mut self.traffic_lights {
            light.update();
        }

        // Update all vehicles
        for i in 0..self.vehicles.len() {
            let can_move = self.can_vehicle_move(i);
            self.vehicles[i].update(can_move);
            if self.vehicles[i].is_off_screen() {
                self.vehicles.remove(i);
                break; 
            }
        }

        Ok(())
    }


     /// Determines if a vehicle is allowed to move based on:
    /// - Traffic light state
    /// - Distance to vehicle in front
    fn can_vehicle_move(&self, vehicle_index: usize) -> bool {
        let vehicle = &self.vehicles[vehicle_index];
        
        if let Some(light) = self.traffic_lights.iter().find(|l| l.direction == vehicle.direction) {
            if light.state == traffic_light::LightState::Red && vehicle.is_at_light() {
                return false;
            }
        }
        
        if vehicle_index > 0 {
            let front_vehicle = &self.vehicles[vehicle_index - 1];
            if vehicle.distance_to(front_vehicle) < Vehicle::SAFE_DISTANCE {
                return false;
            }
        }
        true
    }


     /// Renders all simulation components (called each frame)
     fn render(&mut self) -> Result<(), String> {
        self.render.clear();
        
        self.road.render(&mut self.render)?;
        
        for light in &self.traffic_lights {
            light.render(&mut self.render)?;
        }
        
        for vehicle in &self.vehicles {
            vehicle.render(&mut self.render)?;
        }
        
        self.render.present();
        Ok(())
    }


     /// Attempts to spawn a new vehicle from specified direction
    /// Enforces spawn cooldown and safe spacing
    pub fn spawn_vehicle(&mut self, direction: vehicle::Direction) -> Result<(), String> {
        if self.last_spawn_time.elapsed() < self.spawn_cooldown {
            return Ok(());
        }
        
        if let Some(last_vehicle) = self.vehicles.last() {
            if last_vehicle.distance_to_spawn_point(direction) < Vehicle::SAFE_DISTANCE * 2 {
                return Ok(());
            }
        }
        
        let route = vehicle::Route::random();
        let mut vehicle = Vehicle::new(direction, route);
        
        vehicle.set_spawn_position(&self.road);
        
        self.vehicles.push(vehicle);
        self.last_spawn_time = Instant::now();
        
        Ok(())
    }

}