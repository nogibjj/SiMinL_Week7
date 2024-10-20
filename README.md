# Rust Project Template with functional CI/CD, devcontainer, dockerfile

This repository contains Rust implementations for approximating definite integrals of functions with the Trapezoidal Rule. Basic structure including CI/CD, Testing, Makefile and reproducible environments are included.

# Project Structure
- SRC: Contains lib.rs which has the core functions to compute  , and main.rs which calls functions from lib.rs
- Data

# Functions

# Running the Code
## Option 1
Prerequisites: Rust is installed

1. Build and run
- cargo build
2. Execute the main program
- cargo run
3. Testing
- make test
4. Linting
- Make lint
5. Formatting
- Make format

## Option 2
Download Prebuilt Binary (No Rust Required)
Download the prebuilt binary artifact and ensure using a Linux environment (e.g., GitHub Codespaces, a Linux VM, or a Linux machine)

1. Go to the Actions tab in the repository.
2. Select the latest workflow run.
3. Scroll down to Artifacts and download the yijia_ids706_mini_proj7.
4. Move the binary to a Linux environment where you want to run the tool (e.g., GitHub Codespaces or a Linux machine).
Usage of Binary:
./yijia_ids706_mini_proj7

---It will prompt you to enter a list of numbers (separated by spaces) and will return the mean and median.---

CI/CD Pipeline
This project uses GitHub Actions for continuous integration. The pipeline automatically:

- Checks formatting using cargo fmt.
- Lints the code with cargo clippy.
= Runs tests using cargo test.
- Builds the project in release mode using cargo build --release.