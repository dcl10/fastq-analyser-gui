use bio::io::{fasta, fastq};
use bio::seq_analysis::{gc, orf};
use bio::utils::TextSlice;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FastqSeqResult {
    id: String,
    desc: String,
    gc: f32,
    n_orfs: usize,
    is_valid: bool,
    phred_score: u32,
    seq_len: usize,
    result_type: String,
}

impl Default for FastqSeqResult {
    fn default() -> Self {
        FastqSeqResult {
            id: String::from("id"),
            desc: String::from("..."),
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            phred_score: 0,
            seq_len: 0,
            result_type: String::from("fastq"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FastaSeqResult {
    id: String,
    desc: String,
    gc: f32,
    n_orfs: usize,
    is_valid: bool,
    seq_len: usize,
    result_type: String,
}

impl Default for FastaSeqResult {
    fn default() -> Self {
        FastaSeqResult {
            id: String::from("id"),
            desc: String::from("..."),
            gc: 0.0,
            n_orfs: 0,
            is_valid: false,
            seq_len: 0,
            result_type: String::from("fasta"),
        }
    }
}

fn analyse_fastq_records(records: &Vec<fastq::Record>) -> Vec<FastqSeqResult> {
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

fn analyse_fasta_records(records: &Vec<fasta::Record>) -> Vec<FastaSeqResult> {
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

#[tauri::command]
pub fn analyse_fastq_sequences(sequences: &str) -> Vec<FastqSeqResult> {
    let reader = fastq::Reader::new(sequences.as_bytes());
    let records: Vec<fastq::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_fastq_records(&records);

    results
}

#[tauri::command]
pub fn analyse_fastq_file(path: &std::path::Path) -> Vec<FastqSeqResult> {
    let reader = fastq::Reader::from_file(path).unwrap();
    let records: Vec<fastq::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_fastq_records(&records);

    results
}

#[tauri::command]
pub fn analyse_fasta_sequences(sequences: &str) -> Vec<FastaSeqResult> {
    let reader = fasta::Reader::new(sequences.as_bytes());
    let records: Vec<fasta::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_fasta_records(&records);

    results
}

#[tauri::command]
pub fn analyse_fasta_file(path: &std::path::Path) -> Vec<FastaSeqResult> {
    let reader = fasta::Reader::from_file(path).unwrap();
    let records: Vec<fasta::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_fasta_records(&records);

    results
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::analysis::{
        analyse_fasta_file, analyse_fasta_sequences, analyse_fastq_file, analyse_fastq_sequences,
        calc_phred_score,
    };

    fn create_test_fq_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        let mut fqs_str: String = "@id description\nATAT\n+\n!!!!\n".to_owned();
        for i in 2..21 {
            fqs_str.push_str(format!("@id{} description\nGCGC\n+\n!!!!\n", i).as_str());
        }

        let mut test_file = std::fs::File::create(path)?;
        test_file.write_all(fqs_str.as_bytes())?;
        Ok(())
    }

    fn create_test_fa_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        let mut fqs_str: String = ">id description\nATAT\n".to_owned();
        for i in 2..21 {
            fqs_str.push_str(format!(">id{} description\nGCGC\n", i).as_str());
        }

        let mut test_file = std::fs::File::create(path)?;
        test_file.write_all(fqs_str.as_bytes())?;
        Ok(())
    }

    fn remove_test_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        std::fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_analyse_fastq_sequences() {
        let mut fqs_str = "@id description\nATAT\n+\n!!!!\n".to_owned();
        fqs_str.push_str("@id description\nGCGC\n+\n!!!!\n");

        let results = analyse_fastq_sequences(fqs_str.as_str());
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_missing_fq_sequence() {
        let missing_sequence = "@id description\n\n+\n!!!!\n";

        let results = analyse_fastq_sequences(missing_sequence);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }

    #[test]
    fn test_missing_fq_quality() {
        let missing_quality = "@id description\nATAT\n+\n\n";

        let results = analyse_fastq_sequences(missing_quality);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }

    #[test]
    fn test_analyse_fastq_file() {
        let test_file_name = std::path::Path::new("test_fastq.fq");
        create_test_fq_file(test_file_name);
        let results = analyse_fastq_file(test_file_name);
        remove_test_file(test_file_name);
        assert_eq!(results.len(), 20);
        for result in results {
            assert!(result.is_valid)
        }
    }

    #[test]
    fn test_analyse_fasta_sequences() {
        let mut fas_str = ">id description\nATAT\n".to_owned();
        fas_str.push_str(">id description\nGCGC\n");

        let results = analyse_fasta_sequences(fas_str.as_str());
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_missing_fa_sequence() {
        let missing_sequence = ">id description\n";

        let results = analyse_fasta_sequences(missing_sequence);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].seq_len, 0);
        assert!(results[0].is_valid)
    }

    #[test]
    fn test_analyse_fasta_file() {
        let test_file_name = std::path::Path::new("test_fastq.fa");
        create_test_fa_file(test_file_name);
        let results = analyse_fasta_file(test_file_name);
        remove_test_file(test_file_name);
        assert_eq!(results.len(), 20);
        for result in results {
            assert!(result.is_valid)
        }
    }

    #[test]
    fn test_calc_phred_score() {
        let qual_str = b"####";
        assert_eq!(calc_phred_score(qual_str), 8);

        let qual_str = b"!!!!";
        assert_eq!(calc_phred_score(qual_str), 0);

        let mut big_qual_str = String::from("####");
        for i in 0..4999 {
            big_qual_str.push_str("####");
        }
        assert_eq!(calc_phred_score(big_qual_str.as_bytes()), (8 * 5000) as u32)
    }
}
