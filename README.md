# Random Mapping Monte Carlo

Runs monte carlo simulations to study iterativley calling random mapping functions.

```
SUBCOMMANDS:
    entropy <mapping_size_bits> <depth> <iterations> <rounds>
    search <mapping_size_bits> <treasure_depth> <search_depth> <iterations> <rounds>
```

`entropy` returns the estimated entropy of the output space of the random mapping given the params

`search` returns the percent of rounds where the treasure value was found while searching

./random_mapping_monte_carlo entropy 7 5 10000 10000
> 4.993751745581513