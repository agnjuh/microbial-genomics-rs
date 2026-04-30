use serde::Serialize;
use std::env;
use std::fs::File;
use std::io::Write;

mod complexity;
mod fastq;
mod kmer;
mod parallel;

#[derive(Serialize)]
struct KmerRecord {
    kmer: String,
    count: usize,
    low_complexity: bool,
}

#[derive(Serialize)]
struct Report {
    input_file: String,
    reads: usize,
    gc_content: f64,
    k: usize,
    unique_kmers: usize,
    top_kmers: Vec<KmerRecord>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <FASTQ file> [k] [output.json]", args[0]);
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

    let stats = fastq::compute_stats(filepath, k);

    let top_kmers: Vec<KmerRecord> = kmer::top_kmers(&stats.kmer_counts, 10)
        .into_iter()
        .map(|(kmer, count)| KmerRecord {
            low_complexity: complexity::is_low_complexity(&kmer, 0.8),
            kmer,
            count,
        })
        .collect();

    let report = Report {
        input_file: filepath.to_string(),
        reads: stats.reads,
        gc_content: stats.gc_content,
        k,
        unique_kmers: stats.kmer_counts.len(),
        top_kmers,
    };

    if let Some(path) = output_path {
        let json = serde_json::to_string_pretty(&report).expect("Could not serialize report");
        let mut file = File::create(path).expect("Could not create output file");
        file.write_all(json.as_bytes())
            .expect("Could not write JSON report");
    } else {
        println!("Reads: {}", report.reads);
        println!("GC content: {:.4}", report.gc_content);
        println!("Unique {}-mers: {}", report.k, report.unique_kmers);

        println!("\nTop {}-mers:", report.k);
        println!("kmer\tcount\tlow_complexity");

        for record in report.top_kmers {
            println!(
                "{}\t{}\t{}",
                record.kmer, record.count, record.low_complexity
            );
        }
    }
}