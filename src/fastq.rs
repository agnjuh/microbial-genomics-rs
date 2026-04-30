
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::kmer;

pub struct FastqStats {
    pub reads: usize,
    pub gc_content: f64,
    pub kmer_counts: HashMap<String, usize>,
}

pub fn compute_stats(path: &str, k: usize) -> FastqStats {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut gc_count = 0;
    let mut base_count = 0;
    let mut kmer_counts: HashMap<String, usize> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");

        if i % 4 == 1 {
            base_count += line.len();
            gc_count += line
                .chars()
                .filter(|c| *c == 'G' || *c == 'C')
                .count();

            kmer::count_kmers(&line, k, &mut kmer_counts);
        }

        line_count += 1;
    }

    let reads = line_count / 4;
    let gc_content = gc_count as f64 / base_count as f64;

    FastqStats {
        reads,
        gc_content,
        kmer_counts,
    }
}