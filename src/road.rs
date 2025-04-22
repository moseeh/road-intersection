use sdl2::rect::Point;
use crate::vehicle::VehicleDirection;

pub struct Road {
    pub width: i32,
    pub lane_width: i32,
    pub intersection_size: i32,
}

impl Road {
    pub fn new() -> Self {
        Road {
            width: 50,
            lane_width: 25,
            intersection_size: 60,
        }
    }
    
    pub fn get_lanes(&self) -> Vec<(Point, Point)> {
        let center_x = 400;
        let center_y = 300;
        let half_width = self.width / 2;
        
        // Define the four lanes with start and end points
        vec![
            // North lane (going up)
            (
                Point::new(center_x - half_width, 600),
                Point::new(center_x - half_width, 0)
            ),
            // South lane (going down)
            (
                Point::new(center_x + half_width, 0),
                Point::new(center_x + half_width, 600)
            ),
            // East lane (going right)
            (
                Point::new(0, center_y - half_width),
                Point::new(800, center_y - half_width)
            ),
            // West lane (going left)
            (
                Point::new(800, center_y + half_width),
                Point::new(0, center_y + half_width)
            ),
        ]
    }
    
    pub fn get_intersection_rect(&self) -> (i32, i32, i32, i32) {
        let center_x = 400;
        let center_y = 300;
        let half_size = self.intersection_size / 2;
        
        (
            center_x - half_size,
            center_y - half_size,
            self.intersection_size,
            self.intersection_size
        )
    }
    
    pub fn is_in_intersection(&self, position: Point) -> bool {
        let (x, y, width, height) = self.get_intersection_rect();
        position.x >= x && position.x <= x + width && 
        position.y >= y && position.y <= y + height
    }
    
    pub fn is_near_intersection(&self, position: Point, direction: VehicleDirection) -> bool {
        let (x, y, width, height) = self.get_intersection_rect();
        let buffer = 50; // Distance before intersection to check
        
        match direction {
            VehicleDirection::North => 
                position.y <= y + height + buffer && position.y > y + height,
            VehicleDirection::South => 
                position.y >= y - buffer && position.y < y,
            VehicleDirection::East => 
                position.x >= x - buffer && position.x < x,
            VehicleDirection::West => 
                position.x <= x + width + buffer && position.x > x + width,
        }
    }
}