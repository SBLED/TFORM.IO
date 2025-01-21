/*!
Formatter: orchestrates parsing and then renders as Markdown or HTML.
*/

use std::io::{BufReader, Read};

use crate::{
    config::Config,
    errors::FormatterError,
    parser::Parser,
    structures::{Block, Document},
};

/// Main entry point for text formatting.
pub struct Formatter {
    parser: Parser,
}

impl Formatter {
    /// Create a new Formatter from a `Config`.
    pub fn new(config: Config) -> Self {
        let parser = Parser::new(config);
        Formatter { parser }
    }

    /// Format input (via `Read`) into Markdown.
    pub fn format_to_markdown<R: Read>(&self, reader: R) -> Result<String, FormatterError> {
        let buf_reader = BufReader::new(reader);
        let doc = self.parser.parse(buf_reader)?;
        Ok(document_to_markdown(&doc))
    }

    /// Format input (via `Read`) into HTML.
    pub fn format_to_html<R: Read>(&self, reader: R) -> Result<String, FormatterError> {
        let buf_reader = BufReader::new(reader);
        let doc = self.parser.parse(buf_reader)?;
        Ok(document_to_html(&doc))
    }
}

/// Convert intermediate Document to Markdown string.
fn document_to_markdown(doc: &Document) -> String {
    let mut output = String::new();

    for block in &doc.blocks {
        match block {
            Block::Heading(level, text) => {
                let hashes = "#".repeat(*level as usize);
                output.push_str(&format!("{} {}\n\n", hashes, text));
            }
            Block::Paragraph(text) => {
                output.push_str(text);
                output.push_str("\n\n");
            }
            Block::List(items) => {
                for item in items {
                    output.push_str(&format!("- {}\n", item));
                }
                output.push('\n');
            }
            Block::CodeBlock(code) => {
                output.push_str("```\n");
                output.push_str(code);
                output.push_str("\n```\n\n");
            }
        }
    }
    output
}

/// Convert intermediate Document to HTML string.
fn document_to_html(doc: &Document) -> String {
    let mut output = String::new();

    for block in &doc.blocks {
        match block {
            Block::Heading(level, text) => {
                output.push_str(&format!("<h{lvl}>{}</h{lvl}>\n", text, lvl = level));
            }
            Block::Paragraph(text) => {
                output.push_str("<p>");
                output.push_str(text);
                output.push_str("</p>\n");
            }
            Block::List(items) => {
                output.push_str("<ul>\n");
                for item in items {
                    output.push_str(&format!("<li>{}</li>\n", item));
                }
                output.push_str("</ul>\n");
            }
            Block::CodeBlock(code) => {
                output.push_str("<pre><code>");
                // For real usage, you may need to escape HTML inside `code`.
                output.push_str(code);
                output.push_str("</code></pre>\n");
            }
        }
    }
    output
}
