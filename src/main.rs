use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::Write;

mod complexity;
mod fastq;
mod graph;
mod kmer;
mod parallel;

#[derive(Serialize)]
struct KmerRecord {
    kmer: String,
    count: usize,
    frequency_per_read: f64,
    low_complexity: bool,
}

#[derive(Serialize)]
struct GraphReport {
    nodes: usize,
    weighted_edges: usize,
    avg_out_degree: f64,
}

#[derive(Serialize)]
struct Report {
    input_file: String,
    reads: usize,
    gc_content: f64,
    k: usize,
    unique_kmers: usize,
    top_kmers: Vec<KmerRecord>,
    graph: GraphReport,
}

fn write_top_kmers_csv(path: &str, top_kmers: &[KmerRecord]) {
    let mut file = File::create(path).expect("Could not create CSV file");

    writeln!(file, "kmer,count,frequency_per_read,low_complexity")
        .expect("Could not write CSV header");

    for record in top_kmers {
        writeln!(
            file,
            "{},{},{:.6},{}",
            record.kmer,
            record.count,
            record.frequency_per_read,
            record.low_complexity
        )
        .expect("Could not write CSV row");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <FASTQ file> [k] [output.json] [top_kmers.csv]",
            args[0]
        );
        std::process::exit(1);
    }

    let filepath = &args[1];

    let k: usize = if args.len() >= 3 {
        args[2].parse().expect("k must be a positive integer")
    } else {
        5
    };

    let output_path = if args.len() >= 4 {
        Some(&args[3])
    } else {
        None
    };

    let csv_path = if args.len() >= 5 {
        Some(&args[4])
    } else {
        None
    };

    let stats = fastq::compute_stats(filepath, k);

    let top_kmers: Vec<KmerRecord> = kmer::top_kmers(&stats.kmer_counts, 10)
        .into_iter()
        .map(|(kmer, count)| KmerRecord {
            frequency_per_read: count as f64 / stats.reads as f64,
            low_complexity: complexity::is_low_complexity(&kmer, 0.8),
            kmer,
            count,
        })
        .collect();

    let graph_stats = graph::build_debruijn_stats(&stats.kmer_counts);

    let report = Report {
        input_file: filepath.to_string(),
        reads: stats.reads,
        gc_content: stats.gc_content,
        k,
        unique_kmers: stats.kmer_counts.len(),
        top_kmers,
        graph: GraphReport {
            nodes: graph_stats.nodes,
            weighted_edges: graph_stats.edges,
            avg_out_degree: graph_stats.avg_out_degree,
        },
    };

    if let Some(path) = output_path {
        let json = serde_json::to_string_pretty(&report).expect("Could not serialize report");
        let mut file = File::create(path).expect("Could not create output file");
        file.write_all(json.as_bytes())
            .expect("Could not write JSON report");
    }

    if let Some(path) = csv_path {
        write_top_kmers_csv(path, &report.top_kmers);
    }

    if output_path.is_none() {
        println!("Reads: {}", report.reads);
        println!("GC content: {:.4}", report.gc_content);
        println!("Unique {}-mers: {}", report.k, report.unique_kmers);

        println!(
            "\nGraph stats: nodes={}, weighted_edges={}, avg_out_degree={:.4}",
            report.graph.nodes,
            report.graph.weighted_edges,
            report.graph.avg_out_degree
        );

        println!("\nTop {}-mers:", report.k);
        println!("kmer\tcount\tfrequency_per_read\tlow_complexity");

        for record in &report.top_kmers {
            println!(
                "{}\t{}\t{:.6}\t{}",
                record.kmer,
                record.count,
                record.frequency_per_read,
                record.low_complexity
            );
        }
    }
}