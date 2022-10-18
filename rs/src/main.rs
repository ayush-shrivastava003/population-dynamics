extern crate population_dynamics;
use population_dynamics::*;
// use population_dynamics::creature::*;

fn main() {
    let mut sim = simulator::Simulator::new();
    println!("{:?}", sim.board);
    sim.run(100); // 50 generations seems to be more than enough to show the carrying capacity of the population
}
