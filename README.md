# microbial-genomics-rs  
A Rust-based approach to scalable processing of microbial sequencing data

## Overview

This repository implements a Rust-based approach for processing microbial sequencing data, with a focus on efficient streaming of FASTQ files, k-mer profiling, and detection of low-complexity sequence patterns.

The work is motivated by challenges in large-scale microbial genomics and emerging single-cell sequencing approaches, where data volumes are substantial and computational efficiency becomes critical.

## Features

- Streaming FASTQ processing (memory-efficient)
- GC content estimation
- k-mer counting with configurable k
- Identification of overrepresented k-mers
- Detection of low-complexity sequence patterns (e.g. homopolymers)
- JSON report generation for downstream analysis
- Snakemake workflow integration

## Dataset

The analysis uses a subset of a publicly available Illumina sequencing dataset of *Escherichia coli*.

To keep the repository lightweight, a subset of approximately 100,000 reads is used.

### Data preparation

```bash
mkdir -p data/raw data/subset

curl -L -o data/raw/ecoli_R1.fastq.gz \
https://ftp.sra.ebi.ac.uk/vol1/fastq/SRR258/001/SRR2584861/SRR2584861_1.fastq.gz

gunzip -c data/raw/ecoli_R1.fastq.gz | head -n 400000 > data/subset/ecoli_100k_R1.fastq
```

## Usage

### Run the Rust tool

```bash
cargo run --release -- data/subset/ecoli_100k_R1.fastq 11 results/ecoli_k11_report.json
```

### Run via Snakemake

snakemake -s workflow/Snakefile --cores 1

## Output

The tool produces a JSON report containing:

- Number of reads  
- GC content  
- Number of unique k-mers  
- Top k-mers with low-complexity annotation  

## Data Interpretation

The observed GC content (~0.50) is consistent with bacterial genomes such as *E. coli*.

For small k (k = 5, 7), the k-mer space is saturated (all possible k-mers are observed), reflecting the diversity and coverage of the dataset. At k = 11, only a subset of all possible k-mers is present, providing a more informative representation of sequence structure.

Highly abundant k-mers include homopolymeric sequences (e.g. "AAAAAAAAAAA"), which are flagged as low-complexity. These may arise from sequencing artefacts, low-complexity regions, or technical biases.

Other frequent k-mers form overlapping sequence patterns (e.g. CTGTCTCTTAT → TGTCTCTTATA), indicating that they originate from highly represented genomic regions.

This illustrates how k-mer profiles can be used to:

- identify sequence biases and artefacts  
- detect repetitive or overrepresented sequence motifs  
- provide a basis for graph-based genome reconstruction (e.g. de Bruijn graphs)  

## Motivation

Modern microbial genomics and single-cell sequencing generate large volumes of data that require efficient and scalable processing.

This work explores how Rust can be used to build high-performance tools for:

- sequence processing  
- k-mer-based analysis  
- future extensions such as genome assembly and indexing  

## Extensions

The current implementation focuses on streaming and k-mer-based summarisation. Natural extensions include:

- parallelisation of k-mer counting to support large-scale datasets  
- construction of graph-based representations (e.g. de Bruijn graphs)  
- extension to single-cell and metagenomic sequencing data  
