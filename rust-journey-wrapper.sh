#!/bin/bash

# Check if we're in the correct directory
if [ ! -f "info.toml" ]; then
    echo "Error: Could not find info.toml in the current directory."
    echo "Please run this command from the project root directory, not from inside the rust-journey-cli directory."
    echo ""
    echo "If you're in the rust-journey-cli directory, try: cd .. && rust-journey $@"
    exit 1
fi

# Run the actual CLI tool
rust-journey "$@" 