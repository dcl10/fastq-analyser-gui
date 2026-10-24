use bio::io::{fasta, fastq};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Size of the buffer between the file (or gzip decoder) and the record parser.
pub const DEFAULT_READ_CAPACITY: usize = 64 * 1024;

/// Wraps a reader and adds every byte read through it to a shared counter, so another thread can see how far through
/// a file the reader has got.
pub struct CountingReader<R> {
    inner: R,
    bytes_read: Arc<AtomicU64>,
}

impl<R: Read> Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.bytes_read.fetch_add(n as u64, Ordering::Relaxed);
        Ok(n)
    }
}

/// Opens `path` for reading, decompressing on the fly if it ends in `.gz`.
fn open_maybe_gz(path: &Path) -> io::Result<Box<dyn Read>> {
    Ok(maybe_gz(path, File::open(path)?))
}

/// Opens `path` like `open_maybe_gz`, adding the bytes read from the file itself (compressed bytes for `.gz`) to
/// `bytes_read`.
fn open_maybe_gz_counted(path: &Path, bytes_read: Arc<AtomicU64>) -> io::Result<Box<dyn Read>> {
    let file = CountingReader { inner: File::open(path)?, bytes_read };
    Ok(maybe_gz(path, file))
}

fn maybe_gz<R: Read + 'static>(path: &Path, inner: R) -> Box<dyn Read> {
    if path.extension().is_some_and(|ext| ext == "gz") {
        Box::new(GzDecoder::new(inner))
    } else {
        Box::new(inner)
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

/// Like `read_fasta_streaming`, adding the bytes read from the file to `bytes_read` for progress reporting.
pub fn read_fasta_streaming_counted(
    path: &Path,
    capacity: usize,
    bytes_read: Arc<AtomicU64>,
) -> io::Result<fasta::Reader<BufReader<Box<dyn Read>>>> {
    Ok(fasta::Reader::with_capacity(capacity, open_maybe_gz_counted(path, bytes_read)?))
}

/// Like `read_fastq_streaming`, adding the bytes read from the file to `bytes_read` for progress reporting.
pub fn read_fastq_streaming_counted(
    path: &Path,
    capacity: usize,
    bytes_read: Arc<AtomicU64>,
) -> io::Result<fastq::Reader<BufReader<Box<dyn Read>>>> {
    Ok(fastq::Reader::with_capacity(capacity, open_maybe_gz_counted(path, bytes_read)?))
}

#[cfg(test)]
mod tests {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::Write;
    use std::path::Path;
    use uuid::Uuid;

    use super::{read_fasta_streaming, read_fastq_streaming, DEFAULT_READ_CAPACITY};

    const FASTA: &str = ">id1 description\nATAT\n>id2\nGCGC\n";
    const FASTQ: &str = "@id1 description\nATAT\n+\n!!!!\n@id2\nGCGC\n+\n!!!!\n";

    fn write_file(path: &Path, contents: &str, gzip: bool) {
        let file = std::fs::File::create(path).unwrap();
        if gzip {
            let mut encoder = GzEncoder::new(file, Compression::default());
            encoder.write_all(contents.as_bytes()).unwrap();
            encoder.finish().unwrap();
        } else {
            let mut file = file;
            file.write_all(contents.as_bytes()).unwrap();
        }
    }

    fn fasta_ids(path: &Path) -> Vec<String> {
        read_fasta_streaming(path, DEFAULT_READ_CAPACITY)
            .unwrap()
            .records()
            .map(|rec| rec.unwrap().id().to_owned())
            .collect()
    }

    fn fastq_ids(path: &Path) -> Vec<String> {
        read_fastq_streaming(path, DEFAULT_READ_CAPACITY)
            .unwrap()
            .records()
            .map(|rec| rec.unwrap().id().to_owned())
            .collect()
    }

    #[test]
    fn test_read_fasta_streaming_plain_and_gz() {
        let plain = format!("test_io_{}.fa", Uuid::new_v4());
        let gz = format!("{plain}.gz");
        write_file(Path::new(&plain), FASTA, false);
        write_file(Path::new(&gz), FASTA, true);

        let plain_ids = fasta_ids(Path::new(&plain));
        let gz_ids = fasta_ids(Path::new(&gz));
        std::fs::remove_file(&plain).unwrap();
        std::fs::remove_file(&gz).unwrap();

        assert_eq!(plain_ids, vec!["id1", "id2"]);
        assert_eq!(gz_ids, plain_ids);
    }

    #[test]
    fn test_read_fastq_streaming_plain_and_gz() {
        let plain = format!("test_io_{}.fq", Uuid::new_v4());
        let gz = format!("{plain}.gz");
        write_file(Path::new(&plain), FASTQ, false);
        write_file(Path::new(&gz), FASTQ, true);

        let plain_ids = fastq_ids(Path::new(&plain));
        let gz_ids = fastq_ids(Path::new(&gz));
        std::fs::remove_file(&plain).unwrap();
        std::fs::remove_file(&gz).unwrap();

        assert_eq!(plain_ids, vec!["id1", "id2"]);
        assert_eq!(gz_ids, plain_ids);
    }

    #[test]
    fn test_read_gz_leaves_no_unpacked_file() {
        let gz = format!("test_io_{}.fa.gz", Uuid::new_v4());
        write_file(Path::new(&gz), FASTA, true);

        let _ = fasta_ids(Path::new(&gz));
        std::fs::remove_file(&gz).unwrap();

        assert!(!Path::new(&gz.replace(".gz", "")).exists());
    }

    #[test]
    fn test_read_streaming_missing_file() {
        let missing = format!("test_io_{}.fa", Uuid::new_v4());
        assert!(read_fasta_streaming(Path::new(&missing), DEFAULT_READ_CAPACITY).is_err());
        assert!(read_fastq_streaming(Path::new(&missing), DEFAULT_READ_CAPACITY).is_err());
    }
}
