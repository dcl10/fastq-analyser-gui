use bio::io::{fasta, fastq};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub fn read_fasta(path: &Path) -> fasta::Reader<BufReader<File>> {
    if path.ends_with(".gz") {
        let gzip_file = File::open(path).unwrap();
        let decoder = GzDecoder::new(gzip_file);
        return fasta::Reader::new(decoder.into_inner());
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
