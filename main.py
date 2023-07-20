from creatures import Predator, Prey
from random import choice, shuffle, randint
from math import log10

class Square():
    def __init__(self, predators, prey):
        self.predators = predators
        self.prey = prey
        self.food = 250

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
        info = { # TODO: add median scores
            "predators": {"total": 0, "males": 0, "females": 0, "m/f ratio": 0, "avg trait score": 0, "avg food gain": 0},
            "prey": {"total": 0, "males": 0, "females": 0, "m/f ratio": 0, "avg trait score": 0, "avg min mate chance": 0}
        }

        for pred in self.predators:
            p = info["predators"]
            if pred.sex == 0: p["males"] += 1
            else: p["females"] += 1
            p["avg trait score"] += pred.vision
            p["avg food gain"] += pred.food_gain

        for prey in self.prey:
            p = info["prey"]
            if prey.sex == 0:
                p["males"] += 1
                p["avg min mate chance"] += prey.mate_chance_min
            else: p["females"] += 1
            p["avg trait score"] += prey.camouflage

        pred_len = len(self.predators)
        if pred_len > 0:
            info["predators"]["total"] = pred_len
            info["predators"]["avg trait score"] /= pred_len
            info["predators"]["avg food gain"] /= pred_len
            info["predators"]["m/f ratio"] = info["predators"]["males"] / info["predators"]["females"]

        prey_len = len(self.prey)
        if prey_len > 0:
            info["prey"]["total"] = prey_len
            info["prey"]["avg trait score"] /= prey_len
            info["prey"]["m/f ratio"] = info["prey"]["males"] / info["prey"]["females"]
            if info["prey"]["males"] > 0:
                info["prey"]["avg min mate chance"] /= info["prey"]["males"]

        return info

    def find_mate(self, is_pred):
        creatures = self.predators if is_pred else self.prey
        mates = []
        for creature in creatures:
            if creature.sex != 1 and creature.hunger < 2: continue
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
                prey.hunger += 0.5 # TODO: MAKE THIS NOT CONSTANT
                if prey.food_chance_max <= randint(1, 10):
                    self.food -= 1
                    prey.hunger += 0.5 # TODO: MAKE THIS NOT CONSTANT
        
    def reproduce(self):
        for creatures in [self.predators, self.prey]:
            is_pred = True if creatures == self.predators else False
            for creature in creatures:
                if creature.hunger < 2 and creature.sex == 1: continue
                mate = self.find_mate(is_pred)
                if 5 > randint(1, 10): continue
                creatures.append(creature.reproduce(mate)) # 50/50 chance to reproduce, may want to change this



class Board():
    def __init__(self):
        self.squares = [Square( # for anyone actually trying to read this code i am so sorry for what you're about to see
            [Predator(
                ''.join([choice('ATGC') for _ in range(7)])
                ) for _ in range(150)
            ], # 500 predators with random genomes
            [Prey(
                ''.join([choice('ATGC') for _ in range(7)])
                ) for _ in range(500)
            ] # 500 prey with random genomes
        )]# for _ in range(10)] # 10 squares in the board

    def shuffle_squares(self):
        pass

b = Board()
print(b.squares[0])
for i in range(10):
    print(i)
    b.squares[0].feed()
    b.squares[0].reproduce()
    print(b.squares[0])