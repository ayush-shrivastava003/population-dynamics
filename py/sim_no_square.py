# Same as the original simulator.py, but the board is not split up into any squares.
# Testing to see if the squares are limiting population growth.
from creatures import *
import random
import time
from sys import stdout
import pandas as pd
import matplotlib

class Board():
    def __init__(self):
        self.food = 10 * (10**4)
        self.prey = []
        self.predators = []

    def setup(self):
        self.food = 10 * (10**4)
        self.prey.clear()
        self.predators.clear()

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
                        children.append(Prey.reproduce(mate, creature))
                    elif creature.fur == Fur.Gray and mate_chance <= 5:
                        children.append(Prey.reproduce(mate, creature))
                        children.append(Prey.reproduce(mate, creature))
                    elif creature.fur == Fur.White and mate_chance == 1:
                        children.append(Prey.reproduce(mate, creature))
                        children.append(Prey.reproduce(mate, creature))
        self.prey.extend(children)

        children = []
        for creature in self.predators:
            if creature.sex == Sex.Male and creature.food_eaten == 2:
                mates = self.get_mates(False)
                if len(mates) > 0:
                    mate = random.choice(mates)
                    mate_chance = random.randint(1, 10)

                    if mate_chance > 5:
                        children.append(Predator.reproduce(mate, creature))
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

            if 1 < food_chance and food_chance <= 3:
                prey.food_eaten = 0
                prey.dies_in = 0
                creature.food_eaten = 1
            elif food_chance > 3:
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
        self.board = Board()
        self.pops = []

        for _ in range(1000):
            self.board.prey.append(Prey(
                PreyGenotype(
                    fur=[Allele.Dominant, Allele.Dominant],
                    sex=[Allele.Dominant, Allele.Recessive]
            )))
            self.board.prey.append(Prey(
                PreyGenotype(
                    fur=[Allele.Recessive, Allele.Recessive],
                    sex=[Allele.Recessive, Allele.Recessive]
            )))

        for _ in range(25):
            self.board.predators.append(Predator(
                PredatorGenotype(
                    vision=[Allele.Dominant, Allele.Dominant],
                    sex=[Allele.Dominant, Allele.Recessive]
            )))

            self.board.predators.append(Predator(
                PredatorGenotype(
                    vision=[Allele.Recessive, Allele.Recessive],
                    sex=[Allele.Recessive, Allele.Recessive]
            )))
    
        print("Board is set up.")

    def run(self, generations: int, num=None):
        start = time.perf_counter()
        num = num if num else 0
        scale = 100 / generations

        print("Beginning simulation!")
        for gen in range(generations+1):
            self.board.run()
            stats = self.reset_board()
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
        
        data_frame = pd.DataFrame({"Prey population": [i[0] for i in self.pops], "Predator Population": [i[1] for i in self.pops]})
        # writer = pd.ExcelWriter("pop.xlsx", engine="xlsxwriter")
        # data_frame.to_excel(writer, sheet_name="Sheet1")

        # workbook = writer.book
        # # sheet = writer.sheets["Sheet1"]
        # chart = workbook.add_chart({"type": "line"})
        # chart.add_series({
        #     "name": "=Sheet1!$B$1",
        #     "categories": "=Sheet1!$A$1:$A$102",
        #     "values": "=Sheet1!$B$2:$B$102"
        # })
        # chart.add_series({
        #     "name": "=Sheet1!$C$1",
        #     "categories": "=Sheet1!$A$1:$A$102",
        #     "values": "=Sheet1!$C$2:$C$102"
        # })

        # writer.sheets["Sheet1"].insert_chart("E1", chart)
        # writer.save()

        data_frame.plot().get_figure().savefig(f"plots/graph{num}.png")

        print("\nFinished in {} seconds", time.perf_counter() - start)
        print(self.pops)
    
    # def random_square(self):
    #     return random.choice(self.board)

    def collect_creatures(self):
        prey = []
        predators = []

        dead_pred = 0
        dead_prey = 0
        for creature in self.board.prey:
            if creature.dies_in > 0:
                prey.append(creature)
            else:
                dead_prey += 1
        for creature in self.board.predators:
            if creature.dies_in > 0:
                predators.append(creature)
            else:
                dead_pred += 1

        # if len(self.board.prey) > 0:
        #     print(f"{(dead_prey / len(self.board.prey)) * 100} ({dead_prey} / {len(self.board.prey)}) prey died")
        
        # if len(self.board.predators) > 0:
        #     print(f"{(dead_pred / len(self.board.predators)) * 100} ({dead_pred} / {len(self.board.predators)}) predators died")
        
        return prey, predators #mortality_prey, mortality_pred

    def reset_board(self):
        prey, predators = self.collect_creatures()
        stats = (len(prey), len(predators))

        self.board.setup()

        for creature in prey:
            creature.food_eaten = 0
            creature.dies_in -= 1

            self.board.prey.append(creature)
        
        for creature in predators:
            creature.food_eaten = 0
            creature.dies_in -= 1

            self.board.predators.append(creature)

        return stats