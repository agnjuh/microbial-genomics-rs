use std::collections::HashMap;

pub struct GraphStats {
    pub nodes: usize,
    pub edges: usize,
    pub avg_out_degree: f64,
}

pub fn build_debruijn_stats(kmer_counts: &HashMap<String, usize>) -> GraphStats {
    let mut adjacency: HashMap<String, usize> = HashMap::new();
    let mut edge_count = 0;

    for (kmer, count) in kmer_counts {
        if kmer.len() < 2 {
            continue;
        }

        let prefix = &kmer[..kmer.len() - 1];
        let suffix = &kmer[1..];

        *adjacency.entry(prefix.to_string()).or_insert(0) += count;
        adjacency.entry(suffix.to_string()).or_insert(0);

        edge_count += count;
    }

    let node_count = adjacency.len();

    let total_out: usize = adjacency.values().sum();

    let avg_out_degree = if node_count > 0 {
        total_out as f64 / node_count as f64
    } else {
        0.0
    };

    GraphStats {
        nodes: node_count,
        edges: edge_count,
        avg_out_degree,
    }
}
