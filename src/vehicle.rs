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

    pub fn update(&mut self, can_move: bool) {
        if can_move {
            match self.direction {
                Direction::North => self.position.1 -= self.velocity, // Move up
                Direction::South => self.position.1 += self.velocity, // Move down
                Direction::East => self.position.0 += self.velocity,  // Move right
                Direction::West => self.position.0 -= self.velocity,  // Move left
            }
        }
        self.update_bounding_box();
    }

    fn update_bounding_box(&mut self) {
        self.bounding_box.x = self.position.0;
        self.bounding_box.y = self.position.1;
    }
    pub fn is_at_light(&self) -> bool {
        match self.direction {
            Direction::North => self.position.1 <= 300,  // Near top intersection
            Direction::South => self.position.1 >= 300,  // Near bottom intersection
            Direction::East => self.position.0 >= 350,   // Near right intersection
            Direction::West => self.position.0 <= 450,   // Near left intersection
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.position.0 < -100 || self.position.0 > 900 ||
        self.position.1 < -100 || self.position.1 > 700
    }


       /// Calculates squared distance to another vehicle (optimization)
       pub fn distance_to(&self, other: &Vehicle) -> i32 {
        let dx = (self.position.0 - other.position.0).pow(2) as i32;
        let dy = (self.position.1 - other.position.1).pow(2) as i32;
        dx + dy
    }

    /// Calculates squared distance to spawn point
    pub fn distance_to_spawn_point(&self, spawn_direction: Direction) -> i32 {
        let spawn_point = match spawn_direction {
            Direction::North => (400, 600),  // Bottom center
            Direction::South => (400, 0),    // Top center
            Direction::East => (0, 300),    // Left center
            Direction::West => (800, 300),   // Right center
        };
        
        ((self.position.0 - spawn_point.0).pow(2) + 
         (self.position.1 - spawn_point.1).pow(2)) as i32
    }

    /// Renders vehicle as colored rectangle
    pub fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.bounding_box)?;
        Ok(())
    }
}