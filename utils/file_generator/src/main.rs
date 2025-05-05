use rand::{rng, Rng};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::time::Instant;

// Default target size set to 20 GiB
const DEFAULT_TARGET_SIZE_BYTES: u64 = 20 * 1024 * 1024 * 1024;
const WORD_LEN: usize = 5;

// Define a chunk size for generating data before writing to BufWriter.
// A larger chunk size generally leads to fewer write calls, improving performance.
const CHUNK_BUFFER_SIZE: usize = 64 * 1024 * 1024; // 64MB chunk buffer

// Size of the buffer for raw random bytes used for word generation.
// Processing random bytes in blocks might be faster than sampling one by one.
const RAW_RANDOM_BUFFER_SIZE: usize = 8 * 1024; // 4KB buffer for raw random bytes

// Define the set of characters we want to use for words (alphanumeric).
// This is used for mapping random bytes to characters.
const ALPHANUMERIC_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const ALPHANNUMERIC_LEN: usize = ALPHANUMERIC_CHARS.len();

fn main() -> Result<(), Error> {
    // Get the output filename and target size from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_filename> [<target_size_in_mb>]", args[0]);
        std::process::exit(1);
    }

    let output_filename = &args[1];
    let target_size_mb: u64 = if args.len() > 2 {
        args[2].parse().unwrap_or_else(|_| {
            eprintln!(
                "Invalid target size. Using default size of {} MiB.",
                DEFAULT_TARGET_SIZE_BYTES / (1024 * 1024)
            );
            DEFAULT_TARGET_SIZE_BYTES
        })
    } else {
        DEFAULT_TARGET_SIZE_BYTES
    };

    let target_size_bytes = target_size_mb * 1024 * 1024;

    println!("Starting optimized file generation ...");
    println!("Filename: {} Size: {} MiB", output_filename, target_size_mb);

    let start_time = Instant::now();
    let file = File::create(output_filename)?;
    let mut writer = BufWriter::with_capacity(CHUNK_BUFFER_SIZE, file);

    let mut bytes_written: u64 = 0;
    let mut rng = rng();

    // Create a reusable buffer to build chunks of data in memory before writing.
    let mut chunk_buffer: Vec<u8> = Vec::with_capacity(CHUNK_BUFFER_SIZE);

    // Create a buffer to hold raw random bytes that we will process to form words.
    let mut raw_random_buffer = [0u8; RAW_RANDOM_BUFFER_SIZE];
    let mut raw_random_buffer_pos = RAW_RANDOM_BUFFER_SIZE; // Start at the end to trigger initial fill

    // Calculate the required length for a word plus a newline, which is now constant.
    const REQUIRED_LEN_FOR_WORD: usize = WORD_LEN + 1; // Word length + 1 byte for newline

    // Outer loop: Continue until the target file size is reached or exceeded.
    while bytes_written < target_size_bytes {
        // Clear the chunk buffer to reuse its allocated memory for the next chunk.
        chunk_buffer.clear();

        // --- Inner Loop: Fill the chunk buffer ---
        while chunk_buffer.len() + REQUIRED_LEN_FOR_WORD <= CHUNK_BUFFER_SIZE {
            if raw_random_buffer_pos + WORD_LEN > RAW_RANDOM_BUFFER_SIZE {
                rng.fill(&mut raw_random_buffer);
                raw_random_buffer_pos = 0;
            }

            if bytes_written + chunk_buffer.len() as u64 + REQUIRED_LEN_FOR_WORD as u64
                > target_size_bytes
            {
                break; // Stop filling the current chunk
            }

            for _ in 0..WORD_LEN {
                let char_byte = ALPHANUMERIC_CHARS
                    [raw_random_buffer[raw_random_buffer_pos] as usize % ALPHANNUMERIC_LEN];
                chunk_buffer.push(char_byte);
                raw_random_buffer_pos += 1;
            }

            chunk_buffer.push(b'\n');
        }

        // --- Handling the Loop Termination Condition ---
        if chunk_buffer.is_empty() {
            if bytes_written >= target_size_bytes {
                break; // Target reached, exit outer loop
            }
            if target_size_bytes - bytes_written < REQUIRED_LEN_FOR_WORD as u64 {
                break; // Remaining space is too small for another word
            }
            eprintln!(
                "Warning: Chunk buffer is empty, but target size not reached. Breaking loop."
            );
            break;
        }

        writer.write_all(&chunk_buffer)?;
        bytes_written += chunk_buffer.len() as u64;
    }

    writer.flush()?;

    let duration = start_time.elapsed();
    let file_size_gb = bytes_written as f64 / (1024.0 * 1024.0 * 1024.0);

    println!(
        "\nSuccessfully generated '{}' ({:.2} GB) in {:.2?}.",
        output_filename, file_size_gb, duration
    );

    Ok(())
}
