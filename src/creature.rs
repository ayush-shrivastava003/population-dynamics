use std::fmt;
use rand::prelude::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub enum Allele {
    Dominant,
    Recessive
}

#[derive(Debug, Clone, Copy)]
pub enum Fur {
    Black,
    Gray,
    White
}

#[derive(Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female
}

#[derive(Debug, Clone, Copy)]
pub struct Genotype {
    pub fur: [Allele; 2],
    pub sex: [Allele; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct Creature {
    pub genotype: Genotype,
    pub fur: Fur,
    pub sex: Sex
    // remaining_life: u8
}

impl Creature {
    pub fn new(genotype: Genotype) -> Self {
        let fur = if matches!(genotype.fur[0], Allele::Dominant) {
            if matches!(genotype.fur[1], Allele::Dominant) {
                Fur::Black
            } else {
                Fur::Gray
            }
        } else if matches!(genotype.fur[1], Allele::Dominant) {
            Fur::Gray
        } else {
            Fur::White
        };

        let sex = if matches!(genotype.sex[0], Allele::Dominant) || matches!(genotype.sex[1], Allele::Dominant) {
            Sex::Male
        } else {
            Sex::Female
        };

        Self {genotype, fur, sex}
    }

    pub fn genes(&self) -> Vec<Allele> {
        // randomly selects one allele from each gene, and then forms a half a genotype for its child
        let mut random = rand::thread_rng();
        let mut choices = Vec::<Allele>::new();
        let traits = [&self.genotype.fur, &self.genotype.sex];

        for i in 0..2 {
            let choice = traits[i].choose(&mut random);
            choices.push(choice.unwrap().clone());
        }

        choices
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.fur, self.sex)
    }
}