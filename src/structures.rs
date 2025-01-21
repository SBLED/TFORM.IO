/*!
Contains data structures representing the intermediate document model.
*/

/// Represents a higher-level block in the parsed document.
#[derive(Debug, Clone)]
pub enum Block {
    Heading(u8, String), // level, text
    Paragraph(String),
    List(Vec<String>),   // simplest representation of a list
    CodeBlock(String),
    // Add more as needed (quotes, tables, etc.)
}

/// A parsed document consists of a series of `Block` items.
#[derive(Debug, Clone)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    /// Create a new, empty document.
    pub fn new() -> Self {
        Document { blocks: vec![] }
    }
}
