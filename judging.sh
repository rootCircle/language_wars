#!/usr/bin/env bash

set -e
set -u
set -o pipefail

# Assuming Linux based environment, if not we will test it manually on other platforms

LOCK_FILE=".test_cases_generated.lock"

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

if ! chmod +x ./build.sh; then
    echo "Error: Failed to make build.sh executable."
    exit 1
fi

if ! ./build.sh; then
    echo "Error: build.sh failed."
    exit 1
fi

if [ ! -f "$LOCK_FILE" ]; then
    echo "Lock file does not exist. Generating $LOCK_FILE..."

    if ! command_exists ./file_generator; then
        echo "Error: Command './file_generator' not found."
        exit 1
    fi

    if ! command_exists ./produce_x; then
        echo "Error: Command './produce_x' not found."
        exit 1
    fi

    ./file_generator test_cases/output_2gb.txt 2000
    ./file_generator test_cases/output_5gb.txt 5000
    ./file_generator test_cases/output_20gib.txt 20000

    ./produce_x 200 # generates 200 MiB file in test_cases directory
    ./produce_x 500 # generates 500 MiB file in test_cases directory
    ./produce_x 800 # generates 800 MiB file in test_cases directory

    if ! chmod +x ./unique_word_result; then
      echo "Error: Failed to make unique_word_result executable."
      exit 1
    fi

    ./unique_word_result 'result.actual.txt' > /dev/null 2>&1
    
    touch "$LOCK_FILE"
    
    echo "$LOCK_FILE has been created."
else
    echo "$LOCK_FILE already exists. No action taken."
fi

if ! chmod +x ./run.sh; then
    echo "Error: Failed to make run.sh executable."
    exit 1
fi

./run.sh > /dev/null 2>&1

# Compare the results
if ! diff result.txt result.actual.txt > /dev/null; then
    echo "Error: The output files differ."
    exit 1
fi

# Benchmark the code
hyperfine './run.sh'

