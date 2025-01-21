#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use tform::config::Config;
    use tform::formatter::Formatter;

    /// Test that a simple heading is correctly formatted as Markdown.
    #[test]
    fn test_format_to_markdown_heading() {
        let config = Config::default();
        let formatter = Formatter::new(config);
        
        // Input containing a heading line
        let input = "# My Title\nSome paragraph text.";
        
        // Convert to Markdown
        let output = formatter
            .format_to_markdown(Cursor::new(input))
            .expect("Failed to format to Markdown");
        
        // We expect the parser + formatter to produce:
        // "# My Title\n\nSome paragraph text.\n\n"
        // There can be slight differences if your parser merges lines/spaces.
        assert!(output.contains("# My Title"), "Heading not found in output.");
        assert!(output.contains("Some paragraph text."), "Paragraph not found in output.");
    }

    /// Test that a simple heading is correctly formatted as HTML.
    #[test]
    fn test_format_to_html_heading() {
        let config = Config::default();
        let formatter = Formatter::new(config);
        
        let input = "# My Title\nSome paragraph text.";
        let output = formatter
            .format_to_html(Cursor::new(input))
            .expect("Failed to format to HTML");
        
        // We expect: 
        // <h1>My Title</h1>
        // <p>Some paragraph text.</p>
        assert!(output.contains("<h1>My Title</h1>"), "Heading not found in HTML output.");
        assert!(output.contains("<p>Some paragraph text.</p>"), "Paragraph not found in HTML output.");
    }

    /// Test that lists are recognized and converted to Markdown bullet points.
    #[test]
    fn test_format_to_markdown_list() {
        let config = Config::default();
        let formatter = Formatter::new(config);
        
        let input = "- item one\n- item two\n- item three";
        let output = formatter
            .format_to_markdown(Cursor::new(input))
            .expect("Failed to format list to Markdown");
        
        // Should produce:
        // - item one
        // - item two
        // - item three
        //
        // with extra newlines possibly in between blocks, depending on your parser.
        assert!(output.contains("- item one"), "List item 'item one' missing");
        assert!(output.contains("- item two"), "List item 'item two' missing");
        assert!(output.contains("- item three"), "List item 'item three' missing");
    }

    /// Test that lists are recognized and converted to HTML list items.
    #[test]
    fn test_format_to_html_list() {
        let config = Config::default();
        let formatter = Formatter::new(config);
        
        let input = "- item one\n- item two\n- item three";
        let output = formatter
            .format_to_html(Cursor::new(input))
            .expect("Failed to format list to HTML");
        
        // Expect something like:
        // <ul>
        //   <li>item one</li>
        //   <li>item two</li>
        //   <li>item three</li>
        // </ul>
        assert!(output.contains("<ul>"), "Missing <ul> tag");
        assert!(output.contains("<li>item one</li>"), "Missing item one <li>");
        assert!(output.contains("<li>item two</li>"), "Missing item two <li>");
        assert!(output.contains("<li>item three</li>"), "Missing item three <li>");
        assert!(output.contains("</ul>"), "Missing closing </ul> tag");
    }

    /// Test that code blocks are recognized and converted in Markdown output.
    #[test]
    fn test_format_to_markdown_code_block() {
        let config = Config::default();
        let formatter = Formatter::new(config);

        // Depending on your parser, you might detect triple-backticks or other patterns.
        // For example usage, let's assume code blocks are matched if the line starts with "```".
        // If your parser uses a different logic, adjust this test accordingly.
        let input = "```\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
        let output = formatter
            .format_to_markdown(Cursor::new(input))
            .expect("Failed to format code block to Markdown");
        
        // Expect:
        // ```
        // fn main() {
        //     println!("Hello, world!");
        // }
        // ```
        assert!(output.contains("```\nfn main() {"), "Code block formatting missing or incorrect");
        assert!(output.contains("println!(\"Hello, world!\");"), "Expected code line missing");
        assert!(output.contains("```\n\n"), "Code block not closed properly");
    }

    /// Test that code blocks are recognized and converted in HTML output.
    #[test]
    fn test_format_to_html_code_block() {
        let config = Config::default();
        let formatter = Formatter::new(config);

        let input = "```\nfn main() {\n    println!(\"Hello, world!\");\n}\n```";
        let output = formatter
            .format_to_html(Cursor::new(input))
            .expect("Failed to format code block to HTML");
        
        // Expect something like:
        // <pre><code>fn main() {
        //     println!("Hello, world!");
        // }</code></pre>
        assert!(output.contains("<pre><code>"), "Missing <pre><code> for code block");
        assert!(output.contains("fn main() {"), "Expected code snippet not found");
        assert!(output.contains("</code></pre>"), "Code block not closed properly");
    }

    /// Optional: Test custom Config variations if your parser handles them (e.g., removing extra spaces).
    #[test]
    fn test_custom_config() {
        // Example: If your config can disable heading detection, we can test that:
        let mut config = Config::default();
        config.detect_headings = false; // For example

        let formatter = Formatter::new(config);
        let input = "Not A Heading\nJust a line.";
        let output = formatter
            .format_to_markdown(Cursor::new(input))
            .expect("Failed with custom config");
        
        // Because `detect_headings` is false, the '#' line might be interpreted as a normal paragraph.
        // Adjust your assertions based on how the parser is supposed to behave with headings off.
        assert!(!output.contains("# Not A Heading"), "Heading was incorrectly formatted");
        assert!(output.contains("Not A Heading"), "Expected text missing in output");
    }
}
