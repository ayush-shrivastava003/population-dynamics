use crate::{prey::*, predator::*};
use rand::{prelude::SliceRandom, Rng, thread_rng, rngs::ThreadRng};
use std::{time::Instant, io::{stdout, Write}};

#[derive(Debug)]
pub struct Square {
    food: u8,
    prey: Vec<Prey>,
    predators: Vec<Predator>
}

impl Square {
    fn new() -> Self {
        Self {
            food: 250,
            prey: vec![],
            predators: vec![]
        }
    }

    pub fn get_prey_mates(&self) -> Vec<&Prey> {
        let mut mates = vec![];

        for creature in &self.prey {
            if matches!(creature.sex, Sex::Female) && creature.food_eaten == 2 {
                mates.push(creature)
            }
        }

        mates
    }

    pub fn get_predator_mates(&self) -> Vec<&Predator> {
        let mut mates = vec![];

        for creature in &self.predators {
            if matches!(creature.sex, Sex::Female) && creature.food_eaten == 2 {
                mates.push(creature);
            }
        }

        mates
    }

    pub fn choose_prey(&mut self, randomizer: &mut ThreadRng) -> &mut Prey {
        let mut option_prey = self.prey.choose_mut(randomizer);

        while matches!(&option_prey, &None) || option_prey.as_ref().unwrap().dies_in > 0 {
            option_prey = self.prey.choose_mut(randomizer);
        }

        option_prey.unwrap()
    }

    pub fn reproduce(&mut self, randomizer: &mut ThreadRng) {
        let mut children = vec![];
        for creature in &self.prey {
            if matches!(creature.sex, Sex::Male) && creature.food_eaten == 2 {
                let mates = self.get_prey_mates();
                if mates.len() > 0 {
                    let mate = mates.choose(randomizer).unwrap();
                    let mate_chance: u8 = randomizer.gen_range(1..=10);
                    // println!("Mate chance: {}", mate_chance);
                    match creature.fur {
                        Fur::Black => if mate_chance <= 8 {
                            children.push(Prey::reproduce(mate, creature));
                        },
                        Fur::Gray => if mate_chance <= 5 {
                            children.push(Prey::reproduce(mate, creature));
                        }
                        _ => if mate_chance == 1 {
                            children.push(Prey::reproduce(mate, creature));
                        }
                    }
                }

            }
        }
        self.prey.extend_from_slice(&children);

        let mut children = vec![];
        let mut reproduced = 0;
        for creature in &self.predators {
            if matches!(creature.sex, Sex::Male) && creature.food_eaten == 2 {
                let mates = self.get_predator_mates();
                if mates.len() > 0 {
                    let mate = mates.choose(randomizer).unwrap();
                    let mate_chance: u8 = randomizer.gen_range(1..=10);

                    if mate_chance < 3 {
                        children.push(Predator::reproduce(mate, creature));
                        reproduced += 1;
                    }
                }

            }
        }

        self.predators.extend_from_slice(&children);
    }

    pub fn feed(&mut self, randomizer: &mut ThreadRng) {
        for mut creature in &mut self.prey {
            if self.food == 0 { break; }

            let food_chance: u8 = randomizer.gen_range(1..=10);

            // if matches!(creature.foraging, ForagingAbility::Strong) { // 4/10 chance of survival, 4/10 chance of reproduction
            //     if 2 < food_chance && food_chance <= 6 && self.food >= 1 {
            //         creature.food_eaten = 1;
            //         self.food -= 1
            //     } else if food_chance > 6 && self.food >= 1 {
            //         if self.food == 1 {
            //             creature.food_eaten = 1;
            //             self.food -= 1
            //         } else {
            //             creature.food_eaten = 2;
            //             self.food -= 1
            //         }
            //     } else {
            //         creature.food_eaten = 0;
            //         self.food -= 1
            //     }
            // } else { // 3/10 chance of surival, 2/10 chance of reproduction
            //     if 5 < food_chance && food_chance <= 8 && self.food >= 1 {
            //         creature.food_eaten = 1;
            //         self.food -= 1
            //     } else if food_chance > 8 && self.food >= 1 {
            //         if self.food == 1 {
            //             creature.food_eaten = 1;
            //             self.food -= 1
            //         } else {
            //             creature.food_eaten = 2;
            //             self.food -= 2
            //         }
            //     } else {
            //         creature.food_eaten = 0
            //     }
            // }

            // 4/10 chance of survival, 4/10 chance of reproduction, 2/10 death
            if 2 < food_chance && food_chance <= 5 && self.food >= 1 {
                creature.food_eaten = 1;
                self.food -= 1
            } else if food_chance > 5 && self.food >= 1 {
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
        }

        for creature in &mut self.predators {
            let food_chance: u8 = randomizer.gen_range(1..=10);
            let prey = self.choose_prey(randomizer);

            let prey = self.prey.choose_mut(randomizer).unwrap();

            if 7 < food_chance && food_chance <= 9 {
                prey.food_eaten = 0; // prevent it from finding mates
                prey.dies_in = 0; // kill it in the prey collection
                creature.food_eaten = 1;
            } else if food_chance > 9 {
                prey.food_eaten = 0; // prevent it from finding mates
                prey.dies_in = 0; // kill it in the prey collection

                let prey2 = self.prey.choose_mut(randomizer).unwrap();
                prey2.food_eaten = 0;
                prey2.dies_in = 0;
                creature.food_eaten = 2;
            } else {
                creature.food_eaten = 0;
            }

            // creature.food_eaten

        }
    }

    pub fn run(&mut self) {
        let mut randomizer = thread_rng();

        self.feed(&mut randomizer);
        self.reproduce(&mut randomizer);

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
            square.prey.push(Prey::new(
                PreyGenotype {
                    fur: [Allele::Dominant, Allele::Dominant],
                    sex: [Allele::Dominant, Allele::Recessive],
                    foraging: [Allele::Dominant, Allele::Recessive]
                    // fur: [rand::random(), rand::random()],
                    // sex: [rand::random(), rand::random()],
                    // foraging: [rand::random(), rand::random()]
            }));

            // square.prey.push(Prey::new(
            //     PreyGenotype {
            //         fur: [Allele::Recessive, Allele::Recessive],
            //         sex: [Allele::Recessive, Allele::Recessive],
            //         foraging: [Allele::Dominant, Allele::Recessive]
            //         // fur: [rand::random(), rand::random()],
            //         // sex: [rand::random(), rand::random()],
            //         // foraging: [rand::random(), rand::random()]
            // }));

            square.predators.push(Predator::new(
                PredGenotype {
                    sex: [Allele::Dominant, Allele::Recessive],
                    vision: [Allele::Dominant, Allele::Recessive]
                    // sex: [rand::random(), rand::random()],
                    // vision: [rand::random(), rand::random()]
            }));

            square.predators.push(Predator::new(
                PredGenotype {
                    sex: [Allele::Recessive, Allele::Recessive],
                    vision: [Allele::Dominant, Allele::Recessive]
                    // sex: [rand::random(), rand::random()],
                    // vision: [rand::random(), rand::random()]
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

        let mut strong_male = 0;
        let mut weak_male = 0;
        let mut strong_female = 0;
        let mut weak_female = 0;

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
                ForagingAbility::Strong => {
                    strong += 1;
                    match creature.sex {
                        Sex::Male => strong_male += 1,
                        _ => strong_female += 1
                    }
                },
                ForagingAbility::Weak => {
                    weak += 1;
                    match creature.sex {
                        Sex::Male => weak_male += 1,
                        _ => weak_female += 1
                    }
                }
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

        println!("\nDihybrid Foraging x Sex:"); 
        /*
        Testing if Foraging vs. Sex results in a 9:3:3:1 ratio, but it clearly looks like it doesn't with the current settings:
            367 (55.521935%) strong male : 40 (6.0514374%) weak male : 247 (37.367622%) strong female : 7 (1.0590014%) weak female
        It's defnitely to a significant bias for a strong foraging ability (obv - the probabilities for survival/reproduction are way worse if ur foraging sucks)
        Additionally, sex is not really something that matters right now. It's defnitely possible that the numbers will get even more skewed once
        females require more food during pregnancy.

        After making the foraging ability redundant (both variants had the same chances of survival/reproduction), the dihybrid comes a lot closer
        to the expected ratio, which is really exciting.
            393 (59.00901%) strong male : 83 (12.462462%) weak male : 153 (22.972973%) strong female : 37 (5.555556%) weak female
        I'm too lazy to do a chi-square test to see if the numbers match, but they generally seem to be close enough to the expected percentages.
        */
        println!(
            "{} ({}%) strong male : {} ({}%) weak male : {} ({}%) strong female : {} ({}%) weak female",
            strong_male,
            (strong_male as f32 / all_num) * 100.0,
            weak_male,
            (weak_male as f32 / all_num) * 100.0,
            strong_female,
            (strong_female as f32 / all_num) * 100.0,
            weak_female,
            (weak_female as f32 / all_num) * 100.0,
        )
    }

    pub fn run(&mut self, generations: usize) {
        // self.stats();
        let start = Instant::now();
        let scale = 100.0 / generations as f32;

        for gen in 0..=generations {
            for square in &mut self.board {
                square.run();
            }

            let gen_scale = (gen as f32 * scale) as usize;
            print!(
                "\r\x1b[2KGen. {}/{} - {} [{}{}]",
                gen,
                generations,
                self.shuffle_board(),
                "#".repeat(gen_scale),
                " ".repeat(100-gen_scale)
            );
            stdout().flush().unwrap();
        }
        println!();
        self.stats();
        println!("\nFinished in {} seconds", start.elapsed().as_secs_f32());
    }

    fn random_square(&mut self) -> &mut Square {
        let mut random = rand::thread_rng();
        self.board.choose_mut(&mut random).unwrap()
    }

    fn collect_prey(&mut self) -> Vec<Prey> {
        let mut prey = Vec::<Prey>::new();
        
        for square in &mut self.board {
            for creature in &square.prey {
                if creature.food_eaten > 0 && creature.dies_in > 0 {
                    prey.push(*creature);
                }
            }
        }

        prey
    }

    fn collect_predators(&mut self) -> Vec<Predator> {
        let mut predators = Vec::<Predator>::new();
        for square in &mut self.board {
            let mut dead_counter = 0;
            let total = square.predators.len();

            for creature in &square.predators {
                if creature.food_eaten > 0 && creature.dies_in > 0 {
                    predators.push(*creature);
                } else {
                    dead_counter += 1;
                }
            }

            // println!(
            //     "{}% ({} / {}) of the predator population in the square died. {}", 
            //     ((dead_counter as f32) / (total as f32)) * 100.0,
            //     dead_counter,
            //     total,
            //     predators.len()
            // )
        }

        predators
    }

    fn shuffle_board(&mut self) -> String {
        // get all organisms, reset all squares, and redistribute all organisms at random
        let prey = self.collect_prey();
        let predators = self.collect_predators();
        let stats = format!("{} prey : {} predators", prey.len(), predators.len());
        
        for mut creature in prey {
            creature.food_eaten = 0;
            creature.dies_in -= 1;

            let mut square = self.random_square();
            square.food = 250;
            square.prey.push(creature);
        }

        for mut creature in predators {
            creature.food_eaten = 0;
            creature.dies_in -= 1;

            let square = self.random_square();
            square.predators.push(creature);

        }

        stats

    }

}