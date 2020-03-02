# Random Mapping Monte Carlo

Runs monte carlo simulations to study iterativley calling random mapping functions.

mapping_size_bits: The log2 size of the set being randomly mapped. 
depth: The number of times to repeat the random mapping, feeding the output back in as input
iterations: The number of times to repeat the depth mapping per instance of a random mapping function. Rule of thumb is at least 50 * 2^mapping_size_bits to 100 * 2^mapping_size_bits.
rounds: The number of random mapping functions to generate and iterate. Generally 1000 to 10000, unless computationally infeasable. 


```
SUBCOMMANDS:
    entropy <mapping_size_bits> <depth> <iterations> <rounds>
    search <mapping_size_bits> <treasure_depth> <search_depth> <iterations> <rounds>
```

Both commands genrate a random mapping of integers [0, 2^mapping_size_bits) -> [0, 2^mapping_size_bits). 

`entropy` Returns the estimated entropy of the output space of the random mapping when iterated to the given depth.

`search` At the start of each round a new random_mapping is generated and a tresure is selected by choosing a random start point and iterating to the treasure depth. Then it searches for that value (treasure) while iterating to the search depth (search_depth) a given number times (iterations). It returns the percent of rounds where the treasure value was found while searching.

./random_mapping_monte_carlo entropy 7 5 12800 1000
> 4.993751745581513


For related work see [Collision Attacks based on the Entropy Loss caused by Random Functions - Andrea Ro ̈ck](https://www.rocq.inria.fr/secret/Andrea.Roeck/pdfs/wework_randFct.pdf)