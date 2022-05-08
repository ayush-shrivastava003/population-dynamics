use crate::creature::*;
use rand::{prelude::SliceRandom, Rng};

#[derive(Debug)]
pub struct Square {
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

    pub fn get_mates(&self) -> Vec<&Creature> {
        let mut mates = vec![];

        for creature in &self.prey {
            if matches!(creature.sex, Sex::Female) && creature.food_eaten == 2 {
                mates.push(creature)
            }
        }

        mates
    }

    pub fn run(&mut self) {
        let mut randomizer = rand::thread_rng();

        for mut creature in &mut self.prey {
            if self.food == 0 {
                break
            }

            let food_chance: u8 = randomizer.gen_range(1..=10);

            if matches!(creature.foraging, ForagingAbility::Strong) { // 4/10 chance of survival, 4/10 chance of reproduction
                if 2 < food_chance && food_chance <= 6 && self.food >= 1 {
                    creature.food_eaten = 1;
                    self.food -= 1
                } else if food_chance > 6 && self.food >= 1 {
                    if self.food == 1 {
                        creature.food_eaten = 1;
                        self.food -= 1
                    } else {
                        creature.food_eaten = 2;
                        self.food -= 1
                    }
                } else {
                    creature.food_eaten = 0;
                    self.food -= 1
                }
            } else { // 3/10 chance of surival, 2/10 chance of reproduction
                if 5 < food_chance && food_chance <= 8 && self.food >= 1 {
                    creature.food_eaten = 1;
                    self.food -= 1
                } else if food_chance > 8 && self.food >= 1 {
                    if self.food == 1 {
                        creature.food_eaten = 1;
                        self.food -= 1
                    } else {
                        creature.food_eaten = 2;
                        self.food -= 2
                    }
                } else {
                    creature.food_eaten = 0
                }
            }
        }

        let mut children = vec![];
        for creature in &self.prey {
            if matches!(creature.sex, Sex::Male) && creature.food_eaten == 2 {
                let mates = self.get_mates();
                if mates.len() > 0 {
                    let mate = mates.choose(&mut randomizer).unwrap();
                    children.push(Creature::reproduce(mate, creature));
                }

            }
        }
        self.prey.extend_from_slice(&children);
    }
}

pub struct Simulator {
    pub board: Vec<Square>
}

impl Simulator {
    pub fn new() -> Self {
        let mut board = Vec::<Square>::new();

        for _ in 0..40 {
            let mut square = Square::new();
            square.prey.push(Creature::new(
                Genotype {
                    fur: [rand::random(), rand::random()],
                    sex: [rand::random(), rand::random()],
                    foraging: [rand::random(), rand::random()]
            }));

            board.push(square);
        }

        Self {board}
    }

    fn stats(&mut self) {
        println!("=======\nRESULTS\n=======\n");
        let mut males = 0;
        let mut females = 0;
        let mut black = 0;
        let mut white = 0;
        let mut gray = 0;
        let mut strong = 0;
        let mut weak = 0;

        let all = self.collect_prey();
        let all_num = all.len() as f32;

        for creature in all {
            match creature.sex {
                Sex::Male => males += 1,
                _ => females += 1
            }

            match creature.fur {
                Fur::Black => black += 1,
                Fur::Gray => gray += 1,
                _ => white += 1
            }

            match creature.foraging {
                ForagingAbility::Strong => strong += 1,
                ForagingAbility::Weak => weak += 1
            }
        }

        println!("Fur Color:");
        println!(
            "{} ({}%) black : {} ({}%) gray : {} ({}%) white",
            black,
            (black as f32 / all_num) * 100.0,
            gray,
            (gray as f32 / all_num) * 100.0,
            white,
            (white as f32 / all_num) * 100.0
        );
        
        println!("\nSex:");
        println!(
            "{} ({}%) male : {} ({}%) female",
            males,
            (males as f32 / all_num) * 100.0,
            females,
            (females as f32 / all_num) * 100.0
        );

        println!("\nForaging Ability:");
        println!(
            "{} ({}%) strong : {} ({}%) weak",
            strong,
            (strong as f32 / all_num) * 100.0,
            weak,
            (weak as f32 / all_num) * 100.0
        );
    }

    #[allow(unused_variables)]
    pub fn run(&mut self, generations: u32) {
        self.stats();
        for gen in 0..=generations {
            println!("generation {}", gen);

            for square in &mut self.board {
                square.run()
            }

            // print!("\r");
            // print!("{}[2KStatus: shuffling board...", 27 as char);
            // println!("Population: {:?}", self.shuffe_board());
            self.shuffle_board();
            // println!("Completed generation {} in {} seconds", gen, start.elapsed().as_secs_f32());
        }
        println!();
        self.stats();
    }

    fn random_square(&mut self) -> &mut Square {
        let mut random = rand::thread_rng();
        self.board.choose_mut(&mut random).unwrap()
    }

    fn collect_prey(&mut self) -> Vec<Creature> {
        let mut prey = Vec::<Creature>::new();
        
        for square in &mut self.board {
            for creature in &square.prey {
                if creature.food_eaten > 0 {
                    prey.push(*creature);
                }
            }
        }

        prey
    }

    fn shuffle_board(&mut self) -> usize {
        // get all organisms, reset all squares, and redistribute all organisms at random
        let prey = self.collect_prey();
        let prey_len = prey.len();
        
        for mut creature in prey {
            creature.food_eaten = 0;
            self.random_square().prey.push(creature);
        }

        prey_len
    }
}