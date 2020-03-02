use std::env;
use rayon::prelude::*;
use random_fast_rng::{Random, local_rng};

// size is a power of 2
fn rng(mapping_size: usize) -> usize {
    (mapping_size-1) & local_rng().get_usize()
}


struct RandomMapping {
    mapping: Vec<usize>
} 


impl RandomMapping {
    fn new(size: usize) -> RandomMapping {
        RandomMapping {
            mapping: (0..size).map(|_| rng(size)).collect()
        }
    }

    fn value_at(&self, x: usize) -> usize {
        return self.mapping[x]
    }

    fn value_at_depth(&self, x: usize, depth: u32) -> usize {
        let mut v = x;
        for _ in 0..depth {
            v = self.mapping[v];
        }
        v
    }

    fn search_to_depth(&self, x: usize, y:usize, depth: u32) -> bool {
        let mut v = x;
        // if v == y { return true } // ????

        for _ in 0..depth {
            v = self.mapping[v];
            if v == y { return true }
        }
        false
    }
}


fn get_entrophy(counts: &Vec<u64>) -> f64 {
    let sum:u64 = counts.iter().map(|v| *v).sum();

    counts.iter().map(|v| {
        if *v == 0 { return 0f64 }
        -1f64 * ((*v as f64) / (sum as f64)) * ((*v as f64) / (sum as f64)).log2()

    }).sum()
}


#[derive(Debug)]
struct SearchArgs {
    mapping_size_bits: u32,
    treasure_depth: u32,
    search_depth: u32,
    iterations: u64,
    rounds: u64,
}


#[derive(Debug)]
struct EntropyArgs {
    mapping_size_bits: u32,
    depth: u32,
    iterations: u64,
    rounds: u64,
}


enum Args {
    Invalid,
    Search(SearchArgs),
    Entropy(EntropyArgs)
}


fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() == 6 && args[1] == "entropy" {

        let mapping_size_bits: u32 =  match args[2].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let depth: u32 =  match args[3].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let iterations: u64 =  match args[4].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let rounds: u64 =  match args[5].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        return Args::Entropy(EntropyArgs {
            mapping_size_bits,
            depth,
            iterations,
            rounds
        })

    } else if args.len() == 7 && args[1] == "search" {

        let mapping_size_bits: u32 =  match args[2].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let treasure_depth: u32 =  match args[3].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let search_depth: u32 =  match args[4].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let iterations: u64 =  match args[5].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        let rounds: u64 =  match args[6].parse() {
            Ok(n) => n,
            Err(_) => return Args::Invalid,
        };

        return Args::Search(SearchArgs {
            mapping_size_bits,
            treasure_depth,
            search_depth,
            iterations,
            rounds
        })
    } 

    Args::Invalid
}


fn run_search(args: SearchArgs) {
    println!("{:?}", args);

    let mapping_size = 2usize.pow(args.mapping_size_bits);

    let avg_found:f64 = (0..args.rounds).into_par_iter().map(|_| {

        let mapping = RandomMapping::new(mapping_size);
        let treasure = mapping.value_at_depth(rng(mapping_size), args.treasure_depth);

        for _ in 0..args.iterations {
            if mapping.search_to_depth(rng(mapping_size), treasure, args.search_depth) {
                return 1f64/(args.rounds as f64)
            }
        }

        return 0f64

    }).sum();

    println!("{}%", avg_found*100f64);
}


fn run_entropy(args: EntropyArgs) {
    
    println!("{:?}", args);

    let mapping_size = 2usize.pow(args.mapping_size_bits);

    let avg_entropy:f64 = (0..args.rounds).into_par_iter().map(|_| {

        let mut counts = vec![0u64; mapping_size];

        let mapping = RandomMapping::new(mapping_size);

        for _ in 0..args.iterations {
            let v = mapping.value_at_depth(rng(mapping_size), args.depth);
            counts[v] += 1;
        }

        (1f64/(args.rounds as f64)) * get_entrophy(&counts)

    }).sum();

    println!("{}", avg_entropy);

}


fn main() {
    match parse_args() {
        Args::Search(search_args) => run_search(search_args),
        Args::Entropy(entropy_args) => run_entropy(entropy_args),
        Args::Invalid => println!("Invalid Arguments")
    };
}