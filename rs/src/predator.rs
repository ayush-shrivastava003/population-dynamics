use crate::prey::{Allele, Sex};
use rand::{prelude::SliceRandom};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Vision {
    NearSighted,
    FarSighted,
}

#[derive(Debug, Clone, Copy)]
pub struct PredGenotype {
    pub sex: [Allele; 2],
    pub vision: [Allele; 2]
}

#[derive(Debug, Clone, Copy)]
pub struct Predator {
    pub genotype: PredGenotype,
    pub food_eaten: u8,
    pub sex: Sex,
    pub vision: Vision,
    pub dies_in: u8
}

impl Predator {
    pub fn new(genotype: PredGenotype) -> Self { // TODO: make function to determine phenotype
        let sex = if matches!(genotype.sex[0], Allele::Dominant) || matches!(genotype.sex[1], Allele::Dominant) {
            Sex::Male
        } else {
            Sex::Female
        };

        let vision = if matches!(genotype.vision[0], Allele::Dominant) || matches!(genotype.vision[1], Allele::Dominant) {
            Vision::NearSighted
        } else {
            Vision::FarSighted
        };

        Self {
            genotype,
            food_eaten: 0,
            dies_in: 2,
            sex,
            vision
        }
    }

    pub fn genes(&self) -> Vec<Allele> {
        // TODO: make a creature trait for common functions like genes and reproduce
        let mut random = rand::thread_rng();
        let mut choices = Vec::<Allele>::new();
        let traits = [&self.genotype.sex, &self.genotype.vision];

        for i in traits {
            let choice = i.choose(&mut random);
            choices.push(choice.unwrap().clone());
        }

        choices
    }

    pub fn reproduce(mother: &Predator, father: &Predator) -> Predator {
        let mother_genes = mother.genes();
        let father_genes = father.genes();
        let genotype = PredGenotype {
            sex: [mother_genes[0].clone(), father_genes[0].clone()],
            vision: [mother_genes[1].clone(), father_genes[1].clone()],
        };

        let child = Predator::new(genotype);
        // println!("{} and {} produced {}", mother, father, child);
        child
    }

}

impl fmt::Display for Predator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.sex, self.vision)
    }
}