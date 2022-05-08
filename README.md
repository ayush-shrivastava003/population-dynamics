# population-dynamics

WIP population simulator loosely based off of [this](https://github.com/henry-lang/bio-sim) repo by @henry-lang and [this](https://www.youtube.com/watch?v=r_It_X7v-1E) video by Sebastian Lague. Follows the rules of poulation dynamics but adds basic concepts of genetics & inheritance to further replicate real life.

## simulation rules
* board has 40 squares
* each square contains 20 pieces of food and a variable amount of rabbits

* all rabbits must eat 1 food to survive and 2 to have the option to reproduce
* a female will only accept a male if she isn't already pregnant
* females will be pregnant for 3 days

* black fur males have an 80% chance of female acceptance
* grey fur males have a 50% chance of female acceptance
* white fur males have a 10% chance of female acceptance

## observations

* The ratio of fur types is highly dependent on the starting population. If both parents are homozygous (one dominant and the other recessive), gray fur will be in the minority (roughly 15%), and the other two will be about equal (roughly 40%). If both are heterozygous, gray fur will prosper, taking 95% of the population. This is particularly interesting because I had suspected that black fur would perform the best and white fur would be almost extinct due to their acceptance probabilities.

* It was not a surprise, but a strong foraging ability was very much favored over a weak one, with strong foragers accounting for about 98% of the population.

* In earlier tests, where the starting population's genotype was totally random, the male-to-female ratio was about 3:1. When I hardcoded the starting genotypes, the ratio was almost exactly 1:1. This seems to suggest that a lot of males' sex gene was D-D rather than D-R, so I'm glad that that error was sorted out.

## possible improvements

* Currently, foraging is completely binary - you're either really good at finding food or awful. I might implement a wider range of foraging abilities to see an actual bell-curve-type distribution.

* There's likely an error that's present with the mating system. I didn't expect white fur to be equally as common as black fur, since their probabilities of finding a mate are significantly different. However, I'm not entirely sure what the root of the problem could be.

* Each generation, all food instantly grows back. This allowed the population to climb extremely easily since there wasn't too much concern for finding food. I might instead incorporate a regenerative system, where food slowly grows back over the course of five generations (e.g 20 per gen, bringing it back to 100 after 5 gens). Then again, the huge demand for food might keep it ranging exlusively between 0-20.

* A predator species is definitely going to be implemented at some point (that is the eventual goal, hence the repo name "population-dynamics"), but the way they interact with the prey might change. There might be a speed gene which will increase their chances of finding prey.

* An energy system should be implemented. There's currently no cost to having black fur, or being good at foraging. Perhaps those abilities require more energy than their opposites. Making those extremely helpful attributes come at a cost may at least slow the directional selection.