use rand::{distr::Alphanumeric, Rng};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

const WORD_MIN: usize = 6;
const WORD_MAX: usize = 8;

fn generate_word() -> String {
    let len = rand::thread_rng().gen_range(WORD_MIN..=WORD_MAX);
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic())
        .take(len)
        .map(char::from)
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <X in GB>", args[0]);
        process::exit(1);
    }

    let gb: f32 = args[1].parse().expect("Invalid number for GB input");
    let bytes_needed: f32 = gb * 1024_f32 * 1024_f32 * 1024_f32;

    let file = File::create("output.txt").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    let mut bytes_written: f32 = 0_f32;

    while bytes_written < bytes_needed {
        let word = generate_word();
        let word_with_space = format!("{} ", word);
        bytes_written += word_with_space.len() as f32;

        writer
            .write_all(word_with_space.as_bytes())
            .expect("Write failed");
    }

    writer.flush().expect("Flush failed");
    println!("Generated {} GB of word data in output.txt", gb);
}
