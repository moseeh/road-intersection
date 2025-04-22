use crate::vehicle::VehicleDirection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrafficLightState {
    Red,
    Green,
}

pub struct TrafficLight {
    pub direction: VehicleDirection,
    pub state: TrafficLightState,
    pub animation_frame: u8, // For animation bonus
}

impl TrafficLight {
    pub fn new(direction: VehicleDirection, initial_state: TrafficLightState) -> Self {
        TrafficLight {
            direction,
            state: initial_state,
            animation_frame: 0,
        }
    }
    
    pub fn toggle(&mut self) {
        self.state = match self.state {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Green => TrafficLightState::Red,
        };
        self.animation_frame = 0; // Reset animation on state change
    }
    
    pub fn update_animation(&mut self) {
        // Cycle through 0-9 for animation frames
        self.animation_frame = (self.animation_frame + 1) % 10;
    }
    
    pub fn get_position(&self) -> (i32, i32) {
        match self.direction {
            VehicleDirection::North => (400 + 25, 300 - 30), // Right side of northbound lane
            VehicleDirection::South => (400 - 25, 300 + 30), // Left side of southbound lane
            VehicleDirection::East => (400 - 30, 300 - 25),  // Top side of eastbound lane
            VehicleDirection::West => (400 + 30, 300 + 25),  // Bottom side of westbound lane
        }
    }
}