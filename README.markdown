# File Search Utility in Rust

## Overview

This Rust program is a command-line utility designed to search for specific words or phrases within a text file and return the lines containing those terms. It implements an inverted index to efficiently map words to the line numbers where they appear, enabling fast query lookups. The program is useful for tasks such as log file analysis, text searching, or any scenario requiring line-based text retrieval.

## Features

- **Command-Line Interface**: Accepts a file path and one or more search queries as arguments.
- **Inverted Index**: Builds an index of words mapped to line numbers for efficient searching.
- **Case-Sensitive Search**: Matches exact words as provided in the query.
- **Line Number Tracking**: Returns matching lines with their corresponding line numbers.
- **Error Handling**: Gracefully handles invalid inputs, missing files, and other errors.

## How It Works

The program processes a text file and a set of search queries provided via command-line arguments. It performs the following steps:

1. **Parse Command-Line Arguments**: Validates and extracts the file path and search queries.
2. **Read and Index File**: Reads the file line by line, storing each line in a `HashMap` with its line number as the key.
3. **Tokenize Words**: Splits each line into words and builds an inverted index (a `HashMap` mapping words to a `HashSet` of line numbers).
4. **Search for Queries**: For each query, looks up the word in the inverted index and retrieves the corresponding lines.
5. **Display Results**: Prints the line numbers and text of lines containing the queried words.

## Program Structure

The program is organized into several key components:

### 1. Config Struct

- **Purpose**: Holds the configuration derived from command-line arguments.
- **Fields**:
  - `file_path`: The path to the input text file.
  - `search_query`: A slice of search terms provided by the user.
- **Method**:
  - `new(args: &[String]) -> Result<Config, String>`: Parses command-line arguments, ensuring at least a file path and one query are provided. Returns an error if the input is invalid.

### 2. open_and_read_file Function

- **Purpose**: Opens the specified file and returns an iterator over دادأ¢âأنيOpens the specified file and returns an iterator over its lines.
- **Input**: A file path as a string.
- **Output**: A `Result` containing a `Lines<BufReader<File>>` iterator or an error if the file cannot be opened.
- **Details**: Uses `BufReader` for efficient file reading.

### 3. index_file_lines Function

- **Purpose**: Stores each line of the file in a `HashMap<i32, String>`.
- **Inputs**:
  - `lines`: An iterator over the file's lines.
  - `lines_storage`: A mutable reference to the `HashMap` storing line numbers and their text.
- **Details**: Assigns each line a sequential line number starting from 1 and stores the line text.

### 4. tokenize_words Function

- **Purpose**: Builds an inverted index mapping words to the line numbers where they appear.
- **Inputs**:
  - `lines_storage`: The `HashMap` containing line numbers and their text.
  - `token_storage`: A mutable reference to a `HashMap<String, HashSet<i32>>` for the inverted index.
- **Details**:
  - Splits each line into words using whitespace as a delimiter.
  - For each word, updates the `token_storage` by adding the current line number to the word's `HashSet` of line numbers.
  - Handles cases where a word appears multiple times by ensuring unique line numbers.

### 5. get_query_match Function

- **Purpose**: Searches for a query word and retrieves the lines containing it.
- **Inputs**:
  - `user_input`: The query word to search for.
  - `line_storage`: The `HashMap` containing line numbers and their text.
  - `token_storage`: The inverted index mapping words to line numbers.
  - `retrievable_lines`: A mutable `HashMap` to store matching lines.
- **Details**:
  - Looks up the query in the inverted index.
  - For each matching line number, retrieves the line text from `line_storage` and stores it in `retrievable_lines`.
  - Avoids duplicate entries by checking if the line number already exists.

### 6. main Function

- **Purpose**: Orchestrates the program's execution.
- **Steps**:
  - Collects command-line arguments.
  - Creates a `Config` instance to validate inputs.
  - Initializes data structures (`line_storage`, `token_storage`, `retrievable_lines`).
  - Reads and indexes the file.
  - Builds the inverted index.
  - Processes each query to find matching lines.
  - Prints results or a "no result found" message if no matches are found.

## Usage

To use the program, compile and run it with the `cargo` command, providing a file path and one or more search queries.

### Command Format

```bash
cargo run -- <file_path> <query1> [query2 ...]
```

### Example

Suppose you have a file `sample.txt` with the following content:

```
Hello world
This is a test
World of Rust
```

Run the program to search for the words "world" and "test":

```bash
cargo run -- sample.txt world test
```

### Output

```
indexing done>>
matching found @ 1 : Hello world
matching found @ 2 : This is a test
matching found @ 3 : World of Rust
```

If no matches are found:

```
no result found....
```

### Requirements

- **Rust**: Ensure you have the Rust toolchain installed (`rustc` and `cargo`).
- **Input File**: The file must exist and be readable.
- **Arguments**: At least two arguments are required (file path and one query).

## Error Handling

The program includes robust error handling:

- **Invalid Arguments**: If fewer than three arguments are provided, it prints an error message and exits.
- **File Errors**: If the file cannot be opened (e.g., it doesn't exist), the program propagates the error using Rust's `Result` type.
- **Query Handling**: If a query has no matches, the program continues processing other queries and reports "no result found" if no matches are found for any query.

## Potential Improvements

- **Case-Insensitive Search**: Add an option to ignore case when matching queries.
- **Advanced Tokenization**: Support punctuation removal or more sophisticated word splitting.
- **Multiple File Support**: Extend the program to search across multiple files.
- **Performance Optimization**: Use parallel processing for large files or implement caching for repeated queries.
- **Output Formatting**: Allow customizable output formats (e.g., JSON or CSV).

## Dependencies

The program uses the following Rust standard library modules:

- `std::collections::{HashMap, HashSet}`: For storing lines and the inverted index.
- `std::env::args`: For parsing command-line arguments.
- `std::error::Error`: For error handling.
- `std::fs::File`: For file operations.
- `std::io::{BufReader, BufRead}`: For efficient file reading.

No external crates are required, making the program lightweight and portable.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please submit issues or pull requests to the repository. Suggested areas for contribution include:

- Adding new features (e.g., regex support, case-insensitive search).
- Improving performance for large files.
- Enhancing documentation or adding tests.