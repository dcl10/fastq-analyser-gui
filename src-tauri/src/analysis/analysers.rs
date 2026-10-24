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
    analyse_fastq_reader_batches(reader, batch_size, |batch| {
        results.extend(batch);
        Ok(())
    })?;
    Ok(results)
}

/// Analyses every record from `reader`, `batch_size` records at a time, handing each batch's results to `on_batch`
/// rather than keeping them, so memory stays at about one batch however long the input is. An error from `on_batch`
/// stops the analysis and is returned.
pub fn analyse_fastq_reader_batches<B: BufRead, F>(
    reader: fastq::Reader<B>,
    batch_size: usize,
    mut on_batch: F,
) -> Result<(), String>
where
    F: FnMut(Vec<FastqSeqResult>) -> Result<(), String>,
{
    let mut batch = Vec::with_capacity(batch_size);
    let mut offset = 0;

    for (i, rec) in reader.records().enumerate() {
        batch.push(rec.map_err(|e| format!("Record {}: {e}", i + 1))?);
        if batch.len() == batch_size {
            on_batch(analyse_fastq_records(&batch, offset)?)?;
            offset += batch.len();
            batch.clear();
        }
    }
    if !batch.is_empty() {
        on_batch(analyse_fastq_records(&batch, offset)?)?;
    }

    Ok(())
}

/// Analyses every record from `reader`, `batch_size` records at a time, so only one batch of parsed records is held
/// in memory at once.
pub fn analyse_fasta_reader<B: BufRead>(
    reader: fasta::Reader<B>,
    batch_size: usize,
) -> Result<Vec<FastaSeqResult>, String> {
    let mut results = Vec::new();
    analyse_fasta_reader_batches(reader, batch_size, |batch| {
        results.extend(batch);
        Ok(())
    })?;
    Ok(results)
}

/// Analyses every record from `reader`, `batch_size` records at a time, handing each batch's results to `on_batch`
/// rather than keeping them, so memory stays at about one batch however long the input is. An error from `on_batch`
/// stops the analysis and is returned.
pub fn analyse_fasta_reader_batches<B: BufRead, F>(
    reader: fasta::Reader<B>,
    batch_size: usize,
    mut on_batch: F,
) -> Result<(), String>
where
    F: FnMut(Vec<FastaSeqResult>) -> Result<(), String>,
{
    let mut batch = Vec::with_capacity(batch_size);
    let mut offset = 0;

    for (i, rec) in reader.records().enumerate() {
        batch.push(rec.map_err(|e| format!("Record {}: {e}", i + 1))?);
        if batch.len() == batch_size {
            on_batch(analyse_fasta_records(&batch, offset)?)?;
            offset += batch.len();
            batch.clear();
        }
    }
    if !batch.is_empty() {
        on_batch(analyse_fasta_records(&batch, offset)?)?;
    }

    Ok(())
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
}

#[cfg(test)]
mod tests {
    use bio::io::{fasta, fastq};

    use super::{
        analyse_fasta_reader, analyse_fasta_reader_batches, analyse_fastq_reader, analyse_fastq_reader_batches,
        calc_phred_score,
    };

    fn fasta_input(n: usize) -> String {
        (1..=n).map(|i| format!(">id{i}\nATGC\n")).collect()
    }

    fn fastq_input(n: usize) -> String {
        (1..=n).map(|i| format!("@id{i}\nATGC\n+\n!!!!\n")).collect()
    }

    #[test]
    fn test_analyse_fasta_reader_across_batches() {
        let input = fasta_input(10);
        let results = analyse_fasta_reader(fasta::Reader::new(input.as_bytes()), 3).unwrap();

        let ids: Vec<String> = results.into_iter().map(|r| r.id).collect();
        let expected: Vec<String> = (1..=10).map(|i| format!("id{i}")).collect();
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_analyse_fastq_reader_across_batches() {
        let input = fastq_input(10);
        let results = analyse_fastq_reader(fastq::Reader::new(input.as_bytes()), 3).unwrap();

        let ids: Vec<String> = results.into_iter().map(|r| r.id).collect();
        let expected: Vec<String> = (1..=10).map(|i| format!("id{i}")).collect();
        assert_eq!(ids, expected);
    }

    #[test]
    fn test_analyse_fasta_reader_error_in_later_batch() {
        // Record 8 has an invalid character, so it fails validation in the third batch of 3.
        let mut input = fasta_input(7);
        input.push_str(">bad\nAT GC\n");
        input.push_str(&fasta_input(2));

        let err = analyse_fasta_reader(fasta::Reader::new(input.as_bytes()), 3).unwrap_err();
        assert!(err.starts_with("Record 8 (bad)"), "unexpected error: {err}");
    }

    #[test]
    fn test_analyse_fastq_reader_error_in_later_batch() {
        // Record 8's quality is shorter than its sequence, so it fails validation in the third batch of 3.
        let mut input = fastq_input(7);
        input.push_str("@bad\nATGC\n+\n!!\n");
        input.push_str(&fastq_input(2));

        let err = analyse_fastq_reader(fastq::Reader::new(input.as_bytes()), 3).unwrap_err();
        assert!(err.starts_with("Record 8 (bad)"), "unexpected error: {err}");
    }

    #[test]
    fn test_analyse_reader_empty_input() {
        assert!(analyse_fasta_reader(fasta::Reader::new(&b""[..]), 3).unwrap().is_empty());
        assert!(analyse_fastq_reader(fastq::Reader::new(&b""[..]), 3).unwrap().is_empty());
    }

    #[test]
    fn test_analyse_fastq_reader_batches_sizes() {
        let input = fastq_input(10);
        let mut sizes = Vec::new();
        analyse_fastq_reader_batches(fastq::Reader::new(input.as_bytes()), 4, |batch| {
            sizes.push(batch.len());
            Ok(())
        })
        .unwrap();
        assert_eq!(sizes, vec![4, 4, 2]);
    }

    #[test]
    fn test_analyse_fasta_reader_batches_sizes() {
        let input = fasta_input(10);
        let mut sizes = Vec::new();
        analyse_fasta_reader_batches(fasta::Reader::new(input.as_bytes()), 5, |batch| {
            sizes.push(batch.len());
            Ok(())
        })
        .unwrap();
        assert_eq!(sizes, vec![5, 5]);
    }

    #[test]
    fn test_analyse_reader_batches_stops_on_callback_error() {
        let input = fastq_input(10);
        let mut calls = 0;
        let err = analyse_fastq_reader_batches(fastq::Reader::new(input.as_bytes()), 3, |_| {
            calls += 1;
            Err("stop".to_owned())
        })
        .unwrap_err();
        assert_eq!(err, "stop");
        assert_eq!(calls, 1);

        let input = fasta_input(10);
        let mut calls = 0;
        let err = analyse_fasta_reader_batches(fasta::Reader::new(input.as_bytes()), 3, |_| {
            calls += 1;
            Err("stop".to_owned())
        })
        .unwrap_err();
        assert_eq!(err, "stop");
        assert_eq!(calls, 1);
    }

    #[test]
    fn test_calc_phred_score() {
        assert_eq!(calc_phred_score(b"!!!"), 0);
        assert_eq!(calc_phred_score(b"*+"), 19)
    }
}
