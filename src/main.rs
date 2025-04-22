mod simulation;
mod vehicle;
mod traffic_light;
mod road;
mod renderer;
mod input;

use simulation::Simulation;

fn main() -> Result<(), String> {
    let mut simulation = Simulation::new()?;
    simulation.run()?;
    Ok(())
}