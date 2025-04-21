fn main()-> Result<(), String>{
let mut simulation = Simulation::new()?;
    simulation.run()?;
    Ok(())
}
