use rayon::prelude::*;
use std::collections::HashMap;

use crate::kmer;

pub fn count_kmers_parallel(sequences: &[String], k: usize) -> HashMap<String, usize> {
    sequences
        .par_iter()
        .map(|seq| {
            let mut local_counts = HashMap::new();
            kmer::count_kmers(seq, k, &mut local_counts);
            local_counts
        })
        .reduce(
            HashMap::new,
            |mut global_counts, local_counts| {
                for (kmer, count) in local_counts {
                    *global_counts.entry(kmer).or_insert(0) += count;
                }
                global_counts
            },
        )
}
