use crate::analysis::analysers::{analyse_fasta_records, analyse_fastq_records};
use crate::io::{read_fasta, read_fastq};
use crate::models::{FastaSeqResult, FastqSeqResult};
use bio::io::{fasta, fastq};

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
    let reader = read_fastq(path);
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
    let reader = read_fasta(path);
    let records: Vec<fasta::Record> = reader
        .records()
        .map(|rec| rec.unwrap_or_default())
        .collect();

    let results = analyse_fasta_records(&records);

    results
}

#[cfg(test)]
mod tests {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;

    use crate::analysis::commands::{
        analyse_fasta_file, analyse_fasta_sequences, analyse_fastq_file, analyse_fastq_sequences,
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

    fn create_test_fqgz_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        let mut fqs_str: String = "@id description\nATAT\n+\n!!!!\n".to_owned();
        for i in 2..21 {
            fqs_str.push_str(format!("@id{} description\nGCGC\n+\n!!!!\n", i).as_str());
        }

        let test_file = std::fs::File::create(path)?;
        let mut encoder = GzEncoder::new(test_file, Compression::default());
        encoder.write_all(fqs_str.as_bytes())?;
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
        assert!(create_test_fq_file(test_file_name).is_ok());
        let results = analyse_fastq_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert_eq!(results.len(), 20);
        for result in results {
            assert!(result.is_valid)
        }
    }

    #[test]
    fn test_analyse_fastq_file_zipped() {
        let test_file_name = std::path::Path::new("test_fastq.fq.gz");
        let test_file_unpacked = std::path::Path::new("test_fastq.fq");
        assert!(create_test_fqgz_file(test_file_name).is_ok());
        let results = analyse_fastq_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        if test_file_unpacked.exists() {
            assert!(remove_test_file(test_file_unpacked).is_ok());
        }
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
        assert!(create_test_fa_file(test_file_name).is_ok());
        let results = analyse_fasta_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert_eq!(results.len(), 20);
        for result in results {
            assert!(result.is_valid)
        }
    }
}
