from creatures import Predator, Prey
from random import choice, shuffle, randint
from matplotlib import pyplot as plt
from math import log10
import pandas as pd

class Square():
    def __init__(self, predators, prey):
        self.predators = predators
        self.prey = prey
        self.food = 250
        self.generation = 0
        self.pops = []
        self.visions = []

    def __repr__(self):
        info = self.analytics()
        s = "===== SQUARE INFO =====\n"
        s += "|--- predator info\n"
        s += f"|    |--- total: {info['predators']['total']}\n"
        s += f"|    |--- m/f ratio: {info['predators']['m/f ratio']}\n"
        s += f"|    |--- avg trait score: {info['predators']['avg trait score']}\n"
        s += f"|    |--- avg food gain: {info['predators']['avg food gain']}\n\n"
        s += f"|--- prey info\n"
        s += f"|    |--- total: {info['prey']['total']}\n"
        s += f"|    |--- m/f ratio: {info['prey']['m/f ratio']}\n"
        s += f"|    |--- avg trait score: {info['prey']['avg trait score']}\n"
        s += f"|    |--- avg min mate chance: {info['prey']['avg min mate chance']}\n"
        s += "=" * 23

        return s

    def analytics(self):
        # info = { # TODO: add median scores
        #     "predators": {"total": 0, "males": 0, "females": 0, "m/f ratio": 0, "avg trait score": 0, "avg food gain": 0},
        #     "prey": {"total": 0, "males": 0, "females": 0, "m/f ratio": 0, "avg trait score": 0, "avg min mate chance": 0}
        # }
        visions = []

        for pred in self.predators:
            # p = info["predators"]
            # if pred.sex == 0: p["males"] += 1
            # else: p["females"] += 1
            # p["avg trait score"] += pred.vision
            visions.append(pred.vision)
        #     p["avg food gain"] += pred.food_gain

        # for prey in self.prey:
        #     p = info["prey"]
        #     if prey.sex == 0:
        #         p["males"] += 1
        #         p["avg min mate chance"] += prey.mate_chance_min
        #     else: p["females"] += 1
        #     p["avg trait score"] += prey.camouflage

        # pred_len = len(self.predators)
        # if pred_len > 0:
        #     info["predators"]["total"] = pred_len
        #     info["predators"]["avg trait score"] /= pred_len
        #     info["predators"]["avg food gain"] /= pred_len
        #     info["predators"]["m/f ratio"] = info["predators"]["males"] / info["predators"]["females"]

        # prey_len = len(self.prey)
        # if prey_len > 0:
        #     info["prey"]["total"] = prey_len
        #     info["prey"]["avg trait score"] /= prey_len
        #     info["prey"]["m/f ratio"] = info["prey"]["males"] / info["prey"]["females"]
        #     if info["prey"]["males"] > 0:
        #         info["prey"]["avg min mate chance"] /= info["prey"]["males"]
        self.pops.append((len(self.prey), len(self.predators)))
        # g = plt.hist(visions, color="lightgreen", bins=50)
        # plt.savefig(f"plots/{self.generation}.png")
        self.visions.append(visions)
        # return info

    def find_mate(self, is_pred):
        creatures = self.predators if is_pred else self.prey
        mates = []
        print(len(creatures), type(creatures))
        for creature in creatures:
            if creature.sex != 1 or creature.hunger < 2 or creature.lives <= 0: continue
            # print("mate found", creature, len(creatures))
            mates.append(creature)
        return choice(mates)
    
    def feed(self):
        for pred in self.predators:
            pred.hunger -= pred.hunger_cost
            prey = choice(self.prey)
            if pred.vision > prey.camouflage:
                self.prey.remove(prey)
                pred.hunger += pred.food_gain # 1.44905 * log10(pred.food_gain) will have the intersection pt @ (24, 2). no transformation = (13.604, 1.134)
            else:
                prey.hunger -= prey.hunger_cost

        for prey in self.prey:
            if prey.food_chance_max <= randint(1, 10):
                self.food -= 1
                prey.hunger += log10(prey.food_chance_max) # TODO: MAKE THIS NOT CONSTANT
                if prey.food_chance_max <= randint(1, 10):
                    self.food -= 1
                    prey.hunger += log10(prey.food_chance_max) # TODO: MAKE THIS NOT CONSTANT
        
    def reproduce(self):
        for creatures in [self.predators, self.prey]:
            is_pred = True if creatures == self.predators else False
            new_creatures = []
            for creature in creatures:
                if creature.hunger < 2 and creature.sex == 1 and creature.lives <= 0: continue
                mate = self.find_mate(is_pred)
                if 5 > randint(1, 10): continue
                new_creatures.append(creature.reproduce(mate)) # 50/50 chance to reproduce, may want to change this
                # new_creatures.append(creature.reproduce(mate))
                # new_creatures.append(creature.reproduce(mate))
                # creature.lives = 0
                # mate.lives = 0
            creatures.extend(new_creatures)

    def prepare(self): # prepare for the next round by removing dead creatures & resupplying food
        self.food = 100
        surviving_preds = []
        for pred in self.predators:
            if pred.lives <= 0 or pred.hunger <= 0: continue
            pred.lives -= 1
            pred.hunger = 1
            surviving_preds.append(pred)
        
        surviving_prey = []
        for prey in self.prey:
            if prey.lives <= 0 or prey.hunger <= 0: continue
            prey.lives -= 1
            prey.hunger = 1
            surviving_prey.append(prey)



class Board():
    def __init__(self):
        self.squares = Square( # for anyone actually trying to read this code i am so sorry for what you're about to see
            [Predator(
                ''.join([choice('ATGC') for _ in range(7)])
                ) for _ in range(15)
            ], # 500 predators with random genomes
            [Prey(
                ''.join([choice('ATGC') for _ in range(7)])
                ) for _ in range(50)
            ] # 500 prey with random genomes
        )# for _ in range(10)] # 10 squares in the board

    def run(self):
        for i in range(20):
            print("gen", i)
            self.squares.generation = i
            self.squares.feed()
            self.squares.reproduce()
            self.squares.prepare()
            print(self.squares.analytics())
        df = pd.DataFrame({"Prey Population": [i[0] for i in self.squares.pops], "Predator Population": [i[1] for i in self.squares.pops]})
        df.plot().get_figure().savefig(f"plots/pops.png")
        df2 = pd.DataFrame({"Final scores": self.squares.visions[-1]})
        df2.plot(kind="kde").get_figure().savefig(f"plots/dists_final.png")

b = Board()
b.run()