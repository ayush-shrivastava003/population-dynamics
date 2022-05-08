extern crate population_dynamics;
use population_dynamics::*;
// use population_dynamics::creature::*;

fn main() {
    let mut sim = simulator::Simulator::new();
    sim.run(10000);
}
