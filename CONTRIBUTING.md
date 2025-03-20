# Contributing to Rust Journey

Thank you for considering contributing to Rust Journey! This document provides guidelines and instructions for contributing to both the course content and the CLI tool.

## How to Contribute

### For Course Content

1. **Add or Improve Exercises**: 
   - Create new exercises that teach Rust concepts
   - Improve existing exercises for clarity or educational value

2. **Fix Issues**:
   - Correct technical errors in the course material
   - Address unclear explanations or instructions

3. **Expand Coverage**:
   - Add content for Rust features not currently covered
   - Create advanced exercises for topics that need more depth

### For the CLI Tool

1. **Bug Fixes**:
   - Fix issues in the functionality of the CLI tool
   - Address edge cases in exercise verification

2. **Feature Enhancements**:
   - Add new commands or options to improve the learning experience
   - Improve the watch mode or verification mechanics

3. **Documentation**:
   - Improve the CLI help text
   - Add more examples or clarify installation instructions

## Development Setup

### For Course Content

1. Clone the repository:
   ```bash
   git clone https://github.com/YOUR_GITHUB_USERNAME/rust-journey.git
   cd rust-journey
   ```

2. Add or modify exercise files in the appropriate chapter directories

3. Update the `info.toml` file if you're adding new exercises

### For CLI Tool

1. Navigate to the CLI tool directory:
   ```bash
   cd rust-journey/rust-journey-cli
   ```

2. Make your changes to the CLI code

3. Build and test:
   ```bash
   cargo build
   cargo run -- list  # Test the list command
   ```

## Pull Request Process

1. Fork the repository and create a new branch for your changes
2. Make your changes with clear commit messages
3. Test your changes thoroughly
4. Submit a pull request with a clear description of the changes
5. Address any feedback from maintainers

## Coding Standards

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use meaningful variable and function names
- Include comments for complex logic
- Write clear error messages and user-facing text

## License

By contributing, you agree that your contributions will be licensed under the same MIT License that covers the project.

## Contact

If you have questions or need help, please open an issue on the repository. 