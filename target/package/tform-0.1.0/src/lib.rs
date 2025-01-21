/*!
# tform Crate

`tform` is a crate that takes poorly formatted text input of arbitrary size
(up to hundreds of megabytes) and converts it into well-structured Markdown or HTML.

## Features

1. Streaming support.
2. Fixes common formatting issues (extra spaces, random newlines, etc.).
3. Detects headings, lists, code blocks, and more.
4. User can override defaults via a configuration file (TOML or JSON).
5. Exposes APIs to produce Markdown or HTML.

## Usage

```rust
use tform::{Formatter, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config (TOML/JSON path or use defaults)
    let config = Config::from_file("tform_config.toml").unwrap_or_default();

    // Create a formatter with the config
    let formatter = Formatter::new(config);

    // Input text (for demonstration, we use a string, but you can also stream from a file)
    let input_text = "Hello   world!\n#Heading 1\nThis  is   a test.";

    // Convert to Markdown
    let markdown = formatter.format_to_markdown(input_text.as_bytes())?;
    println!("Converted Markdown:\n{}", markdown);

    // Convert to HTML
    let html = formatter.format_to_html(input_text.as_bytes())?;
    println!("Converted HTML:\n{}", html);

    Ok(())
}
*/

pub mod config;
pub mod errors;
pub mod formatter;
pub mod parser;
pub mod structures;

// Re-export commonly used structs so callers can do use tform::{Config, Formatter};
pub use config::Config;
pub use formatter::Formatter;