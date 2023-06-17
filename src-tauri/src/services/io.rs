use bio::io::{fasta, fastq};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub fn read_fasta(path: &Path) -> fasta::Reader<BufReader<File>> {
    let path_str = path.as_os_str().to_str().unwrap();
    if path_str.ends_with(".gz") {
        let extracted_gz_path = extract_gzip(path);
        return fasta::Reader::from_file(extracted_gz_path).unwrap();
    } else {
        return fasta::Reader::from_file(path).unwrap();
    }
}

pub fn read_fastq(path: &Path) -> fastq::Reader<BufReader<File>> {
    let path_str = path.as_os_str().to_str().unwrap();
    if path_str.ends_with(".gz") {
        let extracted_gz_path = extract_gzip(path);
        return fastq::Reader::from_file(extracted_gz_path).unwrap();
    } else {
        return fastq::Reader::from_file(path).unwrap();
    }
}

pub fn save_results<T>(results: Vec<T>, dest: &Path) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    todo!()
}

fn extract_gzip(path: &Path) -> String {
    let gzip_file = File::open(path).unwrap();
    let mut decoder = GzDecoder::new(gzip_file);
    let mut buf_string = String::new();
    let out_file_path = path.as_os_str().to_str().unwrap().replace(".gz", "");
    let extracted_file = File::create(&out_file_path).unwrap();
    decoder.read_to_string(&mut buf_string).unwrap();
    let mut file_buffer = std::io::BufWriter::new(extracted_file);
    file_buffer.write_all(buf_string.as_bytes()).unwrap();
    out_file_path
}

#[cfg(test)]
mod tests {
    use crate::models::{FastaSeqResult, FastqSeqResult};
    use uuid::Uuid;

    use super::save_results;

    #[test]
    fn test_save_results_saves_fastq_seq_result_to_dest() {
        // Arrange
        let results = Vec::from_iter([FastqSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);

        // Act
        let save_action = save_results(results, save_dest.as_path());

        // Assert
        assert!(save_action.is_ok());
        assert!(save_dest.exists())
    }

    #[test]
    fn test_save_results_saves_fasta_seq_resultt_to_dest() {
        // Arrange
        let results = Vec::from_iter([FastaSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);

        // Act
        let save_action = save_results(results, save_dest.as_path());

        // Assert
        assert!(save_action.is_ok());
        assert!(save_dest.exists())
    }

    #[test]
    fn test_save_results_errors_on_nonexistent_dest() {
        // Arrange
        let results = Vec::from_iter([FastqSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(Uuid::new_v4().to_string() + &save_file);

        // Act
        let save_action = save_results(results, save_dest.as_path());

        // Assert
        assert!(save_action.is_err());
        assert!(!save_dest.exists())
    }
}
