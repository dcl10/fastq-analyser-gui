use bio::io::{fasta, fastq};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

/// Size of the buffer between the file (or gzip decoder) and the record parser.
pub const DEFAULT_READ_CAPACITY: usize = 64 * 1024;

/// Opens `path` for reading, decompressing on the fly if it ends in `.gz`.
fn open_maybe_gz(path: &Path) -> io::Result<Box<dyn Read>> {
    let file = File::open(path)?;
    if path.extension().is_some_and(|ext| ext == "gz") {
        Ok(Box::new(GzDecoder::new(file)))
    } else {
        Ok(Box::new(file))
    }
}

pub fn read_fasta_streaming(
    path: &Path,
    capacity: usize,
) -> io::Result<fasta::Reader<BufReader<Box<dyn Read>>>> {
    Ok(fasta::Reader::with_capacity(capacity, open_maybe_gz(path)?))
}

pub fn read_fastq_streaming(
    path: &Path,
    capacity: usize,
) -> io::Result<fastq::Reader<BufReader<Box<dyn Read>>>> {
    Ok(fastq::Reader::with_capacity(capacity, open_maybe_gz(path)?))
}
