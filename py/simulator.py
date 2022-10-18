from creatures import *
import random
import time
from sys import stdout

class Square():
    def __init__(self, id):
        self.food = 250
        self.prey = []
        self.predators = []
        self.id = id

    ## Utility methods ##
    def choose_prey(self):
        if len(self.prey) == 0:
            return None
        
        edible_prey = []
        for creature in self.prey:
            if creature.dies_in > 0:
                edible_prey.append(creature)

        if len(edible_prey) == 0:
            return None

        choice = random.choice(edible_prey)
        return choice

    def get_mates(self, is_prey: bool):
        mates = []
        species = self.prey if is_prey else self.predators
        for creature in species:
            if creature.sex == Sex.Female and creature.food_eaten == 2:
                mates.append(creature)
        
        return mates

    ## Core methods ##
    def reproduce(self):
        children = []
        for creature in self.prey:
            if creature.sex == Sex.Male and creature.food_eaten == 2:
                mates = self.get_mates(True)
                if len(mates) > 0:
                    mate = random.choice(mates)
                    mate_chance = random.randint(1, 10)

                    if creature.fur == Fur.Black and mate_chance <= 8:
                        children.append(Prey.reproduce(mate, creature))
                    elif creature.fur == Fur.Gray and mate_chance <= 5:
                        children.append(Prey.reproduce(mate, creature))
                    elif creature.fur == Fur.White and mate_chance == 1:
                        children.append(Prey.reproduce(mate, creature))
        self.prey.extend(children)

        children = []
        for creature in self.predators:
            if creature.sex == Sex.Male and creature.food_eaten == 2:
                mates = self.get_mates(False)
                if len(mates) > 0:
                    mate = random.choice(mates)
                    mate_chance = random.randint(1, 10)

                    if mate_chance > 1:
                        children.append(Predator.reproduce(mate, creature))

        self.predators.extend(children)

    def feed(self):
        for creature in self.prey:
            if self.food == 0: break
            
            food_chance = random.randint(1, 10) # endpoints inclusive

            if (2 < food_chance and food_chance <= 6) and self.food >= 1:
                creature.food_eaten = 1
                self.food -= 1
            elif food_chance > 6 and self.food >= 1:
                if self.food == 1:
                    creature.food_eaten = 1
                    self.food -= 1
                else:
                    creature.food_eaten = 2
                    self.food -= 2
            else:
                creature.food_eaten = 0

        for creature in self.predators:
            food_chance = random.randint(1, 10)
            prey = self.choose_prey()

            if prey == None:
                break

            if 2 < food_chance and food_chance <= 6:
                prey.food_eaten = 0
                prey.dies_in = 0
                creature.food_eaten = 1
            elif food_chance > 6:
                prey.food_eaten = 0
                prey.dies_in = 0
                creature.food_eaten = 1

                prey2 = self.choose_prey()
                
                if prey2 != None:
                    prey2.food_eaten = 0
                    prey2.dies_in = 0
                    creature.food_eaten = 2
            else:
                creature.food_eaten = 0


    def run(self):
        self.feed()
        self.reproduce()

class Simulator():
    def __init__(self):
        self.board = []
        self.pops = []

        for i in range(40):
            square = Square(i)

            for _ in range(1000):
                square.prey.append(Prey(
                    PreyGenotype(
                        fur=[Allele.Dominant, Allele.Dominant],
                        sex=[Allele.Dominant, Allele.Recessive]
                )))
                square.prey.append(Prey(
                    PreyGenotype(
                        fur=[Allele.Recessive, Allele.Recessive],
                        sex=[Allele.Recessive, Allele.Recessive]
                )))

            for _ in range(100):
                square.predators.append(Predator(
                    PredatorGenotype(
                        vision=[Allele.Dominant, Allele.Dominant],
                        sex=[Allele.Dominant, Allele.Recessive]
                )))

                square.predators.append(Predator(
                    PredatorGenotype(
                        vision=[Allele.Recessive, Allele.Recessive],
                        sex=[Allele.Recessive, Allele.Recessive]
                )))
        
            self.board.append(square)

        print("Board is set up.")

    def run(self, generations: int):
        start = time.perf_counter()
        scale = 100 / generations

        print("Beginning simulation!")
        for gen in range(generations+1):
            for square in self.board:
                square.run()
        
            stats = self.shuffle_board()
            self.pops.append(stats)

            gen_scale = int(gen * scale)
            stdout.write(
                "\r\x1b[2KGen. {}/{} - {} [{}{}]".format(
                gen,
                generations,
                stats,
                "#" * gen_scale,
                " " * (100 - gen_scale)
            ))
            stdout.flush()
        
        print("\nFinished in {} seconds", time.perf_counter() - start)
        print(self.pops)
    
    def random_square(self):
        return random.choice(self.board)

    def collect_creatures(self):
        prey = []
        predators = []
        total_pred = 0
        total_prey = 0

        for square in self.board:
            dead_pred = 0
            dead_prey = 0
            for creature in square.prey:
                if creature.dies_in > 0:
                    prey.append(creature)
                else:
                    dead_prey += 1
            for creature in square.predators:
                if creature.dies_in > 0:
                    predators.append(creature)
                else:
                    dead_pred += 1

            if len(square.prey) > 0:
                print(f"{(dead_prey / len(square.prey)) * 100} ({dead_prey} / {len(square.prey)}) prey died")
            else:
                # print("No prey in the square.")
                pass
            
            if len(square.predators) > 0:
                print(f"{(dead_pred / len(square.predators)) * 100} ({dead_pred} / {len(square.predators)}) predators died")
            else:
                # print("No predators in the square.")
                pass
            
            square.prey.clear()
            square.predators.clear()

        # mortality_prey = round((dead_prey / total_prey) * 100)
        # mortality_pred = round((dead_pred / total_pred) * 100)

        return prey, predators #mortality_prey, mortality_pred

    def shuffle_board(self):
        prey, predators = self.collect_creatures()
        stats = (len(prey), len(predators))

        for square in self.board:
            square.food = 250

        for creature in prey:
            creature.food_eaten = 0
            creature.dies_in -= 1

            square = self.random_square()
            square.prey.append(creature)
        
        for creature in predators:
            creature.food_eaten = 0
            creature.dies_in -= 1

            square = self.random_square()
            # print(len(square.predators))
            square.predators.append(creature)
            # print("after", len(square.predators))

        return stats