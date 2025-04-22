use std::time::{Instant, Duration};
use rand::Rng;

use crate::vehicle::{Vehicle, VehicleDirection, VehicleRoute};
use crate::traffic_light::{TrafficLight, TrafficLightState};
use crate::road::Road;

pub struct Simulation {
    pub road: Road,
    pub vehicles: Vec<Vehicle>,
    pub traffic_lights: [TrafficLight; 4],
    light_switch_time: Instant,
    next_vehicle_id: u32,
}

impl Simulation {
    pub fn new() -> Self {
        let road = Road::new();
        let now = Instant::now();
        
        let traffic_lights = [
            TrafficLight::new(VehicleDirection::North, TrafficLightState::Red),
            TrafficLight::new(VehicleDirection::South, TrafficLightState::Red),
            TrafficLight::new(VehicleDirection::East, TrafficLightState::Green),
            TrafficLight::new(VehicleDirection::West, TrafficLightState::Green),
        ];
        
        Simulation {
            road,
            vehicles: Vec::new(),
            traffic_lights,
            light_switch_time: now,
            next_vehicle_id: 0,
        }
    }
    
    pub fn update(&mut self) {
        self.update_traffic_lights();
        self.update_vehicles();
        self.remove_out_of_bounds_vehicles();
    }
    
    fn update_traffic_lights(&mut self) {
        // Switch traffic lights every 5 seconds
        if self.light_switch_time.elapsed() > Duration::from_secs(5) {
            for light in &mut self.traffic_lights {
                light.toggle();
            }
            self.light_switch_time = Instant::now();
        }
    }
    
    fn update_vehicles(&mut self) {
        // Create a copy of vehicle IDs to avoid borrowing issues
        let vehicle_ids: Vec<u32> = self.vehicles.iter().map(|v| v.id).collect();
        
        for i in 0..vehicle_ids.len() {
            let current_id = vehicle_ids[i];
            
            // Find if there's a vehicle ahead
            let mut min_distance = f32::MAX;
            let mut vehicle_ahead = false;
            
            // Current vehicle's data
            let current_vehicle = self.vehicles.iter().find(|v| v.id == current_id).unwrap();
            let current_direction = current_vehicle.direction;
            let _current_pos = current_vehicle.position;
            let _current_passed_intersection = current_vehicle.passed_intersection;
            let _current_route = current_vehicle.route;
            
            for j in 0..vehicle_ids.len() {
                if i == j { continue; }
                
                let other_id = vehicle_ids[j];
                let other_vehicle = self.vehicles.iter().find(|v| v.id == other_id).unwrap();
                
                // Only check vehicles in the same direction before the intersection
                // or vehicles in the target direction after the intersection
                if self.vehicles_can_collide(current_vehicle, other_vehicle) {
                    let distance = current_vehicle.distance_to(other_vehicle);
                    if distance < min_distance && distance > 0.0 {
                        min_distance = distance;
                        vehicle_ahead = true;
                    }
                }
            }
            
            // Find relevant traffic light for this vehicle
            let traffic_light_state = self.traffic_lights.iter()
                .find(|tl| tl.direction == current_direction)
                .map(|tl| tl.state)
                .unwrap_or(TrafficLightState::Red);
            
            // Now update the vehicle
            if let Some(vehicle) = self.vehicles.iter_mut().find(|v| v.id == current_id) {
                let should_stop = 
                    (min_distance < 50.0 && vehicle_ahead) || 
                    (traffic_light_state == TrafficLightState::Red && !vehicle.passed_intersection && 
                     self.road.is_near_intersection(vehicle.position, vehicle.direction));
                
                vehicle.update(should_stop);
            }
        }
    }
    
    fn vehicles_can_collide(&self, v1: &Vehicle, v2: &Vehicle) -> bool {
        // Same direction, v2 ahead of v1
        if v1.direction == v2.direction && {
            let dx = v1.position.x - v2.position.x;
            let dy = v1.position.y - v2.position.y;
            (dx * dx + dy * dy) < 6400 // 80.0^2
        } {
            // If both heading in same direction, only care if they're on same segment
            if !v1.passed_intersection && !v2.passed_intersection || 
               v1.passed_intersection && v2.passed_intersection {
                return true;
            }
        }
        
        // Or both in intersection
        if self.road.is_in_intersection(v1.position) && self.road.is_in_intersection(v2.position) {
            return true;
        }
        
        false
    }
    
    fn remove_out_of_bounds_vehicles(&mut self) {
        self.vehicles.retain(|vehicle| {
            let (x, y) = (vehicle.position.x, vehicle.position.y);
            x >= -50 && x <= 850 && y >= -50 && y <= 650
        });
    }
    
    pub fn spawn_vehicle(&mut self, direction: VehicleDirection) {
        let mut rng = rand::thread_rng();
        
        // Choose a random route
        let route = match rng.gen_range(0..3) {
            0 => VehicleRoute::Left,
            1 => VehicleRoute::Straight,
            _ => VehicleRoute::Right,
        };
        
        // Generate position based on direction
        let position = match direction {
            VehicleDirection::North => sdl2::rect::Point::new(400 + 25, 600),
            VehicleDirection::South => sdl2::rect::Point::new(400 - 25, 0),
            VehicleDirection::East => sdl2::rect::Point::new(0, 300 - 25),
            VehicleDirection::West => sdl2::rect::Point::new(800, 300 + 25),
        };
        
        // Check if there's space to spawn
        let can_spawn = self.vehicles.iter().all(|v| {
            if v.direction == direction {
                match direction {
                    VehicleDirection::North | VehicleDirection::South => 
                    (v.position.y - position.y).abs() > 60,
                    VehicleDirection::East | VehicleDirection::West => 
                    (v.position.x - position.x).abs() > 60,
                }
            } else {
                true
            }
        });
        
        if can_spawn {
            let vehicle = Vehicle::new(
                self.next_vehicle_id,
                position,
                direction,
                route,
                rng.gen_range(1.5..3.0),
            );
            self.vehicles.push(vehicle);
            self.next_vehicle_id += 1;
        }
    }
    
    pub fn spawn_random_vehicle(&mut self) {
        let mut rng = rand::thread_rng();
        let direction = match rng.gen_range(0..4) {
            0 => VehicleDirection::North,
            1 => VehicleDirection::South,
            2 => VehicleDirection::East,
            _ => VehicleDirection::West,
        };
        self.spawn_vehicle(direction);
    }
}