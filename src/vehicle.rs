use sdl2::rect::Point;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VehicleDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VehicleRoute {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
pub struct Vehicle {
    pub id: u32,
    pub position: Point,
    pub direction: VehicleDirection,
    pub route: VehicleRoute,
    pub speed: f32,
    pub max_speed: f32,
    pub color: (u8, u8, u8),
    pub passed_intersection: bool,
    pub angle: f32, // Rotation angle in degrees
    pub turning_state: f32, // 0.0 to 1.0 progress through a turn
}

impl Vehicle {
    pub fn new(id: u32, position: Point, direction: VehicleDirection, route: VehicleRoute, max_speed: f32) -> Self {
        // Assign color based on route
        let color = match route {
            VehicleRoute::Left => (255, 0, 0),      // Red for left turns
            VehicleRoute::Straight => (0, 255, 0),  // Green for straight
            VehicleRoute::Right => (0, 0, 255),     // Blue for right turns
        };
        
        // Calculate initial angle based on direction
        let angle = match direction {
            VehicleDirection::North => 180.0,
            VehicleDirection::South => 0.0,
            VehicleDirection::East => 270.0,
            VehicleDirection::West => 90.0,
        };
        
        Vehicle {
            id,
            position,
            direction,
            route,
            speed: max_speed,
            max_speed,
            color,
            passed_intersection: false,
            angle,
            turning_state: 0.0,
        }
    }
    
    pub fn update(&mut self, should_stop: bool) {
        if should_stop {
            // Slow down
            self.speed = self.speed * 0.9;
            if self.speed < 0.1 {
                self.speed = 0.0;
            }
        } else {
            // Speed up
            self.speed = self.speed * 1.1;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        }
        
        // Handle intersection logic
        let intersection_center = Point::new(400, 300);
        let distance_to_center = ((self.position.x - intersection_center.x).pow(2) + 
                                (self.position.y - intersection_center.y).pow(2)) as f32;
        
        // Check if vehicle is in the intersection
        let is_in_intersection = distance_to_center < 5000.0;
        
        // Start turning if in intersection and not already completed turn
        if is_in_intersection && self.turning_state < 1.0 {
            // If vehicle already started turning or has just entered the intersection
            if self.turning_state > 0.0 || self.is_at_turn_start_point() {
                self.update_position_with_turn();
                return;
            }
        }
        
        // Regular straight movement
        match self.direction {
            VehicleDirection::North => self.position = Point::new(
                self.position.x, 
                self.position.y - self.speed as i32
            ),
            VehicleDirection::South => self.position = Point::new(
                self.position.x, 
                self.position.y + self.speed as i32
            ),
            VehicleDirection::East => self.position = Point::new(
                self.position.x + self.speed as i32, 
                self.position.y
            ),
            VehicleDirection::West => self.position = Point::new(
                self.position.x - self.speed as i32, 
                self.position.y
            ),
        }
        
        // Check if we've passed the intersection
        if !self.passed_intersection && self.has_passed_intersection() {
            self.passed_intersection = true;
        }
    }
    
    fn update_position_with_turn(&mut self) {
        // Increment turning state
        self.turning_state += 0.01 * self.speed;
        if self.turning_state > 1.0 {
            self.turning_state = 1.0;
            // Update direction after turn
            self.update_direction_after_turn();
        }
        
        let center = Point::new(400, 300); // Intersection center
        
        // Calculate radius and angles based on turn type
        let (radius, start_angle, end_angle) = self.get_turn_parameters();
        
        // Interpolate between start and end angles
        let current_angle = start_angle + (end_angle - start_angle) * self.turning_state;
        
        // Calculate new position
        let new_x = center.x as f32 + radius * current_angle.cos();
        let new_y = center.y as f32 + radius * current_angle.sin();
        
        self.position = Point::new(new_x as i32, new_y as i32);
        
        // Update rotation angle (in degrees for rendering)
        self.angle = (current_angle + PI/2.0) * 180.0 / PI;
    }
    
    fn get_turn_parameters(&self) -> (f32, f32, f32) {
        // Returns (radius, start_angle, end_angle) for the turn
        let turn_radius = 50.0;
        
        match (self.direction, self.route) {
            // Left turns (90 degrees CCW)
            (VehicleDirection::North, VehicleRoute::Left) => (turn_radius, PI, PI * 3.0/2.0),
            (VehicleDirection::South, VehicleRoute::Left) => (turn_radius, 0.0, PI/2.0),
            (VehicleDirection::East, VehicleRoute::Left) => (turn_radius, PI * 3.0/2.0, 0.0),
            (VehicleDirection::West, VehicleRoute::Left) => (turn_radius, PI/2.0, PI),
            
            // Right turns (90 degrees CW)
            (VehicleDirection::North, VehicleRoute::Right) => (turn_radius, PI, PI/2.0),
            (VehicleDirection::South, VehicleRoute::Right) => (turn_radius, 0.0, PI * 3.0/2.0),
            (VehicleDirection::East, VehicleRoute::Right) => (turn_radius, PI * 3.0/2.0, PI),
            (VehicleDirection::West, VehicleRoute::Right) => (turn_radius, PI/2.0, 0.0),
            
            // Straight - no turn
            _ => (0.0, 0.0, 0.0)
        }
    }
    
    fn update_direction_after_turn(&mut self) {
        if self.route == VehicleRoute::Straight {
            return; // No direction change for straight route
        }
        
        self.direction = match (self.direction, self.route) {
            // Left turns
            (VehicleDirection::North, VehicleRoute::Left) => VehicleDirection::West,
            (VehicleDirection::South, VehicleRoute::Left) => VehicleDirection::East,
            (VehicleDirection::East, VehicleRoute::Left) => VehicleDirection::North,
            (VehicleDirection::West, VehicleRoute::Left) => VehicleDirection::South,
            
            // Right turns
            (VehicleDirection::North, VehicleRoute::Right) => VehicleDirection::East,
            (VehicleDirection::South, VehicleRoute::Right) => VehicleDirection::West,
            (VehicleDirection::East, VehicleRoute::Right) => VehicleDirection::South,
            (VehicleDirection::West, VehicleRoute::Right) => VehicleDirection::North,
            
            // Keep direction for straight
            (dir, _) => dir,
        };
    }
    
    fn is_at_turn_start_point(&self) -> bool {
        let center = Point::new(400, 300);
        
        match self.direction {
            VehicleDirection::North => {
                self.position.y > center.y - 30 && self.position.y < center.y
            },
            VehicleDirection::South => {
                self.position.y < center.y + 30 && self.position.y > center.y
            },
            VehicleDirection::East => {
                self.position.x < center.x + 30 && self.position.x > center.x
            },
            VehicleDirection::West => {
                self.position.x > center.x - 30 && self.position.x < center.x
            },
        }
    }
    
    fn has_passed_intersection(&self) -> bool {
        let center = Point::new(400, 300);
        
        match self.direction {
            VehicleDirection::North => self.position.y < center.y - 50,
            VehicleDirection::South => self.position.y > center.y + 50,
            VehicleDirection::East => self.position.x > center.x + 50,
            VehicleDirection::West => self.position.x < center.x - 50,
        }
    }
    
    pub fn distance_to(&self, other: &Vehicle) -> f32 {
        let dx = self.position.x - other.position.x;
        let dy = self.position.y - other.position.y;
        (dx * dx + dy * dy) as f32
    }
}