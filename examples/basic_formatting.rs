// examples/basic_formatting.rs

use tform::{Config, Formatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Create a default configuration
    let config = Config::default();
    let formatter = Formatter::new(config);

    // Example input text
    let input_text = r#"
# My Title   
 
Some    paragraph text with   extra spaces or random
newlines. Some random newlines wit
hin the middle of words!  

- item one
- item two
- item three
"#;

    // Convert to Markdown
    let markdown_output = formatter.format_to_markdown(input_text.as_bytes())?;
    println!("=== Markdown Output ===\n{}", markdown_output);

    // Convert to HTML
    let html_output = formatter.format_to_html(input_text.as_bytes())?;
    println!("=== HTML Output ===\n{}", html_output);

    Ok(())
}
