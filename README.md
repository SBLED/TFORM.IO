# TFORM.IO

A Rust crate that cleans and converts large, poorly formatted text into well-structured Markdown or HTML.  
Designed for streaming (line-by-line) processing of up to hundreds of megabytes of text, **TFORM.IO** removes extra spaces, merges paragraphs, detects headings/lists/code blocks, and lets you override defaults with a simple configuration file.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Examples](#examples)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

---

## Features

### Streaming Support
Process very large text files or streams without loading them entirely into memory.

### Automatic Markdown/HTML Conversion
- Headings (lines starting with `#`, `##`, etc.)
- Bullet lists (lines starting with `-`, `+`, or `*`)
- Code blocks (triple backticks)
- Paragraph separation on blank lines

### Configurable
- Enable/disable headings, list detection, or space-trimming.
- Define custom regex patterns (future expansion).
- Load config from TOML or JSON files.

### High Performance
Written in Rust to handle up to 512 MB of input efficiently.

---

## Installation

Add **TFORM.IO** to your `Cargo.toml`:

```toml
[dependencies]
tform = "0.1.0"
```
Then run:

```bash
cargo build
```

## Usage
Here’s a minimal example of using TFORM.IO:

```rust
use tform::{Config, Formatter};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load config (TOML/JSON) or default
    let config = Config::from_file("tform_config.toml").unwrap_or_default();
    let formatter = Formatter::new(config);

    // Input text (could come from a file, user input, etc.)
    let input_text = r#"


# Heading 1

This   is some    example text.

- item one
- item two

fn main() { println!("Hello, world!"); }

This is some
more example text.
"#;

    // Convert to Markdown
    let markdown_output = formatter.format_to_markdown(input_text.as_bytes())?;
    println!("--- Markdown ---\n{}", markdown_output);

    // Convert to HTML
    let html_output = formatter.format_to_html(input_text.as_bytes())?;
    println!("--- HTML ---\n{}", html_output);

    Ok(())
}
```

Compile and run:
```bash
cargo run
```
---
## Configuration
By default, TFORM.IO uses:
```toml
remove_extra_spaces = true
detect_headings = true
detect_lists = true
custom_patterns = []
```

You can override these by creating a tform_config.toml or JSON file. For example:
```toml
# tform_config.toml
remove_extra_spaces = true
detect_headings = false
detect_lists = true
custom_patterns = ["(?i)todo"]
```

Then load it:
```rust
let config = Config::from_file("tform_config.toml").unwrap_or_default();
let formatter = Formatter::new(config);
```
If detect_headings = false, # Some Text is treated as normal paragraph text instead of a heading.

---
## Examples
TFORM.IO includes example programs under the examples/ folder. You can run them with:

#### Basic Formatting
```bash
cargo run --example basic_formatting
```
#### Custom Rules
```bash
cargo run --example custom_rules
```
#### Streaming
```bash
cargo run --example streaming
```
Each example demonstrates different aspects of TFORM.IO, like loading a config, processing large files line-by-line, or basic text transformations.

---
### Testing
We have both unit tests (within modules) and integration tests (in tests/integration_tests.rs). Run them all:

```bash
cargo test
```
This validates:
* Heading detection (HTML & Markdown)
* List detection
* Code block handling via triple backticks
* Custom config usage (e.g., disabling headings)

---
### Contributing
1. Fork the repository and clone it locally.
2. Create a branch for your feature or bug fix.
3. Write tests that cover your changes.
4. Submit a Pull Request on GitHub with a clear description of your work.
We welcome all suggestions and improvements!
---
### License
This project is licensed under the MIT License. You’re free to use, modify, and distribute this software under its terms.
---
Enjoy TFORM.IO!
With TFORM.IO, you can painlessly convert jumbled text into neat Markdown or HTML—perfect for documentation, PDF generation, or any structured text workflow. If you have any questions or feedback, feel free to open an issue or submit a pull request.