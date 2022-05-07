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