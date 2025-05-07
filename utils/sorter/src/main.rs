use memchr::memchr_iter;
use memmap2::Mmap;
use rayon::prelude::*;
use rustc_hash::{FxHashSet, FxHasher};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, RwLock};

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

const PARTITIONS: usize = 256;
const CHUNK_SIZE: usize = 256 * 1024 * 1024; // 256 MB
const MAX_WORD_LEN: usize = 32; // overlap window size

#[inline]
fn hash_partition(word: &str) -> usize {
    //let mut hasher = DefaultHasher::new();
    let mut hasher = FxHasher::default();
    hasher.write(word.as_bytes());
    (hasher.finish() as usize) % PARTITIONS
}

fn main() -> std::io::Result<()> {
    let input_dir = "test_cases";
    let partition_dir = "partitions";
    let output_file = "result.txt";
    let tmp_output = format!("{}.tmp", output_file);

    ctrlc::set_handler(move || {
        let _ = fs::remove_dir_all(partition_dir);
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // 1) Create partitions directory
    fs::create_dir_all(partition_dir)?;

    // 2) Partition with prefix+suffix overlap
    {
        let writers: Vec<_> = (0..PARTITIONS)
            .map(|i| {
                Arc::new(RwLock::new(BufWriter::new(
                    File::create(format!("{}/partition_{}.txt", partition_dir, i)).unwrap(),
                )))
            })
            .collect();

        let files: Vec<_> = fs::read_dir(input_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file() && p.extension().map(|ext| ext == "txt").unwrap_or(false))
            .collect();

        files.par_iter().for_each(|path| {
            if let Ok(metadata) = fs::metadata(path) {
                let file_size = metadata.len() as usize;
                let num_chunks = file_size.div_ceil(CHUNK_SIZE);

                (0..num_chunks).into_par_iter().for_each(|chunk_idx| {
                    if let Ok(file) = File::open(path) {
                        if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                            let file_size = mmap.len();
                            let start = chunk_idx * CHUNK_SIZE;

                            let prefix = start.min(MAX_WORD_LEN);
                            let read_start = start - prefix;
                            let read_end = (start + CHUNK_SIZE + MAX_WORD_LEN).min(file_size);

                            if read_end > read_start {
                                let chunk = &mmap[read_start..read_end];
                                let valid_end = match std::str::from_utf8(chunk) {
                                    Ok(_) => chunk.len(),
                                    Err(e) => e.valid_up_to(),
                                };
                                let content = String::from_utf8_lossy(&chunk[..valid_end]);

                                let mut begin = 0;
                                if prefix > 0 {
                                    if let Some(rel) = content[..prefix].rfind(char::is_whitespace)
                                    {
                                        begin = rel + 1;
                                    }
                                }

                                let mut end = content.len();
                                if start + CHUNK_SIZE < file_size {
                                    let boundary = prefix + CHUNK_SIZE;
                                    if boundary < content.len() {
                                        if let Some(rel) =
                                            content[boundary..].find(char::is_whitespace)
                                        {
                                            end = boundary + rel;
                                        }
                                    }
                                }

                                for line in content[begin..end].lines() {
                                    for word in simd_split_ascii_whitespace(line) {
                                        let mut word_bytes = word.as_bytes().to_vec();
                                        to_ascii_lowercase_simd(&mut word_bytes);
                                        let w = String::from_utf8_lossy(&word_bytes);
                                        let idx = hash_partition(&w);
                                        if let Ok(mut wr) = writers[idx].write() {
                                            let _ = writeln!(wr, "{}", w);
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        });
        drop(writers);
    }

    // 3) Deduplicate & write .bin
    (0..PARTITIONS).into_par_iter().for_each(|i| {
        let mut set = FxHashSet::default();
        let txt_path = format!("{}/partition_{}.txt", partition_dir, i);
        if let Ok(file) = File::open(&txt_path) {
            for line in BufReader::new(file).lines().map_while(Result::ok) {
                set.insert(line);
            }
        }
        let mut sorted: Vec<_> = set.drain().collect();
        sorted.sort_unstable();

        let bin_path = format!("{}/partition_{}.bin", partition_dir, i);
        let mut out = BufWriter::new(File::create(&bin_path).unwrap());
        for word in &sorted {
            let b = word.as_bytes();
            out.write_all(&(b.len() as u32).to_le_bytes()).unwrap();
            out.write_all(b).unwrap();
        }
    });

    // 4) Multi-way merge into tmp_output

    {
        let mut heap = BinaryHeap::new();
        let mut readers: Vec<Option<(Mmap, usize)>> = Vec::with_capacity(PARTITIONS);
        readers.resize_with(PARTITIONS, || None);

        let mut out = BufWriter::new(File::create(&tmp_output)?);
        let mut last: Option<String> = None;
        for (i, reader) in readers.iter_mut().enumerate().take(PARTITIONS) {
            let bin_path = format!("{}/partition_{}.bin", partition_dir, i);
            if let Ok(file) = File::open(&bin_path) {
                if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                    let mut pos = 0;
                    if let Some(word) = read_next_word_mmap(&mmap, &mut pos) {
                        heap.push(Reverse((word.clone(), i)));
                        *reader = Some((mmap, pos));
                    }
                }
            }
        }

        while let Some(Reverse((word, i))) = heap.pop() {
            if last.as_ref() != Some(&word) {
                writeln!(out, "{}", word)?;
                last = Some(word.clone());
            }

            if let Some((ref mmap, ref mut pos)) = readers[i] {
                if let Some(next_word) = read_next_word_mmap(mmap, pos) {
                    heap.push(Reverse((next_word, i)));
                } else {
                    readers[i] = None;
                }
            }
        }

        out.flush()?;
    }

    // 5) Rename temp result into final
    fs::rename(&tmp_output, output_file)?;

    // 6) Cleanup
    let _ = fs::remove_dir_all(partition_dir);

    Ok(())
}

//#[inline]
//fn read_next_word<R: Read>(reader: &mut R) -> std::io::Result<Option<String>> {
//    let mut len_buf = [0u8; 4];
//    match reader.read_exact(&mut len_buf) {
//        Ok(_) => {
//            let len = u32::from_le_bytes(len_buf) as usize;
//            let mut buf = vec![0u8; len];
//            reader.read_exact(&mut buf)?;
//            Ok(Some(String::from_utf8_lossy(&buf).into_owned()))
//        }
//        Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(None),
//        Err(e) => Err(e),
//    }
//}

/// Splits ASCII string on whitespace using SIMD-accelerated byte search.
/// Only splits on space, tab, and newline for simplicity.
fn simd_split_ascii_whitespace(s: &str) -> impl Iterator<Item = &str> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    // 1) collect all whitespace positions (space, newline, tab)
    let mut positions: Vec<usize> = memchr_iter(b' ', bytes)
        .chain(memchr_iter(b'\n', bytes))
        .chain(memchr_iter(b'\t', bytes))
        .collect();

    // 2) sort them so we split in true left‑to‑right order
    positions.sort_unstable();

    // 3) add end‑of‑string as final split point
    positions.push(len);

    // 4) now slice out each word in order
    let mut start = 0;
    positions.into_iter().filter_map(move |i| {
        if start < i {
            let word = &s[start..i];
            start = i + 1;
            Some(word)
        } else {
            // consecutive whitespace or zero‑length – skip
            start = i + 1;
            None
        }
    })
}

/// Convert ASCII bytes to lowercase in-place.
#[inline]
fn to_ascii_lowercase_simd(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        if b.is_ascii_uppercase() {
            *b += 32; // ASCII difference between uppercase and lowercase
        }
    }
}

/// Reads the next word from an mmap'd `.bin` file based on position offset.
fn read_next_word_mmap(mmap: &[u8], pos: &mut usize) -> Option<String> {
    if *pos + 4 > mmap.len() {
        return None;
    }
    let len = u32::from_le_bytes(mmap[*pos..*pos + 4].try_into().unwrap()) as usize;
    *pos += 4;
    if *pos + len > mmap.len() {
        return None;
    }
    let word = String::from_utf8_lossy(&mmap[*pos..*pos + len]).into_owned();
    *pos += len;
    Some(word)
}
