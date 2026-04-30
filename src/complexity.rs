pub fn is_homopolymer(kmer: &str) -> bool {
    if kmer.is_empty() {
        return false;
    }

    let first = kmer.chars().next().unwrap();
    kmer.chars().all(|c| c == first)
}

pub fn max_base_fraction(kmer: &str) -> f64 {
    let len = kmer.len() as f64;

    let a = kmer.matches('A').count() as f64;
    let c = kmer.matches('C').count() as f64;
    let g = kmer.matches('G').count() as f64;
    let t = kmer.matches('T').count() as f64;

    a.max(c).max(g).max(t) / len
}

pub fn is_low_complexity(kmer: &str, threshold: f64) -> bool {
    is_homopolymer(kmer) || max_base_fraction(kmer) >= threshold
}
