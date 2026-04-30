use std::collections::HashMap;

pub fn count_kmers(sequence: &str, k: usize, counts: &mut HashMap<String, usize>) {
    if sequence.len() < k {
        return;
    }

    for i in 0..=sequence.len() - k {
        let kmer = &sequence[i..i + k];

        if kmer.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T')) {
            *counts.entry(kmer.to_string()).or_insert(0) += 1;
        }
    }
}

pub fn top_kmers(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut kmers: Vec<(String, usize)> = counts
        .iter()
        .map(|(kmer, count)| (kmer.clone(), *count))
        .collect();

    kmers.sort_by(|a, b| b.1.cmp(&a.1));
    kmers.truncate(n);

    kmers
}
