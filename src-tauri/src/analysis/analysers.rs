use crate::models::{FastaSeqResult, FastqSeqResult};
use bio::io::{fasta, fastq};
use bio::seq_analysis::{gc, orf};
use bio::utils::TextSlice;

pub fn analyse_fastq_records(records: &Vec<fastq::Record>) -> Vec<FastqSeqResult> {
    let mut results = Vec::new();

    // Iterate over results and find GC content and ORFs
    for rec in records {
        if rec.check().is_ok() {
            let gc_ = gc::gc_content(rec.seq());
            let n_orfs = find_orfs(rec.seq());
            results.push(FastqSeqResult {
                n_orfs,
                id: rec.id().to_owned(),
                desc: rec.desc().unwrap_or("").to_owned(),
                gc: gc_,
                is_valid: rec.check().is_ok(),
                phred_score: calc_phred_score(rec.qual()),
                seq_len: rec.seq().len(),
                ..Default::default()
            });
        } else {
            results.push(FastqSeqResult {
                id: "Invalid Record".to_owned(),
                is_valid: rec.check().is_ok(),
                ..Default::default()
            });
        }
    }

    results
}

pub fn analyse_fasta_records(records: &Vec<fasta::Record>) -> Vec<FastaSeqResult> {
    let mut results = Vec::new();

    // Iterate over results and find GC content and ORFs
    for rec in records {
        if rec.check().is_ok() {
            let gc_ = gc::gc_content(rec.seq());
            let n_orfs = find_orfs(rec.seq());
            results.push(FastaSeqResult {
                n_orfs,
                id: rec.id().to_owned(),
                desc: rec.desc().unwrap_or("").to_owned(),
                gc: gc_,
                is_valid: rec.check().is_ok(),
                seq_len: rec.seq().len(),
                ..Default::default()
            });
        } else {
            results.push(FastaSeqResult {
                id: "Invalid Record".to_owned(),
                is_valid: rec.check().is_ok(),
                ..Default::default()
            });
        }
    }

    results
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
    fn test_cal_phred_score() {
        assert_eq!(calc_phred_score(b"!!!"), 0);
        assert_eq!(calc_phred_score(b"*+"), 19)
    }
}
