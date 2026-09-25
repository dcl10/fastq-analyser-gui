use crate::models::{FastaSeqResult, FastqSeqResult};
use bio::io::{fasta, fastq};
use bio::seq_analysis::{gc, orf};
use bio::utils::TextSlice;
use rayon::prelude::*;

pub fn analyse_fastq_records(records: &Vec<fastq::Record>) -> Result<Vec<FastqSeqResult>, String> {
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
            Err(e) => Err(format!("Record {} ({}): {e}", i + 1, rec.id())),
        })
        .collect::<Result<_, _>>()?;

    Ok(results)
}

pub fn analyse_fasta_records(records: &Vec<fasta::Record>) -> Result<Vec<FastaSeqResult>, String> {
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
            Err(e) => Err(format!("Record {} ({}): {e}", i + 1, rec.id())),
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
    use super::calc_phred_score;

    #[test]
    fn test_calc_phred_score() {
        assert_eq!(calc_phred_score(b"!!!"), 0);
        assert_eq!(calc_phred_score(b"*+"), 19)
    }
}
