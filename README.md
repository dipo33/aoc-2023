# Advent of Code 2023 Solutions in Rust

Welcome to my repository where I tackle the Advent of Code 2023 challenges using Rust! This project is a personal endeavor to explore and improve my Rust programming skills while engaging with the exciting and challenging puzzles provided by Advent of Code.

## Project Structure

The repository is structured with individual crates for each day of the challenge. I've utilized a custom initialization script to streamline the creation of these crates.

### Initialization Script

To set up a new day's challenge, use the `init.sh` script. This script creates a new crate based on a predefined template, ensuring consistency across all daily challenges.

Usage:
```bash
./init.sh [day_number] [result_type]
```
- `day_number`: The day of the challenge (e.g., 7)
- `result_type`: The expected result type for the day's challenge (e.g., `u32`)

Example:
```bash
./init.sh 7 u32
```

This command will set up the crate for Day 7 with `u32` as the result type.

#### Note on Existing Crates
It's important to note that the `init.sh` script cannot be used to initialize a crate for a day that already has an existing crate. For instance, if a crate for Day 7 already exists, running `./init.sh 7 u32` will not work.

### .env File

For the `init.sh` script to function correctly, a `.env` file is required. Please create this file based on the provided `.env.example`.

**Important:** The `.env` file needs to contain a valid Advent of Code session cookie to work correctly. This cookie is essential for the script to fetch the inputs for each day's challenge.

## Contributing

While this is a personal project, I'm open to suggestions and improvements. Feel free to fork the repository and submit pull requests.

## License

This project is open-sourced under the [MIT License](LICENSE).

Happy coding and best of luck with the Advent of Code 2023!
