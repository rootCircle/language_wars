# Judging Execution Speed with Hyperfine

Hyperfine is a robust benchmarking tool that helps assess the execution time of commands or programs. It is suitable for comparing multiple code snippets or applications in terms of performance.

## Steps to Use Hyperfine

### Installation

For Linux-based systems, install it using apt, pacman, dnf, brew, or other package managers:

```bash
sudo apt install hyperfine
# or
brew install hyperfine
```

### Run the Benchmark

Use hyperfine to execute a specific command or script repeatedly to measure its execution time.

```bash
hyperfine './your_program arg1 arg2'
```

To compare two or more commands, you can run:

```bash
hyperfine './program_1' './program_2'
```

### Configuration Options

Use the `-r` option to set the number of iterations:

```bash
hyperfine -r 100 './your_program'
```

To save the output to a file for future analysis:

```bash
hyperfine -r 100 --export-json result.json './your_program'
```

## Tie-Breaker Using Heaptrack

In case of a tie between multiple programs with similar execution speeds, memory usage can serve as a tiebreaker. Heaptrack is an excellent tool for tracking memory allocation and peak memory consumption during program execution.

### Steps to Use Heaptrack

#### Installation

Install Heaptrack on your system using the appropriate method for your distribution:

```bash
sudo apt install heaptrack
```

#### Run Heaptrack

To monitor a program's heap memory usage, use the following command:

```bash
heaptrack ./your_program
```

This will generate a `.gz` file containing detailed memory allocation statistics.

#### Analyze the Output

After the program finishes, you can visualize the memory consumption by running:

```bash
heaptrack_gui heaptrack-your_program-*.gz
```

The GUI will display the program's memory usage, highlighting:

- **Peak Memory Usage**: The maximum memory consumption during the program's execution.
- **Allocation Hotspots**: Identify memory areas where large amounts of memory are allocated or deallocated.

### Tie-Breaking Consideration

When the execution times are close, use the peak memory consumption to break the tie. The program that uses less memory, given similar performance, should be preferred.

## Example Scenario for Judging

Two programs are tested for execution time using Hyperfine:

- **Program A**: 1.25 seconds
- **Program B**: 1.30 seconds

Since the times are very close, we look at memory usage to determine the winner.

Heaptrack shows the following peak memory consumption:

- **Program A**: 200 MB
- **Program B**: 150 MB

In this case, Program B would be preferred due to its lower memory usage, despite having a slightly slower execution time.
