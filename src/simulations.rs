pub struct Simulation{
    pub vehicle: Vec<Vehicle>,
    pub traffic_lights: Vec<TrafficLight>,
    pub road: Road,
    pub input_handler: InputHandler,
    pub render: Render,
    pub is_running: bool,
    pub last_spawn_time: Instant,
    pub spawn_cooldown: Duration,
}