/*!
Parsing logic: reads input stream, cleans up text, and builds `Document`.
*/

use std::io::BufRead;
use regex::Regex;

use crate::config::Config;
use crate::errors::FormatterError;
use crate::structures::{Block, Document};

/// A parser that applies rules from `Config` to build a `Document`.
pub struct Parser {
    config: Config,
    // Compiled regex for efficiency
    re_extra_spaces: Regex,
    re_heading: Regex,
    re_bullet: Regex,
}

impl Parser {
    /// Creates a new `Parser` from a `Config`.
    pub fn new(config: Config) -> Self {
        Parser {
            config,
            re_extra_spaces: Regex::new(r"\s{2,}").unwrap(),
            re_heading: Regex::new(r"^(#+)\s*(.*)$").unwrap(), // e.g., "# Heading"
            re_bullet: Regex::new(r"^[-*+]\s+(.*)$").unwrap(),
        }
    }

    /// Parses text from a buffered reader into a `Document`.
    pub fn parse<R: BufRead>(&self, reader: R) -> Result<Document, FormatterError> {
        let mut doc = Document::new();

        let mut paragraph_buffer = String::new();
        let mut list_buffer: Vec<String> = vec![];

        // New for code blocks:
        let mut in_code_block = false;
        let mut code_block_buffer: Vec<String> = vec![];

        for line_result in reader.lines() {
            let mut line = line_result?;
            line = line.trim_end().to_string(); // remove trailing whitespace

            // Check for triple-backtick line (open/close code block)
            if line.trim() == "```" {
                if in_code_block {
                    // Closing a code block
                    in_code_block = false;

                    // Push the accumulated code as a CodeBlock
                    let code_text = code_block_buffer.join("\n");
                    doc.blocks.push(Block::CodeBlock(code_text));
                    code_block_buffer.clear();
                } else {
                    // Entering a code block
                    in_code_block = true;

                    // Flush any existing paragraph or list
                    self.flush_paragraph(&mut doc, &mut paragraph_buffer);
                    self.flush_list(&mut doc, &mut list_buffer);
                }
                continue;
            }

            // If we're inside a code block, collect lines verbatim
            if in_code_block {
                code_block_buffer.push(line);
                continue;
            }

            // Remove extra spaces if config is set
            if self.config.remove_extra_spaces {
                line = self.re_extra_spaces.replace_all(&line, " ").to_string();
            }

            // Check for headings if enabled
            if self.config.detect_headings {
                if let Some(caps) = self.re_heading.captures(&line) {
                    // Flush paragraph/list first
                    self.flush_paragraph(&mut doc, &mut paragraph_buffer);
                    self.flush_list(&mut doc, &mut list_buffer);

                    let hashes = &caps[1];
                    let text = caps[2].trim();
                    let level = hashes.len() as u8;
                    doc.blocks.push(Block::Heading(level, text.to_string()));
                    continue;
                }
            }

            // Check for lists if enabled
            if self.config.detect_lists {
                if let Some(caps) = self.re_bullet.captures(&line) {
                    // Flush paragraph first
                    self.flush_paragraph(&mut doc, &mut paragraph_buffer);
                    list_buffer.push(caps[1].to_string());
                    continue;
                } else if !list_buffer.is_empty() {
                    // If we were collecting a list and found a non-list line
                    self.flush_list(&mut doc, &mut list_buffer);
                }
            }

            // If line is blank, flush the paragraph
            if line.trim().is_empty() {
                self.flush_paragraph(&mut doc, &mut paragraph_buffer);
            } else {
                // Accumulate paragraph text
                if !paragraph_buffer.is_empty() {
                    paragraph_buffer.push(' ');
                }
                paragraph_buffer.push_str(&line);
            }
        }

        // Flush any remaining paragraph or list
        self.flush_paragraph(&mut doc, &mut paragraph_buffer);
        self.flush_list(&mut doc, &mut list_buffer);

        // If file ended while still in a code block (no closing ```), decide how to handle it:
        if in_code_block {
            let code_text = code_block_buffer.join("\n");
            doc.blocks.push(Block::CodeBlock(code_text));
        }

        Ok(doc)
    }

    /// Helper to finalize and store a paragraph block
    fn flush_paragraph(&self, doc: &mut Document, buffer: &mut String) {
        if !buffer.trim().is_empty() {
            doc.blocks.push(Block::Paragraph(buffer.trim().to_string()));
        }
        buffer.clear();
    }

    /// Helper to finalize and store a list block
    fn flush_list(&self, doc: &mut Document, list_buffer: &mut Vec<String>) {
        if !list_buffer.is_empty() {
            doc.blocks.push(Block::List(list_buffer.clone()));
        }
        list_buffer.clear();
    }
}
