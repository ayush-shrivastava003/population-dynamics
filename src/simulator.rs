use crate::creature::*;
pub struct Simulator {
    food: u32,
    prey: Vec<Creature>
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            food: 20,
            prey: Vec::<Creature>::new()
        }
    }

    pub fn run(&mut self) -> Genotype {
        let mother = Creature::new(Genotype {
            vision: [Allele::Dominant, Allele::Recessive]
        });

        let father = Creature::new(Genotype {
            vision: [Allele::Dominant, Allele::Recessive]
        });

        self.reproduce(mother, father)
    }

    pub fn reproduce(&mut self, mother: Creature, father: Creature) -> Genotype {
        let mother_gametes = mother.genes();
        let father_gametes = father.genes();
        let genotype: Genotype;

        genotype = Genotype {
            vision: [mother_gametes, father_gametes]
        };

        // self.prey.push(Creature::new(genotype));
        // println!("{:?}", genotype);
        genotype
    }
}