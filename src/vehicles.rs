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