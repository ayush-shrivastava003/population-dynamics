extern crate population_dynamics;
use population_dynamics::*;
use population_dynamics::creature::*;

fn main() {
    let mut children = Vec::<Genotype>::new();
    let mut sim = simulator::Simulator::new();
    let mut heterozygous = 0; 
    let mut homo_dominant = 0;
    let mut homo_recessive = 0;

    for gen in 1..=100 {
        println!("Generation {}", gen);
        children.push(sim.run());
    };

    for child in children {
        if matches!(child.vision[0], Allele::Dominant) {
            if matches!(child.vision[1], Allele::Recessive) {
                heterozygous += 1;
            } else {
                homo_dominant += 1;
            }
        } else {
            if matches!(child.vision[1], Allele::Recessive) {
                homo_recessive += 1;
            } else {
                heterozygous += 1;
            }
        }
    }

    println!("\n{} hetero : {} homo dominant : {} homo recessive", heterozygous, homo_dominant, homo_recessive);
}
