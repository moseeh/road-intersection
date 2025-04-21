use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
pub enum Route {
    Left,
    Right,
    Straight,
}

impl Route {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Route::Left,
            1 => Route::Right,
            _ => Route::Straight,
        }
    }
}


/// Vehicle entity with movement and rendering logic
pub struct Vehicle {
    pub position: (i32, i32),
    pub direction: Direction,
    pub route: Route,
    pub velocity: i32,
    pub color: (u8, u8, u8),
    pub bounding_box: Rect,
}

impl Vehicle {
    pub const SAFE_DISTANCE: i32 = 50;
    pub const WIDTH: i32 = 30;
    pub const HEIGHT: i32 = 20;

    pub fn new(direction: Direction, route: Route) -> Self {
        Self {
            position: (0, 0), 
            direction,
            route,
            velocity: 2, 
            color: match route {
                Route::Left => (255, 0, 0),   
                Route::Right => (0, 255, 0),   
                Route::Straight => (0, 0, 255), 
            },
            bounding_box: Rect::new(0, 0, Self::WIDTH, Self::HEIGHT),
        }
    }

        pub fn set_spawn_position(&mut self, road: &crate::road::Road) {
            self.position = match self.direction {
e
                Direction::North => (road.intersection_center.0, road.bounds.height() as i32),
               
                Direction::South => (road.intersection_center.0, -Self::HEIGHT),
              
                Direction::East => (-Self::WIDTH, road.intersection_center.1),
      
                Direction::West => (road.bounds.width() as i32, road.intersection_center.1),
            };
            self.update_bounding_box();
        }
}