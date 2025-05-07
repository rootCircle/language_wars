#!/usr/bin/env bash

set -e
set -u
set -o pipefail

# Assuming Linux based environment, if not we will test it manually on other platforms

LOCK_FILE=".test_cases_generated.lock"

command_exists() {
    command -v "$1" >/dev/null 2>&1
}

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

    printf "\nCleaning old files before proceeding\n"
    ./clean.sh

    printf "\nGenerating test_cases...\n"
    ./file_generator test_cases/output_2gb.txt 2000
    ./file_generator test_cases/output_5gb.txt 5000
    ./file_generator test_cases/output_10gb.txt 10000

    ./produce_x 200 # generates 200 MiB file in test_cases directory
    ./produce_x 500 # generates 500 MiB file in test_cases directory
    ./produce_x 800 # generates 800 MiB file in test_cases directory

    if ! chmod +x ./unique_word_result.sh; then
      echo "Error: Failed to make unique_word_result executable."
      exit 1
    fi

    printf "\nRunning correct result(slow) for the test_cases in result.actual.txt\n"
    ./unique_word_result.sh test_cases/*.txt 
    
    touch "$LOCK_FILE"
    
    echo "$LOCK_FILE has been created."
else
    echo "$LOCK_FILE already exists. No action taken."
fi

if ! chmod +x ./build.sh; then
    echo "Error: Failed to make build.sh executable."
    exit 1
fi

printf "\nBuilding Files...\n"
if ! ./build.sh; then
    echo "Error: build.sh failed."
    exit 1
fi

if ! chmod +x ./run.sh; then
    echo "Error: Failed to make run.sh executable."
    exit 1
fi

printf "\nRunning Code....\n"
./run.sh > /dev/null 2>&1

# Compare the results
if ! diff --ignore-all-space --ignore-blank-lines --ignore-case result.txt result.actual.txt > /dev/null; then
    echo "Error: The output files differ."
    exit 1
fi
printf "\nBoth file matched, everything is good!\n"

# Benchmark the code
printf "\n\nNow, benchmarking timings\n"
hyperfine './run.sh'

printf "\n\nRunning /usr/bin/time ./run.sh\n"
/usr/bin/time ./run.sh

printf "\nGeneral Debug info\n"
echo "test_cases contents:"
ls test_cases
echo "test_cases folder size:"
du -sh test_cases
echo "system info"
inxi -CmG
