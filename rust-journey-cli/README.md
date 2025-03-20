# Rust Journey CLI

A command-line tool to guide learners through the Rust Journey course, similar to rustlings.

## Features

- **Guided Learning**: Step-by-step guidance through Rust exercises
- **Interactive Watch Mode**: Automatic verification when files change
- **Progress Tracking**: Keep track of completed exercises
- **Helpful Hints**: Get hints when you're stuck on an exercise

## Installation

You can install the Rust Journey CLI using the provided installation script:

```bash
# Install to default location ($HOME/.cargo/bin)
./install.sh

# Or specify a custom installation directory
./install.sh /path/to/installation
```

The installation script will:
1. Build the CLI tool
2. Install it to your specified directory
3. Clone the course repository if needed
4. Set up the necessary configuration files

## Usage

Once installed, you can use the following commands:

```bash
# List all exercises and their status
rust-journey list

# Start working on the next incomplete exercise
rust-journey next

# Watch for changes in the current exercise
rust-journey watch

# Get a hint for the current exercise
rust-journey hint

# Run a specific exercise
rust-journey run "exercise_name"

# Verify all exercises
rust-journey verify
```

## How It Works

The CLI tool works by:
1. Loading exercise definitions from an `info.toml` file
2. Tracking your progress in a `.rust-journey-status` file
3. Compiling and running your code to verify solutions
4. Moving you through exercises in a logical order

## Directory Structure

The Rust Journey course is organized as follows:

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

Each exercise is located in its respective topic directory, with a README.md explaining the concepts and exercise files for you to complete.

## Contributing

If you'd like to contribute to the Rust Journey CLI or course content, please:
1. Fork the repository
2. Create a new branch for your changes
3. Submit a pull request with a clear description of your changes

## License

This project is licensed under the MIT License - see the LICENSE file for details. 