use sdl2::{event::Event, keyboard::Keycode};
use std::time::{Duration, Instant};
use crate::{vehicle::Vehicle, traffic_light::TrafficLight, road::Road, input::InputHandler, renderer::Renderer};

pub struct Simulation {
    pub vehicles: Vec<Vehicle>,
    pub traffic_lights: Vec<TrafficLight>,
    pub road: Road,
    pub input_handler: InputHandler,
    pub renderer: Renderer,
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

        let renderer = Renderer::new(window)?;
        let road = Road::new();
        let traffic_lights = TrafficLight::create_intersection_lights();
        
        Ok(Self {
            vehicles: Vec::new(),
            traffic_lights,
            road,
            input_handler: InputHandler::new(),
            renderer,
            is_running: true,
            last_spawn_time: Instant::now(),
            spawn_cooldown: Duration::from_secs(1),
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.renderer.sdl_context.event_pump()?;
        
        while self.is_running {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.is_running = false;
                    },
                    _ => {}
                }
                self.input_handler.handle_event(event, self);
            }

            // Update
            self.update()?;

            // Render
            self.render()?;

            // Cap at 60 FPS
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        // Update traffic lights
        for light in &mut self.traffic_lights {
            light.update();
        }

        // Update vehicles
        for i in 0..self.vehicles.len() {
            let can_move = self.can_vehicle_move(i);
            self.vehicles[i].update(can_move);
            
            // Remove vehicles that have exited the screen
            if self.vehicles[i].is_off_screen() {
                self.vehicles.remove(i);
                break; // Exit loop to avoid indexing issues
            }
        }

        Ok(())
    }

    fn can_vehicle_move(&self, vehicle_index: usize) -> bool {
        let vehicle = &self.vehicles[vehicle_index];
        
        // Check traffic light
        if let Some(light) = self.traffic_lights.iter().find(|l| l.direction == vehicle.direction) {
            if light.state == traffic_light::LightState::Red && vehicle.is_at_light() {
                return false;
            }
        }
        
        // Check vehicle in front
        if vehicle_index > 0 {
            let front_vehicle = &self.vehicles[vehicle_index - 1];
            if vehicle.distance_to(front_vehicle) < Vehicle::SAFE_DISTANCE {
                return false;
            }
        }
        
        true
    }

    fn render(&mut self) -> Result<(), String> {
        self.renderer.clear();
        self.road.render(&mut self.renderer)?;
        
        for light in &self.traffic_lights {
            light.render(&mut self.renderer)?;
        }
        
        for vehicle in &self.vehicles {
            vehicle.render(&mut self.renderer)?;
        }
        
        self.renderer.present();
        Ok(())
    }

    pub fn spawn_vehicle(&mut self, direction: vehicle::Direction) -> Result<(), String> {
        if self.last_spawn_time.elapsed() < self.spawn_cooldown {
            return Ok(());
        }
        
        // Check if there's space to spawn a new vehicle
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