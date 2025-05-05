#!/usr/bin/env bash

set -e
set -u
set -o pipefail

LOCK_FILE=".test_cases_generated.lock"

echo "This script will perform a destructive cleanup of your project."
echo "It will remove all untracked files and directories, including:"
echo "- Files in the 'test_cases' directory"
echo "- 'result.txt' and 'result.actual.txt' files in the base directory"
echo "- Judging lockfile ${LOCK_FILE} will be removed as well."
echo "- Any other untracked files and directories"
echo

git clean -fdx --dry-run
read -r -p "Are you sure you want to proceed with this cleanup? [y/n] " confirm

if [ "$confirm" = "y" ]; then
  echo "Cleaning up files..."
  git clean -fdx 
  rm -f result.txt result.actual.txt
  rm -f "$LOCK_FILE" 
else
  echo "Cleanup aborted."
fi
