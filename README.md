# Concurrent File Processing

## Problem Statement:
In this challenge, you are tasked with designing a highly efficient program to process massive text datasets while minimizing execution time and memory usage. The input consists of multiple large text files, whose combined size exceeds 20GB, far beyond what can be stored in RAM at once. Traditional in-memory processing is not feasible, making it imperative to develop a solution that efficiently handles streaming input while ensuring correctness.

Your program must be optimized for scale, processing large number of words across multiple files, all while ensuring consistent results and optimal resource utilization. A well-crafted solution will not only process text efficiently but also intelligently distribute workloads, ensuring minimal bottlenecks during execution.

## Task
- Process all text files in the `/test_cases` directory, efficiently counting unique words across all files.
- Minimize memory usage, ensuring that file contents are not fully loaded into RAM.
- Optimize execution time, leveraging strategies that allow multiple large files to be processed concurrently rather than sequentially.
- Maintain deterministic output, meaning the result should remain consistent across different runs regardless of variations in processing order.
- No external libraries or commands should be used


## Input Data:
- Test files are located in the /test_cases directory.
- Each file is a .txt file containing public domain text with varying word distributions.
- The combined dataset exceeds 20GB, making naive full-memory processing impossible.

## Definition of a Word
For this challenge, a word is any contiguous sequence of non‐whitespace Unicode characters normalized for comparison, with diacritics considered equivalent

## Expected Output
The program must output a single integer—the total count of unique words across all processed files.

## Evaluation Criteria
- Execution Time – Faster solutions will receive higher scores.
- Memory Efficiency – Solutions that minimize memory usage will be prioritized.
- Correctness – The output must correctly represent the total unique word count.

## Performance Considerations
- Processing files sequentially is likely to be slow and inefficient due to sheer data volume.
- Well-structured solutions will ensure continuous data flow without blocking execution, avoiding unnecessary disk I/O overhead.
- A high-performance approach will account for concurrent processing, allowing multiple large files to be handled in tandem to reduce execution time.
