# Rust Journey

A comprehensive hands-on approach to mastering Rust through guided chapters, collaborative workshops, and real-world projects.

## Course Structure

The course is organized into chapters covering different aspects of Rust programming:

```
chapters/
├── 01_fundamentals/
│   ├── 01_hello_world/
│   ├── 02_variables/
│   ├── 03_data_types/
│   ├── 04_functions/
│   └── 05_control_flow/
├── 02_ownership/
├── 03_structs_enums/
└── ...
```

Each chapter contains:
- A README.md file explaining key concepts
- Example code demonstrating the concepts
- Challenge exercises for you to solve

## Getting Started with the CLI Tool

This course includes a CLI tool that guides you through the exercises and provides automated verification.

### Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (latest stable version)
- Git

### Installation

#### Option 1: Quick Install (Recommended)

Run the installation script from the project root:

```bash
# Clone the repository
git clone https://github.com/krayt78/rust-journey.git
cd rust-journey

# Install the CLI tool directly from the project root
cargo build --release --manifest-path=rust-journey-cli/Cargo.toml
cp rust-journey-cli/target/release/rust-journey-cli ~/.cargo/bin/rust-journey
```

#### Option 2: Manual Installation

If you prefer to install manually:

```bash
# Clone the repository
git clone https://github.com/YOUR_GITHUB_USERNAME/rust-journey.git
cd rust-journey

# Build the CLI tool from the root directory
cargo build --release --manifest-path=rust-journey-cli/Cargo.toml

# Install it (assumes ~/.cargo/bin is in your PATH)
cp rust-journey-cli/target/release/rust-journey-cli ~/.cargo/bin/rust-journey
```

### Using the CLI Tool

**Important**: Always run the CLI tool from the project root directory, not from within the `rust-journey-cli` folder.

Once installed, you can use the following commands:

```bash
# List all exercises and their status
rust-journey list

# Start working on the next uncompleted exercise
rust-journey next

# Watch for changes in the current exercise (automatically verifies when you save)
rust-journey watch

# Get a hint for the current exercise
rust-journey hint

# Run a specific exercise
rust-journey run "exercise_name"

# Verify all exercises
rust-journey verify
```

#### Using the Wrapper Script (Alternative)

If you've accidentally been running commands from inside the `rust-journey-cli` directory, we've included a wrapper script that can help. From the project root, run:

```bash
# Make the wrapper script executable (if you haven't already)
chmod +x rust-journey-wrapper.sh

# Use the wrapper script which automatically checks for the correct directory
./rust-journey-wrapper.sh list
./rust-journey-wrapper.sh watch
# etc.
```

### Watch Mode (Recommended)

The most effective way to work through the exercises is using the watch mode:

```bash
# Make sure you're in the project root directory
cd /path/to/rust-journey
rust-journey watch
```

This will:
1. Start with your next incomplete exercise
2. Watch for changes to the file
3. Automatically verify when you save
4. Provide feedback on your solution
5. Allow you to move to the next exercise when you succeed

While in watch mode, you can:
- Press `h` for a hint
- Press `l` to see the list of exercises
- Press `n` to move to the next exercise
- Press `q` to quit watch mode

## Learning Path

1. Start with the fundamentals chapter to learn the basics of Rust syntax
2. Progress through subsequent chapters as they build on previous concepts
3. Complete the exercises to reinforce your understanding
4. Use the CLI tool's watch mode for immediate feedback

## Contributing

Contributions to improve the course or the CLI tool are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.