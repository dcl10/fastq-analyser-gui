use std::fs::File;
use std::io::BufReader;
use bio::io::{fasta, fastq};
use flate2::read::{GzDecoder};

pub fn read_fasta(path: &std::path::Path) -> fasta::Reader<BufReader<File>>{
  if path.ends_with(".gz") {
      let gzip_file = File::open(path).unwrap();
      let decoder = GzDecoder::new(gzip_file);
      return fasta::Reader::new(decoder.into_inner());
  } else {
      return fasta::Reader::from_file(path).unwrap()
  }
}

pub fn read_fastq(path: &std::path::Path) -> fastq::Reader<BufReader<File>>{
  if path.ends_with(".gz") {
      let gzip_file = File::open(path).unwrap();
      let decoder = GzDecoder::new(gzip_file);
      return fastq::Reader::new(decoder.into_inner());
  } else {
      return fastq::Reader::from_file(path).unwrap()
  }
}