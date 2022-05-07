use crate::creature::*;
use rand::prelude::SliceRandom;

struct Square {
    food: u8,
    prey: Vec<Creature>
}

impl Square {
    fn new() -> Self {
        Self {
            food: 50,
            prey: vec![]
        }
    }

    pub fn run(&self) {

    }

    pub fn reproduce(&mut self, mother: Creature, father: Creature) -> Creature {
        let mother_genes = mother.genes();
        let father_genes = father.genes();
        let genotype: Genotype;

        genotype = Genotype {
            fur: [mother_genes[0].clone(), father_genes[0].clone()],
            sex: [mother_genes[1].clone(), father_genes[1].clone()]
        };

        // self.prey.push(Creature::new(genotype));
        // println!("{:?}", genotype);
        Creature::new(genotype)
    }

}

pub struct Simulator {
    board: Vec<Square>
}

impl Simulator {
    pub fn new() -> Self {
        let mut board = Vec::<Square>::new();

        for _ in 0..40 {
            board.push(Square::new());
        }

        Self {
            board
        }
    }

    pub fn run(&mut self, generations: u32) {
        for _ in 0..=generations {
            for square in &self.board {
                square.run()
            }

            self.shuffe_board()
        }
    }

    fn random_square(&mut self) -> &mut Square {
        let mut random = rand::thread_rng();
        self.board.choose_mut(&mut random).unwrap()
    }

    fn shuffe_board(&mut self) {
        // get all organisms, reset all squares, and redistribute all organisms at random
        let mut prey = Vec::<Creature>::new();

        for square in &mut self.board {
            for creature in &mut square.prey {
                prey.push(*creature)
            }

            square.prey = vec![];
        }

        for creature in prey {
            self.random_square().prey.push(creature);
        }
    }
}