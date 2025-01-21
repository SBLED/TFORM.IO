// examples/streaming.rs

/*
===========================================================
HOW IT WORKS

1. We open "input.txt" using `File::open` and wrap it in a `BufReader`.
2. We create a `Formatter` with default (or custom) `Config`.
3. We call `format_to_markdown(reader)` to stream the file contents
   through the parser, line by line, constructing the final Markdown output.
4. For HTML, we re-open the file (or you could store the string in memory if you prefer),
   create another `BufReader`, and call `format_to_html(reader)`.

USAGE:
  cargo run --example streaming

Ensure an "input.txt" file exists in the same directory or
change the `file_path` variable above to point to a valid file.
===========================================================
*/


use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use tform::{Config, Formatter};

fn main() -> Result<(), Box<dyn Error>> {
    // Path to an input file that contains your raw text.
    // Make sure "input.txt" exists in the same directory where you run this example.
    let file_path = "input.txt";

    // Open the file for reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Create a default config or load from a file if desired
    let config = Config::default();
    let formatter = Formatter::new(config);

    // Convert file contents to Markdown (streamed through BufReader)
    let markdown_output = formatter.format_to_markdown(reader)?;
    println!("=== Markdown Output (Streamed) ===\n{}", markdown_output);

    // If you also want to show HTML output in the same run, you'd need to
    // re-open the file or buffer it. For simplicity, let's do it here:
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Convert file contents to HTML (streamed through BufReader)
    let html_output = formatter.format_to_html(reader)?;
    println!("=== HTML Output (Streamed) ===\n{}", html_output);

    Ok(())
}

