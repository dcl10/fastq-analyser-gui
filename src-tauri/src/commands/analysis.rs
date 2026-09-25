use crate::analysis::analysers::{analyse_fasta_records, analyse_fastq_records};
use crate::models::{FastaSeqResult, FastqSeqResult};
use crate::services::io::{read_fasta, read_fastq};
use bio::io::{fasta, fastq};

#[tauri::command(async)]
pub fn analyse_fastq_sequences(sequences: &str) -> Result<Vec<FastqSeqResult>, String> {
    let reader = fastq::Reader::new(sequences.as_bytes());
    let records: Vec<fastq::Record> = reader
        .records()
        .enumerate()
        .map(|(i, rec)| rec.map_err(|e| format!("Record {}: {e}", i + 1)))
        .collect::<Result<_, _>>()?;

    let results = analyse_fastq_records(&records)?;

    Ok(results)
}

#[tauri::command(async)]
pub fn analyse_fastq_file(path: &std::path::Path) -> Result<Vec<FastqSeqResult>, String> {
    let reader = read_fastq(path);
    let records: Vec<fastq::Record> = reader
        .records()
        .enumerate()
        .map(|(i, rec)| rec.map_err(|e| format!("Record {}: {e}", i + 1)))
        .collect::<Result<_, _>>()?;

    let path_str = path.to_str().unwrap();
    if path_str.ends_with(".gz") {
        let unpacked = path_str.replace(".gz", "");
        match std::fs::remove_file(unpacked) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
    let results = analyse_fastq_records(&records)?;

    Ok(results)
}

#[tauri::command(async)]
pub fn analyse_fasta_sequences(sequences: &str) -> Result<Vec<FastaSeqResult>, String> {
    let reader = fasta::Reader::new(sequences.as_bytes());
    let records: Vec<fasta::Record> = reader
        .records()
        .enumerate()
        .map(|(i, rec)| rec.map_err(|e| format!("Record {}: {e}", i + 1)))
        .collect::<Result<_, _>>()?;

    let results = analyse_fasta_records(&records)?;

    Ok(results)
}

#[tauri::command(async)]
pub fn analyse_fasta_file(path: &std::path::Path) -> Result<Vec<FastaSeqResult>, String> {
    let reader = read_fasta(path);
    let records: Vec<fasta::Record> = reader
        .records()
        .enumerate()
        .map(|(i, rec)| rec.map_err(|e| format!("Record {}: {e}", i + 1)))
        .collect::<Result<_, _>>()?;

    let path_str = path.to_str().unwrap();
    if path_str.ends_with(".gz") {
        let unpacked = path_str.replace(".gz", "");
        match std::fs::remove_file(unpacked) {
            Ok(_) => (),
            Err(_) => (),
        }
    }
    let results = analyse_fasta_records(&records)?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    use uuid::Uuid;

    use crate::commands::analysis::{
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

    fn create_test_fagz_file<'a>(path: &'a std::path::Path) -> std::io::Result<()> {
        let mut fqs_str: String = ">id description\nATAT\n".to_owned();
        for i in 2..21 {
            fqs_str.push_str(format!(">id{} description\nGCGC\n", i).as_str());
        }

        let test_file = std::fs::File::create(path)?;
        let mut encoder = GzEncoder::new(test_file, Compression::default());
        encoder.write_all(fqs_str.as_bytes())?;
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
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 2);
    }

    #[test]
    fn test_missing_fq_sequence() {
        let missing_sequence = "@id description\n\n+\n!!!!\n";

        let results = analyse_fastq_sequences(missing_sequence);
        assert!(results.is_err());
    }

    #[test]
    fn test_missing_fq_quality() {
        let missing_quality = "@id description\nATAT\n+\n\n";

        let results = analyse_fastq_sequences(missing_quality);
        assert!(results.is_err());
    }

    #[test]
    fn test_analyse_fastq_file() {
        // Unique per test run so parallel tests can't collide on the same file.
        let file_name = format!("test_fastq_{}.fq", Uuid::new_v4());
        let test_file_name = std::path::Path::new(&file_name);
        assert!(create_test_fq_file(test_file_name).is_ok());
        let results = analyse_fastq_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 20);
    }

    #[test]
    fn test_analyse_fastq_file_zipped() {
        let file_name = format!("test_fastq_{}.fq.gz", Uuid::new_v4());
        let unpacked_file_name = file_name.replace(".gz", "");
        let test_file_name = std::path::Path::new(&file_name);
        let test_file_unpacked = std::path::Path::new(&unpacked_file_name);
        assert!(create_test_fqgz_file(test_file_name).is_ok());
        let results = analyse_fastq_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert!(!test_file_unpacked.exists());
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 20);
    }

    #[test]
    fn test_analyse_fasta_sequences() {
        let mut fas_str = ">id description\nATAT\n".to_owned();
        fas_str.push_str(">id description\nGCGC\n");

        let results = analyse_fasta_sequences(fas_str.as_str());
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 2);
    }

    #[test]
    fn test_missing_fa_sequence() {
        let missing_sequence = ">id description\n";

        let results = analyse_fasta_sequences(missing_sequence);
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].seq_len, 0);
    }

    #[test]
    fn test_analyse_fasta_file() {
        let file_name = format!("test_fasta_{}.fa", Uuid::new_v4());
        let test_file_name = std::path::Path::new(&file_name);
        assert!(create_test_fa_file(test_file_name).is_ok());
        let results = analyse_fasta_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 20);
    }

    #[test]
    fn test_analyse_fasta_file_zipped() {
        let file_name = format!("test_fasta_{}.fa.gz", Uuid::new_v4());
        let unpacked_file_name = file_name.replace(".gz", "");
        let test_file_name = std::path::Path::new(&file_name);
        let test_file_unpacked = std::path::Path::new(&unpacked_file_name);
        assert!(create_test_fagz_file(test_file_name).is_ok());
        let results = analyse_fasta_file(test_file_name);
        assert!(remove_test_file(test_file_name).is_ok());
        assert!(!test_file_unpacked.exists());
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 20);
    }
}
