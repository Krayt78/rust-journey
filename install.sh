#!/bin/bash

# Check if Rust is installed
if ! command -v rustc > /dev/null 2>&1; then
    echo "Error: Rust is not installed."
    echo "Please install Rust from https://www.rust-lang.org/tools/install"
    exit 1
fi

echo "Building the rust-journey CLI tool..."
cargo build --release --manifest-path=rust-journey-cli/Cargo.toml

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Error: Build failed."
    exit 1
fi

# Create ~/.cargo/bin if it doesn't exist
mkdir -p ~/.cargo/bin

echo "Installing the CLI tool to ~/.cargo/bin/rust-journey..."
cp rust-journey-cli/target/release/rust-journey-cli ~/.cargo/bin/rust-journey

# Check if installation was successful
if [ $? -ne 0 ]; then
    echo "Error: Installation failed."
    exit 1
fi

echo "Making the CLI tool executable..."
chmod +x ~/.cargo/bin/rust-journey

echo "Installation completed successfully!"
echo "You can now use the rust-journey CLI tool from any directory."
echo "Note: Always run the CLI tool from the project root directory, not from within the rust-journey-cli folder."
echo ""
echo "Try it now:"
echo "  rust-journey list" 