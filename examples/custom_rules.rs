// examples/custom_rules.rs

/*
### How It Works

1. **Loads Configuration**  
   - The code calls `Config::from_file("tform_config.toml")`, which will look for a TOML or JSON file and parse it into a `Config`.  
   - If the file can’t be found or there’s a parsing error, it prints a warning and uses `Config::default()` instead.

2. **Applies Custom Rules**  
   - If your `tform_config.toml` (or JSON file) includes fields like `detect_headings`, `detect_lists`, or your own `custom_patterns`, those will take effect in the parser.  

3. **Processes Input**  
   - The string `input_text` contains headings (`## Custom Rules Example`), a “TODO” note (which you might detect with a custom pattern), bullet points (`+ Additional bullet`), and a code block.  

4. **Outputs Markdown & HTML**  
   - Calls `format_to_markdown` and `format_to_html` on the same input to show how each is transformed.  

To run this example from your project root:

```bash
cargo run --example custom_rules
*/

use std::error::Error;
use tform::{Config, Formatter};

fn main() -> Result<(), Box<dyn Error>> {
    // Change this path to the actual config file you want to load.
    // For example, "tform_config.toml" in the same directory as your crate.
    let config_path = "tform_config.toml";

    // Attempt to load a custom config; if it fails, fall back to default.
    let config = match Config::from_file(config_path) {
        Ok(cfg) => {
            println!("Loaded custom config from '{}'", config_path);
            cfg
        }
        Err(e) => {
            eprintln!("Could not load config from '{}': {}. Using default config instead.", config_path, e);
            Config::default()
        }
    };

    // Create a formatter using either the loaded or default config
    let formatter = Formatter::new(config);

    // Example input text:
    // - It might contain patterns that your config specifically handles (e.g., "TODO").
    // - It also shows headings, lists, and a code block to demonstrate multiple features.
    let input_text = r#"
## Custom Rules Example

TODO: This line might be detected by a custom pattern if configured.
+ Additional bullet
+ Another bullet

```rust
fn main() {
    println!("Hello from custom rules!");
}
"#;

// Convert to Markdown
let markdown_output = formatter.format_to_markdown(input_text.as_bytes())?;
println!("=== Markdown Output (With Custom Rules) ===\n{}", markdown_output);

// Convert to HTML
let html_output = formatter.format_to_html(input_text.as_bytes())?;
println!("=== HTML Output (With Custom Rules) ===\n{}", html_output);

Ok(())
}