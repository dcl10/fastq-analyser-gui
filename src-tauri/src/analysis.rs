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
}

#[tauri::command]
pub fn analyse_sequences(sequences: String) -> Vec<SeqResult> {
    let mut results = Vec::new();
    let reader = fastq::Reader::new(sequences.as_bytes());
    let records = reader.records().map(|rec| rec.unwrap_or_default());

    // Hyperparameters for finding open reading frames (ORFs).
    // NB: DNA alphabet
    let start_codons = vec![b"ATG"];
    let stop_codons = vec![b"TGA", b"TAG", b"TAA"];
    let min_len = 50;
    let finder = orf::Finder::new(start_codons, stop_codons, min_len);

    // Iterate over results and find GC content and ORFs
    for rec in records {
        if rec.check().is_ok() {
            let gc_ = gc::gc_content(rec.seq());
            let n_orfs = finder.find_all(rec.seq()).count();
            results.push(SeqResult {
                n_orfs,
                id: rec.id().to_owned(),
                desc: rec.desc().unwrap_or("").to_owned(),
                gc: gc_,
                is_valid: rec.check().is_ok(),
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

#[tauri::command]
fn analyse_file(path_to_file: std::path::Path) -> Vec<SeqResult> {}

#[cfg(test)]
mod tests {
    use crate::analysis::analyse_sequences;

    #[test]
    fn test_analyse_sequences() {
        let mut fqs_str: String = "@id description\nATAT\n+\n!!!!\n".to_owned();
        fqs_str.push_str("@id description\nGCGC\n+\n!!!!\n");

        let results = analyse_sequences(fqs_str);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_missing_sequence() {
        let missing_sequence: String = "@id description\n\n+\n!!!!\n".to_owned();

        let results = analyse_sequences(missing_sequence);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }

    #[test]
    fn test_missing_quality() {
        let missing_quality: String = "@id description\nATAT\n+\n\n".to_owned();

        let results = analyse_sequences(missing_quality);
        assert_eq!(results.len(), 1);
        assert!(!results[0].is_valid);
    }
}
