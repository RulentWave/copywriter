# Copywriter

**Copywriter** is a command-line tool written in Rust that updates copyright headers and 
license footers in your source code files. It automatically detects the correct comment style 
for various programming languages and supports recursive directory processing.

## Features

- **Automatic Updates:**  
  Inserts a copyright header at the top of your files and appends (or updates) a license 
  footer at the bottom.
- **Flexible Comment Styles:**  
  Supports many programming languages by selecting the proper comment format for each file 
  based on its extension.
- **Dry-Run Mode:**  
  Preview changes without modifying any files.
- **Recursive Processing:**  
  Easily update an entire codebase by specifying a directory.
- **License File Auto-Detection:**  
  By default, the tool searches the project directory tree for a LICENSE file if one is not 
  explicitly provided.
- **Year Update:**  
  If your copyright header already exists but the year is outdated, the tool will update 
  the year (or range of years) automatically.

## Installation

1. **Clone the repository:**

   ```bash
   git clone https://github.com/rulentwave/copywriter.git
   cd copywriter
   ```

2. **Build the project using Cargo:**

   ```bash
   cargo build --release
   ```

   This will create an executable in the `target/release` directory.

## Usage

You can run the tool either through Cargo or directly if you built it in release mode.

### Running via Cargo

```bash
cargo run -- --author "Your Name" path/to/source
```

### Running the Release Build

```bash
./target/release/copywriter --author "Your Name" path/to/source
```

### Command-Line Options

- `--author, -a <NAME>`  
  **(Required)** Specifies the name to use in the copyright header.

- `path`  
  **(Required)** The file or directory to process.

- `--license, -l <FILE>`  
  Specify a custom license file to use instead of automatically searching for one.

- `--dry-run`  
  Runs the tool in a dry-run mode and displays the changes that would be made without 
  actually modifying any files.

## Example

To update all source files in the `src` directory with your name and the current year and 
append your LICENSE file to the bottom:

```bash
./target/release/copywriter --author "Your Name" src/
```

If you wish to preview the changes without modifying any files, run:

```bash
./target/release/copywriter --author "Your Name" --dry-run src/
```

## Supported Languages

The tool supports most common source file extensions, including:

- Rust (`.rs`)
- Python (`.py`)
- JavaScript/TypeScript (`.js`, `.jsx`, `.ts`, `.tsx`)
- C/C++ (`.c`, `.cpp`, `.h`, `.hpp`)
- Java (`.java`)
- Go (`.go`)
- Ruby (`.rb`)
- PHP (`.php`)
- Swift (`.swift`)
- Kotlin (`.kt`)
- C# (`.cs`)
- Shell scripts (`.sh`, `.bash`)
- And many more (see the code for details).

## License

This project is licensed under the [MIT License](LICENSE).
