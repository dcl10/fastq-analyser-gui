use crate::models::{FastaSeqResult, FastqSeqResult};
use bio::io::{fasta, fastq};
use bio::seq_analysis::{gc, orf};
use bio::utils::TextSlice;
use rayon::prelude::*;
use std::io::BufRead;

/// Number of records parsed and held in memory at once when analysing a reader.
pub const BATCH_SIZE: usize = 10_000;

/// Analyses every record from `reader`, `batch_size` records at a time, so only one batch of parsed records is held
/// in memory at once.
pub fn analyse_fastq_reader<B: BufRead>(
    reader: fastq::Reader<B>,
    batch_size: usize,
) -> Result<Vec<FastqSeqResult>, String> {
    let mut results = Vec::new();
    let mut batch = Vec::with_capacity(batch_size);
    let mut offset = 0;

    for (i, rec) in reader.records().enumerate() {
        batch.push(rec.map_err(|e| format!("Record {}: {e}", i + 1))?);
        if batch.len() == batch_size {
            results.extend(analyse_fastq_records(&batch, offset)?);
            offset += batch.len();
            batch.clear();
        }
    }
    if !batch.is_empty() {
        results.extend(analyse_fastq_records(&batch, offset)?);
    }

    Ok(results)
}

/// Analyses every record from `reader`, `batch_size` records at a time, so only one batch of parsed records is held
/// in memory at once.
pub fn analyse_fasta_reader<B: BufRead>(
    reader: fasta::Reader<B>,
    batch_size: usize,
) -> Result<Vec<FastaSeqResult>, String> {
    let mut results = Vec::new();
    let mut batch = Vec::with_capacity(batch_size);
    let mut offset = 0;

    for (i, rec) in reader.records().enumerate() {
        batch.push(rec.map_err(|e| format!("Record {}: {e}", i + 1))?);
        if batch.len() == batch_size {
            results.extend(analyse_fasta_records(&batch, offset)?);
            offset += batch.len();
            batch.clear();
        }
    }
    if !batch.is_empty() {
        results.extend(analyse_fasta_records(&batch, offset)?);
    }

    Ok(results)
}

/// Analyses `records`, where `offset` is the number of records before them in the input so error messages report
/// file-wide record numbers.
pub fn analyse_fastq_records(records: &[fastq::Record], offset: usize) -> Result<Vec<FastqSeqResult>, String> {
    // Iterate over results and find GC content and ORFs
    let results: Vec<FastqSeqResult> = records
        .par_iter()
        .enumerate()
        .map(|(i, rec)| match rec.check() {
            Ok(()) => Ok(FastqSeqResult {
                n_orfs: (find_orfs(rec.seq()) as u32),
                id: rec.id().to_owned(),
                desc: match rec.desc() {
                    Some(desc) => Some(desc.to_owned()),
                    None => None,
                },
                gc: gc::gc_content(rec.seq()),
                phred_score: calc_phred_score(rec.qual()),
                seq_len: (rec.seq().len() as u32),
            }),
            Err(e) => Err(format!("Record {} ({}): {e}", offset + i + 1, rec.id())),
        })
        .collect::<Result<_, _>>()?;

    Ok(results)
}

/// Analyses `records`, where `offset` is the number of records before them in the input so error messages report
/// file-wide record numbers.
pub fn analyse_fasta_records(records: &[fasta::Record], offset: usize) -> Result<Vec<FastaSeqResult>, String> {
    // Iterate over results and find GC content and ORFs
    let results: Vec<FastaSeqResult> = records
        .par_iter()
        .enumerate()
        .map(|(i, rec)| match rec.check() {
            Ok(_) => Ok(FastaSeqResult {
                n_orfs: (find_orfs(rec.seq()) as u32),
                id: rec.id().to_owned(),
                desc: match rec.desc() {
                    Some(desc) => Some(desc.to_owned()),
                    None => None,
                },
                gc: gc::gc_content(rec.seq()),
                seq_len: (rec.seq().len() as u32),
            }),
            Err(e) => Err(format!("Record {} ({}): {e}", offset + i + 1, rec.id())),
        })
        .collect::<Result<_, _>>()?;

    Ok(results)
}

fn find_orfs(seq: TextSlice) -> usize {
    // Hyperparameters for finding open reading frames (ORFs).
    // NB: DNA alphabet
    let start_codons = vec![b"ATG"];
    let stop_codons = vec![b"TGA", b"TAG", b"TAA"];
    let min_len = 50;
    let finder = orf::Finder::new(start_codons, stop_codons, min_len);

    finder.find_all(seq).count()
}

fn calc_phred_score(qual: &[u8]) -> u32 {
    let mut score = 0;
    for q in qual {
        let q_32bit = u32::from(*q);
        score += q_32bit - 33;
    }
    score
}#[cfg(test)]
mod tests {
    use super::calc_phred_score;

    #[test]
    fn test_calc_phred_score() {
        assert_eq!(calc_phred_score(b"!!!"), 0);
        assert_eq!(calc_phred_score(b"*+"), 19)
    }
}
