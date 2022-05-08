extern crate population_dynamics;
use population_dynamics::*;
// use population_dynamics::creature::*;

fn main() {
    let mut sim = simulator::Simulator::new();
    sim.run(50); // 50 generations seems to be more than enough to show the carrying capacity of the population
}
