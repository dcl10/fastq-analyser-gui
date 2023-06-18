use bio::io::{fasta, fastq};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::ErrorKind::{InvalidData, InvalidInput, NotFound};
use std::io::{BufReader, BufWriter, Read, Write};
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

pub fn save_results<T>(results: &Vec<T>, dest: &Path) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let serialised_results = serde_json::to_string_pretty(results);
    if serialised_results.is_err() {
        return Err(std::io::Error::new(InvalidData, "Could not save results."));
    }
    // binding prevents "drop of value while in use" warning
    let binding = String::from(serialised_results.unwrap());
    let results_as_bytes = binding.as_bytes();

    let output_file = File::create(dest);

    match output_file {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            return writer.write_all(results_as_bytes);
        }
        Err(error) => return Err(error),
    }
}

pub fn load_results<'a, T>(source: &Path) -> Result<Vec<T>, std::io::Error>
where
    T: Deserialize<'a>,
{
    if !source.exists() {
        return Err(std::io::Error::new(
            NotFound,
            format!("{} does not exist.", source.display()),
        ));
    }

    if source.is_dir() {
        return Err(std::io::Error::new(
            InvalidInput,
            format!("{} is a directory.", source.display()),
        ));
    }

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
    use std::fs::File;

    use crate::models::{FastaSeqResult, FastqSeqResult};
    use uuid::Uuid;

    use super::{load_results, save_results};

    #[test]
    fn test_save_results_saves_fastq_seq_result_to_dest() {
        // Arrange
        let results = Vec::from_iter([FastqSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);
        println!("{}", save_dest.to_str().unwrap());

        // Act
        let save_action = save_results(&results, save_dest.as_path());

        // Assert
        assert!(save_action.is_ok());
        assert!(save_dest.exists());

        // Clean up
        assert!(std::fs::remove_file(save_dest).is_ok());
    }

    #[test]
    fn test_save_results_saves_fasta_seq_resultt_to_dest() {
        // Arrange
        let results = Vec::from_iter([FastaSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);

        // Act
        let save_action = save_results(&results, save_dest.as_path());

        // Assert
        assert!(save_action.is_ok());
        assert!(save_dest.exists());

        // Clean up
        assert!(std::fs::remove_file(save_dest).is_ok());
    }

    #[test]
    fn test_save_results_errors_on_nonexistent_dest() {
        // Arrange
        let results = Vec::from_iter([FastqSeqResult::default()]);
        let save_dir = tauri::api::path::desktop_dir()
            .unwrap()
            .join(Uuid::new_v4().to_string());
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);

        // Act
        let save_action = save_results(&results, save_dest.as_path());

        // Assert
        assert!(save_action.is_err());
        assert!(!save_dest.exists());

        // Clean up
        assert!(std::fs::remove_file(save_dest).is_err());
    }

    #[test]
    fn test_load_results_loads_vec_of_fastq_seq_results() {
        // Arrange
        let expected_results = vec![
            FastqSeqResult::default(),
            FastqSeqResult::default(),
            FastqSeqResult::default(),
        ];
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);
        let writer = File::create(save_dest.as_path()).unwrap();
        serde_json::to_writer_pretty(writer, &expected_results)
            .expect("Couldn't save results for the test.");

        // Act
        let results = load_results::<FastqSeqResult>(save_dest.as_path());

        // Assert
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(expected_results.len(), results.len());
        for i in 0..expected_results.len() {
            assert_eq!(expected_results[i], results[i]);
        }

        // Clean up
        assert!(std::fs::remove_file(save_dest).is_ok());
    }

    #[test]
    fn test_load_results_loads_vec_of_fasta_seq_results() {
        // Arrange
        let expected_results = vec![
            FastaSeqResult::default(),
            FastaSeqResult::default(),
            FastaSeqResult::default(),
        ];
        let save_dir = tauri::api::path::desktop_dir().unwrap();
        let save_file = Uuid::new_v4().to_string() + ".json";
        let save_dest = save_dir.join(save_file);
        let writer = File::create(save_dest.as_path()).unwrap();
        serde_json::to_writer_pretty(writer, &expected_results)
            .expect("Couldn't save results for the test.");

        // Act
        let results = load_results::<FastaSeqResult>(save_dest.as_path());

        // Assert
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(expected_results.len(), results.len());
        for i in 0..expected_results.len() {
            assert_eq!(expected_results[i], results[i]);
        }

        // Clean up
        assert!(std::fs::remove_file(save_dest).is_ok());
    }

    #[test]
    fn test_load_results_errors_on_nonexistent_results_file() {
        // Arrange
        let results_dir = tauri::api::path::desktop_dir().unwrap();
        let results_file = results_dir.join(Uuid::new_v4().to_string() + ".json");

        // Act
        let result_action = load_results::<FastqSeqResult>(results_file.as_path());

        // Assert
        assert!(result_action.is_err())
    }

    #[test]
    fn test_load_results_errors_on_source_is_dir() {
        // Arrange
        let results_dir = tauri::api::path::desktop_dir().unwrap();

        // Act
        let result_action = load_results::<FastqSeqResult>(results_dir.as_path());

        // Assert
        assert!(result_action.is_err())
    }
}
