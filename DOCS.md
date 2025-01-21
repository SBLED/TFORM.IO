# tform Documentation

The `tform.io` crate is designed to convert large, poorly formatted text
into clean, well-structured Markdown or HTML.

## Overview

- **Streaming Support**: Uses `std::io::BufReader` to process text line-by-line.
- **Structure Detection**: Identifies headings, paragraphs, bullet lists, etc.
- **User-Defined Rules**: Override default parsing and formatting with a JSON or TOML config.

### Architecture

1. **`Config`**: Holds user preferences.  
2. **`Parser`**: Converts raw text into an in-memory `Document` structure.  
3. **`Formatter`**: Generates Markdown or HTML from a `Document`.

### Example Usage

```rust
use tform::{Formatter, Config};

fn main() {
    // Assume we have a configuration file
    let config = Config::from_file("tform_config.toml").unwrap_or_default();

    // Instantiate the formatter
    let formatter = Formatter::new(config);

    // Let's say we have some raw input text
    let input_text = "Some raw   text\n# Heading\n- item 1\n- item 2";

    // Convert to markdown
    let markdown = formatter
        .format_to_markdown(input_text.as_bytes())
        .unwrap();

    println!("Markdown output:\n{}", markdown);

    // Convert to HTML
    let html = formatter
        .format_to_html(input_text.as_bytes())
        .unwrap();

    println!("HTML output:\n{}", html);
}
