from random import sample, choice, randint, shuffle
from math import log10

class Creature():
    def __init__(self, genome):
        self.base_system = {'A': 0, 'T': 1, 'C': 2, 'G': 3}
        self.genome = genome
        self.hunger = 1.5
        self.trait, self.sex, self.hunger_cost = self.parse_genome()

    def __repr__(self):
        s = "===== CREATURE PROFILE ====="
        s += f"\ngenome: {self.genome}"
        s += f"\nsex: {'M' if self.sex == 0 else 'F'}"
        s += f"\ntrait power: {self.trait}"
        s += f"\ncost for trait: {self.hunger_cost}\n"
        return s

    def parse_genome(self):
        trait = 0
        hunger_cost = 0
        sex = 0 if self.base_system[self.genome[-1]] <= 1 else 1

        for s in self.genome[:6]: # trait gene
            trait += self.base_system[s]

        hunger_cost = trait / 12 # max for trait is 24 -> 24 / 12 = 2 (max hunger, i.e will kill creature)

        return trait, sex, hunger_cost

    def gametes(self):
        trait_gene = self.genome[:6]
        trait_gamete = ''.join(sample(trait_gene, len(trait_gene)))[:3]

        if randint(1, 1000) == 1: # 1/1000 chance of mutation
            # print("mutation!")
            idx = randint(0, 2)
            mutation = choice('ATGC'.replace(trait_gamete[idx], ''))
            trait_gamete = list(trait_gamete)
            trait_gamete[idx] = mutation
            trait_gamete = ''.join(trait_gamete)

        return trait_gamete

    def generate_genome(self, mate):
        l = [self.gametes(), mate.gametes()]
        shuffle(l)
        genome = "".join(l)
        genome += choice((self.genome[-1], mate.genome[-1]))
        return genome

class Predator(Creature):
    def __init__(self, genome):
        super().__init__(genome)
        self.lives = 5
        self.vision = self.trait
        self.food_gain = log10(self.vision) if self.vision > 0 else 0
    
    def __repr__(self):
        s = super().__repr__()
        s += f"food gained back: {self.food_gain}\n"
        s += "============================\n"
        return s

    def reproduce(self, mate):
        return Predator(self.generate_genome(mate))

class Prey(Creature):
    def __init__(self, genome):
        super().__init__(genome)
        self.lives = 2
        self.camouflage = self.trait
        self.mate_chance_min = 1.105 ** self.trait
        self.food_chance_max = self.mate_chance_min

    def __repr__(self):
        s = super().__repr__()
        if self.mate_chance_min:
            s += f"min mate chance & food chance max: {self.mate_chance_min}\n"
        s += "============================\n"
        return s

    def reproduce(self, other):
        return Prey(self.generate_genome(other))