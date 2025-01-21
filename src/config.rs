// src/config.rs
/*!
Configuration module for `tform.io`.

Allows loading and storing user-defined formatting rules.
*/

use serde::{Deserialize, Serialize};
use std::fs;

use crate::errors::FormatterError;

/// A set of user-definable rules for text formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// If true, remove multiple consecutive spaces.
    pub remove_extra_spaces: bool,

    /// If true, detect lines starting with "#" as headings.
    pub detect_headings: bool,

    /// If true, detect bullet points in lines starting with "-", "*", "+", etc.
    pub detect_lists: bool,

    /// Additional user-defined patterns that can override default detection rules.
    pub custom_patterns: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            remove_extra_spaces: true,
            detect_headings: true,
            detect_lists: true,
            custom_patterns: vec![],
        }
    }
}

impl Config {
    /// Loads a configuration from a file (TOML or JSON).
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, FormatterError> {
        let contents = fs::read_to_string(path)?;
        let config: Self = if contents.trim_start().starts_with('{') {
            // JSON
            serde_json::from_str(&contents)?
        } else {
            // Assume TOML
            toml::from_str(&contents)?
        };
        Ok(config)
    }
}
