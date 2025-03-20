#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
DEST_DIR="${1:-$HOME/.cargo/bin}"

echo "Building Rust Journey CLI..."
cd "$SCRIPT_DIR"
cargo build --release

echo "Installing to $DEST_DIR..."
mkdir -p "$DEST_DIR"
cp "$SCRIPT_DIR/target/release/rust-journey-cli" "$DEST_DIR/rust-journey"

# Make executable
chmod +x "$DEST_DIR/rust-journey"

echo "Copying course files..."
COURSE_DIR="${2:-$HOME/rust-journey}"
mkdir -p "$COURSE_DIR"

# Check if we need to clone the repository
if [ ! -d "$COURSE_DIR/chapters" ]; then
    echo "Cloning course repository..."
    # TODO: Replace with your actual GitHub username and repository name
    git clone https://github.com/YOUR_GITHUB_USERNAME/rust-journey.git "$COURSE_DIR"
else
    echo "Course files already exist, skipping clone."
fi

# Copy the info.toml file 
cp "$SCRIPT_DIR/info.toml" "$COURSE_DIR/info.toml"

echo "Installation complete!"
echo "You can now run 'rust-journey' to start working through the exercises."
echo "The course files are located at: $COURSE_DIR"
echo ""
echo "Some example commands:"
echo "  rust-journey list                # List all exercises"
echo "  rust-journey watch               # Start watching the next uncompleted exercise"
echo "  rust-journey hint                # Show a hint for the current exercise"
echo "  rust-journey run \"exercise_name\"  # Run a specific exercise"
echo ""
echo "Happy coding!" 