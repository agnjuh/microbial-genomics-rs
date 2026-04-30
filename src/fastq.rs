use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parallel;

pub struct FastqStats {
    pub reads: usize,
    pub gc_content: f64,
    pub kmer_counts: HashMap<String, usize>,
}

fn merge_counts(global: &mut HashMap<String, usize>, local: HashMap<String, usize>) {
    for (kmer, count) in local {
        *global.entry(kmer).or_insert(0) += count;
    }
}

pub fn compute_stats(path: &str, k: usize) -> FastqStats {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let chunk_size = 10_000;

    let mut line_count = 0;
    let mut gc_count = 0;
    let mut base_count = 0;
    let mut kmer_counts: HashMap<String, usize> = HashMap::new();

    let mut sequence_buffer: Vec<String> = Vec::with_capacity(chunk_size);

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading line");

        if i % 4 == 1 {
            base_count += line.len();
            gc_count += line
                .chars()
                .filter(|c| *c == 'G' || *c == 'C')
                .count();

            sequence_buffer.push(line);

            if sequence_buffer.len() >= chunk_size {
                let local_counts = parallel::count_kmers_parallel(&sequence_buffer, k);
                merge_counts(&mut kmer_counts, local_counts);
                sequence_buffer.clear();
            }
        }

        line_count += 1;
    }

    if !sequence_buffer.is_empty() {
        let local_counts = parallel::count_kmers_parallel(&sequence_buffer, k);
        merge_counts(&mut kmer_counts, local_counts);
    }

    let reads = line_count / 4;
    let gc_content = gc_count as f64 / base_count as f64;

    FastqStats {
        reads,
        gc_content,
        kmer_counts,
    }
}