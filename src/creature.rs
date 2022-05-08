use std::fmt;
use rand::{prelude::SliceRandom, distributions::{Distribution, Standard}};


#[derive(Debug, Clone, Copy)]
pub enum Allele {
    Dominant,
    Recessive
}

impl Distribution<Allele> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Allele {
        match rng.gen_range(0..=1) {
            0 => Allele::Dominant,
            _ => Allele::Recessive
        }
    }
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
pub enum ForagingAbility {
    Strong,
    Weak
}

#[derive(Debug, Clone, Copy)]
pub struct Genotype {
    pub fur: [Allele; 2],
    pub sex: [Allele; 2],
    pub foraging: [Allele; 2] // the ability for a creature to find food
}

#[derive(Debug, Clone, Copy)]
pub struct Creature {
    pub genotype: Genotype,
    pub food_eaten: u8,
    pub fur: Fur,
    pub sex: Sex,
    pub foraging: ForagingAbility,

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

        let foraging = if matches!(genotype.foraging[0], Allele::Dominant) || matches!(genotype.foraging[1], Allele::Dominant) {
            ForagingAbility::Strong
        } else {
            ForagingAbility::Weak
        };

        Self {genotype, fur, sex, foraging, food_eaten: 0}
    }

    pub fn genes(&self) -> Vec<Allele> {
        // randomly selects one allele from each gene, and then forms a half a genotype for its child
        let mut random = rand::thread_rng();
        let mut choices = Vec::<Allele>::new();
        let traits = [&self.genotype.fur, &self.genotype.sex, &self.genotype.foraging];

        for i in traits {
            let choice = i.choose(&mut random);
            choices.push(choice.unwrap().clone());
        }

        choices
    }

    pub fn reproduce(mother: &Creature, father: &Creature) -> Creature {
        let mother_genes = mother.genes();
        let father_genes = father.genes();
        let genotype: Genotype;

        genotype = Genotype {
            fur: [mother_genes[0].clone(), father_genes[0].clone()],
            sex: [mother_genes[1].clone(), father_genes[1].clone()],
            foraging: [mother_genes[2].clone(), father_genes[2].clone()]
        };

        Creature::new(genotype)
    }

}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {:?}", self.fur, self.sex)
    }
}