use bio::io::fastq;
use bio::seq_analysis::{gc, orf};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct SeqResult {
    id: String,
    desc: String,
    gc: f32,
    n_orfs: usize,
    is_valid: bool,
    phred_score: u32,
}

fn analyse_records(records: &Vec<fastq::Record>) -> Vec<SeqResult> {
    let mut results = Vec::new();

    // Iterate over results and find GC content and ORFs
    for rec in records {
        if rec.check().is_ok() {
            let gc_ = gc::gc_content(rec.seq());
            let n_orfs = find_orfs(rec);
            results.push(SeqResult {
                n_orfs,
                id: rec.id().to_owned(),
                desc: rec.desc().unwrap_or("").to_owned(),
                gc: gc_,
                is_valid: rec.check().is_ok(),
                phred_score: calc_phred_score(rec.qual()),
            });
        } else {
            results.push(SeqResult {
                id: "Invalid Record".to_owned(),
                is_valid: rec.check().is_ok(),
                ..Default::default()
            });
        }
    }

    results
}

fn find_orfs(rec: &fastq::Record) -> usize {
    // Hyperparameters for finding open reading frames (ORFs).
    // NB: DNA alphabet
    let start_codons = vec![b"ATG"];
    let stop_codons = vec![b"TGA", b"TAG", b"TAA"];
    let min_len = 50;
    let finder = orf::Finder::new(start_codons, stop_codons, min_len);

    finder.find_all(rec.seq()).count()
}

fn calc_phred_score(qual: &[u8]) -> u32 {
    let mut score = 0;
    for q in qual {
        score += q - 33;
    }
    score.into()
}

#[tauri::command]
pub fn analyse_sequences(sequences: &str) -> Vec<SeqResult> {
    let reader = fastq::Reader::new(sequences.as_bytes());
    let records: Vec<fastq::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_records(&records);

    results
}

#[tauri::command]
pub fn analyse_file(path: &std::path::Path) -> Vec<SeqResult> {
    let reader = fastq::Reader::from_file(path).unwrap();
    let records: Vec<fastq::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_records(&records);

    results
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::analysis::{analyse_file, analyse_sequences, calc_phred_score};

    fn create_test_fq_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        let mut fqs_str: String = "@id description\nATAT\n+\n!!!!\n".to_owned();
        for i in 2..21 {
            fqs_str.push_str(format!("@id{} description\nGCGC\n+\n!!!!\n", i).as_str());
        }

        let mut test_file = std::fs::File::create(path)?;
        test_file.write_all(fqs_str.as_bytes())?;
        Ok(())
    }

    fn remove_test_fq_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        std::fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_analyse_sequences() {
        let mut fqs_str = "@id description\nATAT\n+\n!!!!\n".to_owned();
        fqs_str.push_str("@id description\nGCGC\n+\n!!!!\n");

        let results = analyse_sequences(fqs_str.as_str());
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_missing_sequence() {
        let missing_sequence = "@id description\n\n+\n!!!!\n";

        let results = analyse_sequences(missing_sequence);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }

    #[test]
    fn test_missing_quality() {
        let missing_quality = "@id description\nATAT\n+\n\n";

        let results = analyse_sequences(missing_quality);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }

    #[test]
    fn test_analyse_file() {
        let test_file_name = std::path::Path::new("test_fastq.fq");
        create_test_fq_file(test_file_name);
        let results = analyse_file(test_file_name);
        remove_test_fq_file(test_file_name);
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
    }
}
