use rand::Rng;

#[derive(Debug, Clone)]
pub enum Allele {
    Dominant,
    Recessive
}

#[derive(Debug)]
pub struct Genotype {
    pub vision: [Allele; 2]
}

#[derive(Debug)]
pub struct Creature {
    genotype: Genotype,
    remaining_life: u8
}

impl Creature {
    pub fn new(genotype: Genotype) -> Self {
        Self {genotype, remaining_life: 2}
    }

    pub fn genes(&self) -> Allele {
        // randomly selects one allele from each gene, and then forms a half a genotype for its child
        let choice: usize = rand::thread_rng().gen_range(0..2);
        self.genotype.vision[choice].clone()
    }
}