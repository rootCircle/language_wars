# Concurrent File Processing

## Problem Statement:
In this challenge, you are tasked with designing a highly efficient program to process massive text datasets while minimizing execution time and memory usage. The input consists of multiple large text files.Traditional in-memory processing is not feasible, making it imperative to develop a solution that efficiently handles streaming input while ensuring correctness.

Your program must be optimized for scale, processing large number of words across multiple files, all while ensuring consistent results and optimal resource utilization. A well-crafted solution will not only process text efficiently but also intelligently distribute workloads, ensuring minimal bottlenecks during execution.

## Task
- Process all text files in the `/test_cases` directory, efficiently counting unique words across all files.
- Optimize execution time, leveraging strategies that allow multiple large files to be processed concurrently rather than sequentially.
- Maintain deterministic output, meaning the result should remain consistent across different runs regardless of variations in processing order.
- No external libraries or commands should be used

## Constraint
10 min wait time for making a new submission

## Input Data:
- Test files are located in the /test_cases directory.
- Each file is a .txt file containing public domain text with varying word distributions.

## Definition of a Word
For this challenge, a word is any contiguous sequence of non‐whitespace Unicode characters normalized for comparison, with diacritics considered equivalent

## Expected Output
The program must output a single integer—the total count of unique words across all processed files.

## Evaluation Criteria
Your program will be run on a few edges cases before the official tests.

Following parameters will be used for judging: 
- Execution Time – Faster solutions will receive higher scores.
- Memory Efficiency – Solutions that minimize memory usage will be prioritized.

## Helper Files

The **produce_x** file will create an output.txt file containing X gb worth of words.
It is suggested that you keep the output file size in the range of 100-200mb.

To run:
produce_x 0.2
produce_x 0.15
