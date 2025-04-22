mod road;
mod vehicle;
mod traffic_light;
mod simulation;
mod input;

use simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new();
    simulation.run();
}