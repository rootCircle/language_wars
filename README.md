# Code Smackdown (by FOSS Wing, Axios 💚)

## Problem Statement

Develop a highly performant and resource-friendly program that identifies all unique words present across a collection of multiple text files located within a specified directory (`./test_files`). The identified unique words must then be sorted alphabetically and written to a single output file named `result.txt`.

## Task

- Process all text files in the `./test_cases` directory, efficiently counting unique words across all files.
- Optimize execution time, leveraging strategies that allow multiple large files to be processed concurrently rather than sequentially.
- Maintain deterministic output, meaning the result should remain consistent across different runs regardless of variations in processing order.

## Submissions

- Submissions will be done on [Google Form](https://forms.gle/VG4zyWQTFiQtEj4r9).
- 10 min wait time before making a new submission.

## Input Data

- Test files are located in the `./test_cases` directory.
- Each file is a .txt file containing public domain text with varying word distributions.
- Individual input files can be very large, potentially exceeding the available physical RAM and even the combined size of RAM and swap space on the processing system.

## Definition of a Word

- For this challenge, a word is any contiguous sequence of non‐whitespace Unicode characters normalized for comparison, with diacritics considered equivalent
- The maximum length of any word can be safely assumed as 32 bytes.

## Expected Output

The output file `result.txt` should contain all unique words found across the input text files, sorted in alphabetical order. Each word should appear on a new line. The words should be in lowercase to ensure case insensitivity.

## Evaluation Criteria

Your program will be run on a few edge cases before the official tests.

Following parameters will be used for judging:

- **Execution Time** – Faster solutions will receive higher scores.
- **Memory Efficiency (tie breaker)** – Solutions that minimize memory usage will be prioritized.

## Helper Files

The **file_generator** file will create an file containing X MB worth of words. It is suggested that you keep the output file size in the range of 100-200 MB for initial tests.

For later runs, check for files greater than 2-5 GiB; in final legs, files greater than 16 GiB are recommended.

To run:

```bash
./file_generator test_cases/out.txt 2000
```

This command creates a file around 2 GB under the `test_cases` directory. (Expect this to be buggy and slow)

Alternatively, you can use `produce_x` for generating files as well!

```bash
./produce_x 200 # generates file at test_cases directory, of size 200 MiB
```
