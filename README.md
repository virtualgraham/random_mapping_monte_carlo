# Random Mapping Monte Carlo

Runs monte carlo simulations to study iterativley calling random mapping functions.

```
SUBCOMMANDS:
    entropy <mapping_size_bits> <depth> <iterations> <rounds>
    search <mapping_size_bits> <treasure_depth> <search_depth> <iterations> <rounds>
```

Both commands genrate a random mapping of integers [0, 2^mapping_size_bits) -> [0, 2^mapping_size_bits). 

`entropy` Returns the estimated entropy of the output space of the random mapping when iterated to the given depth.

`search` First this picks a value by selecting a random start point and iterating to the treasure depth. Then is searches for that value (treasure) while iterating to the search depth (search_depth) multiple times (iterations). It returns the percent of rounds where the treasure value was found while searching.

./random_mapping_monte_carlo entropy 7 5 10000 10000
> 4.993751745581513

For related work see [Collision Attacks based on the Entropy Loss caused by Random Functions - Andrea Ro ̈ck](https://www.rocq.inria.fr/secret/Andrea.Roeck/pdfs/wework_randFct.pdf)
