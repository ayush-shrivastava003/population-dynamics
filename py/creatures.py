import random
from enum import Enum
# from types import List

class Allele(Enum):
    Dominant = "D",
    Recessive = "r"

class Sex(Enum):
    Male = "M",
    Female = "F"

class Fur(Enum):
    Black = "B",
    Gray = "G",
    White = "W"

class Vision(Enum):
    NearSighted = "NS"
    FarSighted = "FS"

class PreyGenotype():
    def __init__(self, fur: (Allele), sex: (Allele)):
        self.fur = fur
        self.sex = sex

class PredatorGenotype():
    def __init__(self, vision, sex):
        self.vision = vision
        self.sex = sex

class Prey():
    def __init__(self, genotype: PreyGenotype):
        if genotype.fur[0] == Allele.Dominant:
            if genotype.fur[1] == Allele.Dominant:
                self.fur = Fur.Black
            else:
                self.fur = Fur.Gray
        elif genotype.fur[1] == Allele.Dominant:
            self.fur = Fur.Gray
        else:
            self.fur = Fur.White
        
        if genotype.sex[0] == Allele.Dominant or genotype.sex[1] == Allele.Dominant:
            self.sex = Sex.Male
        else:
            self.sex = Sex.Female
        
        self.food_eaten = 0
        self.dies_in = 5
        self.genotype = genotype

    def __repr__(self) -> str:
        return f"Prey {self.sex}/{self.fur}/{self.dies_in}/{self.food_eaten}"

    def genes(self):
        """
        Select a random allele from each trait. Alleles are passed on to the child.
        """
        traits = [self.genotype.fur, self.genotype.sex]
        
        return [random.choice(i) for i in traits]

    def reproduce(mother, father):
        """
        Using the mother and father's randomly chosen genes, a new prey is created.
        """
        mother_genes = mother.genes()
        father_genes = father.genes()

        genotype = PreyGenotype(
            fur = [mother_genes[0], father_genes[0]],
            sex = [mother_genes[1], father_genes[1]]
        )

        return Prey(genotype)

class Predator():
    def __init__(self, genotype: PredatorGenotype):
        if genotype.sex[0] == Allele.Dominant or genotype.sex[1] == Allele.Dominant:
            self.sex = Sex.Male
        else:
            self.sex = Sex.Female
        
        if genotype.vision[0] == Allele.Dominant or genotype.vision[1] == Allele.Dominant:
            self.vision = Vision.NearSighted
        else:
            self.vision = Vision.FarSighted 

        self.dies_in = 3
        self.food_eaten = 0
        self.genotype = genotype

    def __repr__(self) -> str:
        return f"Predator {self.sex}/{self.vision}/{self.dies_in}/{self.food_eaten}"

    def genes(self):
        """
        Select a random allele from each trait. Alleles are passed on to the child.
        """
        traits = [self.genotype.vision, self.genotype.sex]
        
        return [random.choice(i) for i in traits]

    def reproduce(mother, father):
        """
        Using the mother and father's randomly chosen genes, a new prey is created.
        """
        mother_genes = mother.genes()
        father_genes = father.genes()

        genotype = PredatorGenotype(
            vision = [mother_genes[0], father_genes[0]],
            sex = [mother_genes[1], father_genes[1]]
        )

        return Predator(genotype)