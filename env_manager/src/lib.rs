use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub mod validator;
pub use validator::*;
pub use validator::{EnvLinter, ReferenceChecker, SecurityScanner};

/// Error types for .env file operations
#[derive(Error, Debug)]
pub enum EnvError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },
    
    #[error("Invalid variable expansion: {0}")]
    VariableExpansionError(String),
    
    #[error("Required variable missing: {0}")]
    MissingVariable(String),
}

pub type Result<T> = std::result::Result<T, EnvError>;

/// Represents a single environment variable entry
#[derive(Debug, Clone)]
pub struct EnvEntry {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
    pub line_number: usize,
}

/// Represents a parsed .env file
#[derive(Debug, Clone)]
pub struct EnvFile {
    pub path: PathBuf,
    pub entries: HashMap<String, EnvEntry>,
    pub comments: Vec<String>, // Top-level comments before each entry
}

impl EnvFile {
    /// Create a new empty EnvFile
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: HashMap::new(),
            comments: Vec::new(),
        }
    }
    
    /// Load and parse a .env file from disk
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() {
            return Err(EnvError::FileNotFound(path.clone()));
        }
        
        let content = fs::read_to_string(&path)?;
        let mut env_file = Self::parse(&content)?;
        env_file.path = path;
        
        Ok(env_file)
    }
    
    /// Parse .env content from a string
    pub fn parse(content: &str) -> Result<Self> {
        let mut env_file = Self::new(PathBuf::from(".env"));
        let mut pending_comment: Option<String> = None;
        
        for (line_num, line) in content.lines().enumerate() {
            let line_number = line_num + 1;
            
            // Skip empty lines
            if line.trim().is_empty() {
                pending_comment = None;
                continue;
            }
            
            // Handle comments
            if line.trim().starts_with('#') {
                pending_comment = Some(line.trim_start_matches('#').trim().to_string());
                continue;
            }
            
            // Parse key-value pair
            if let Some((key, value)) = Self::parse_line(line)? {
                let entry = EnvEntry {
                    key: key.clone(),
                    value,
                    comment: pending_comment.take(),
                    line_number,
                };
                env_file.entries.insert(key, entry);
            } else {
                pending_comment = None;
            }
        }
        
        Ok(env_file)
    }
    
    /// Parse a single line from .env file
    fn parse_line(line: &str) -> Result<Option<(String, String)>> {
        let line = line.trim();
        
        // Find the first '=' that's not inside quotes
        let mut in_quotes = false;
        let mut quote_char = None;
        let mut equals_pos = None;
        
        for (i, ch) in line.char_indices() {
            match ch {
                '"' | '\'' => {
                    if !in_quotes {
                        in_quotes = true;
                        quote_char = Some(ch);
                    } else if Some(ch) == quote_char {
                        in_quotes = false;
                        quote_char = None;
                    }
                }
                '=' if !in_quotes => {
                    equals_pos = Some(i);
                    break;
                }
                _ => {}
            }
        }
        
        match equals_pos {
            Some(pos) => {
                let key = line[..pos].trim().to_string();
                let value = Self::parse_value(&line[pos + 1..])?;
                
                if key.is_empty() {
                    return Err(EnvError::ParseError {
                        line: 0,
                        message: "Empty key".to_string(),
                    });
                }
                
                Ok(Some((key, value)))
            }
            None => Ok(None),
        }
    }
    
    /// Parse value, handling quotes and special characters
    fn parse_value(value: &str) -> Result<String> {
        let trimmed = value.trim();
        
        // Handle quoted values
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            if trimmed.len() < 2 {
                return Ok(String::new());
            }
            // Remove surrounding quotes
            return Ok(trimmed[1..trimmed.len() - 1].to_string());
        }
        
        // Handle unquoted values with special characters
        if trimmed.contains(['#', '&', '=', ' ']) {
            // Check if it needs quoting
            return Ok(trimmed.to_string());
        }
        
        // Simple unquoted value
        Ok(trimmed.to_string())
    }
    
    /// Get a variable value by key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key).map(|e| &e.value)
    }
    
    /// Set or update a variable
    pub fn set(&mut self, key: String, value: String, comment: Option<String>) {
        let entry = EnvEntry {
            key: key.clone(),
            value,
            comment,
            line_number: 0, // Will be set when saved
        };
        self.entries.insert(key, entry);
    }
    
    /// Remove a variable
    pub fn remove(&mut self, key: &str) -> Option<EnvEntry> {
        self.entries.remove(key)
    }
    
    /// Expand variables that reference other variables
    pub fn expand_variables(&mut self) -> Result<()> {
        let regex = regex::Regex::new(r"\$\{([^}]+)\}|\$([A-Za-z_][A-Za-z0-9_]*)")
            .map_err(|e| EnvError::VariableExpansionError(e.to_string()))?;
        
        // Keep expanding until no more changes are needed
        let mut max_iterations = 10; // Prevent infinite loops
        let mut changed = true;
        
        while changed && max_iterations > 0 {
            changed = false;
            max_iterations -= 1;
            
            // Collect all values first to avoid borrow issues
            let mut updates = Vec::new();
            
            for (key, entry) in self.entries.iter() {
                if regex.is_match(&entry.value) {
                    let expanded = Self::expand_value(&entry.value, self)?;
                    if expanded != entry.value {
                        updates.push((key.clone(), expanded));
                        changed = true;
                    }
                }
            }
            
            // Apply updates
            for (key, expanded_value) in updates {
                if let Some(entry) = self.entries.get_mut(&key) {
                    entry.value = expanded_value;
                }
            }
        }
        
        Ok(())
    }
    
    /// Expand a single value
    fn expand_value(value: &str, env: &Self) -> Result<String> {
        let regex = regex::Regex::new(r"\$\{([^}]+)\}|\$([A-Za-z_][A-Za-z0-9_]*)")
            .map_err(|e| EnvError::VariableExpansionError(e.to_string()))?;
        
        let mut result = value.to_string();
        
        for cap in regex.captures_iter(value) {
            let var_name = cap.get(1).or(cap.get(2)).unwrap().as_str();
            
            if let Some(replacement) = env.get(var_name) {
                result = result.replace(&cap[0], replacement);
            } else {
                return Err(EnvError::VariableExpansionError(format!(
                    "Referenced variable '{}' not found",
                    var_name
                )));
            }
        }
        
        Ok(result)
    }
    
    /// Save the .env file to disk
    pub fn save(&self) -> Result<()> {
        let content = self.to_string();
        fs::write(&self.path, content)?;
        Ok(())
    }
    
    /// Convert to string format
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        
        for entry in self.entries.values() {
            if let Some(ref comment) = entry.comment {
                output.push_str(&format!("# {}\n", comment));
            }
            output.push_str(&format!("{}={}\n", entry.key, entry.value));
        }
        
        output
    }
    
    /// List all keys
    pub fn keys(&self) -> Vec<&String> {
        self.entries.keys().collect()
    }
    
    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }
}

/// Environment type enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnvType {
    Local,
    Production,
    Staging,
    Test,
    Custom(String),
}

impl EnvType {
    pub fn from_filename(filename: &str) -> Self {
        match filename {
            ".env" | ".env.local" => EnvType::Local,
            ".env.production" => EnvType::Production,
            ".env.staging" => EnvType::Staging,
            ".env.test" => EnvType::Test,
            other => {
                // Try to extract custom environment name
                let name = other.strip_prefix(".env.").unwrap_or(other);
                EnvType::Custom(name.to_string())
            }
        }
    }
    
    pub fn to_filename(&self) -> String {
        match self {
            EnvType::Local => ".env".to_string(),
            EnvType::Production => ".env.production".to_string(),
            EnvType::Staging => ".env.staging".to_string(),
            EnvType::Test => ".env.test".to_string(),
            EnvType::Custom(name) => format!(".env.{}", name),
        }
    }
}

/// Manager for multiple environment files
#[derive(Debug)]
pub struct EnvManager {
    pub base_path: PathBuf,
    pub environments: HashMap<EnvType, EnvFile>,
}

impl EnvManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            environments: HashMap::new(),
        }
    }
    
    /// Load all environment files from a directory
    pub fn load_all(&mut self) -> Result<()> {
        let env_types = [
            EnvType::Local,
            EnvType::Production,
            EnvType::Staging,
            EnvType::Test,
        ];
        
        for env_type in env_types {
            let path = self.base_path.join(env_type.to_filename());
            if path.exists() {
                let env_file = EnvFile::load(&path)?;
                self.environments.insert(env_type, env_file);
            }
        }
        
        Ok(())
    }
    
    /// Load a specific environment
    pub fn load_env(&mut self, env_type: EnvType) -> Result<()> {
        let path = self.base_path.join(env_type.to_filename());
        let env_file = EnvFile::load(&path)?;
        self.environments.insert(env_type, env_file);
        Ok(())
    }
    
    /// Get an environment
    pub fn get_env(&self, env_type: &EnvType) -> Option<&EnvFile> {
        self.environments.get(env_type)
    }
    
    /// Add or update an environment
    pub fn set_env(&mut self, env_type: EnvType, env_file: EnvFile) {
        self.environments.insert(env_type, env_file);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple_env() {
        let content = r#"
# Database config
DB_HOST=localhost
DB_PORT=5432
DB_USER=admin
"#;
        
        let env = EnvFile::parse(content).unwrap();
        assert_eq!(env.get("DB_HOST"), Some(&"localhost".to_string()));
        assert_eq!(env.get("DB_PORT"), Some(&"5432".to_string()));
        assert_eq!(env.get("DB_USER"), Some(&"admin".to_string()));
    }
    
    #[test]
    fn test_parse_quoted_values() {
        let content = r#"
API_KEY="secret-key-123"
PASSWORD='my$password'
MESSAGE="Hello World"
"#;
        
        let env = EnvFile::parse(content).unwrap();
        assert_eq!(env.get("API_KEY"), Some(&"secret-key-123".to_string()));
        assert_eq!(env.get("PASSWORD"), Some(&"my$password".to_string()));
        assert_eq!(env.get("MESSAGE"), Some(&"Hello World".to_string()));
    }
    
    #[test]
    fn test_variable_expansion() {
        let content = r#"
BASE_URL=https://example.com
API_URL=${BASE_URL}/api/v1
FULL_URL=$API_URL/users
"#;
        
        let mut env = EnvFile::parse(content).unwrap();
        env.expand_variables().unwrap();
        
        assert_eq!(env.get("API_URL"), Some(&"https://example.com/api/v1".to_string()));
        assert_eq!(env.get("FULL_URL"), Some(&"https://example.com/api/v1/users".to_string()));
    }

    #[test]
    fn test_empty_file() {
        let content = "";
        let env = EnvFile::parse(content).unwrap();
        assert!(env.entries.is_empty());
    }

    #[test]
    fn test_comments_only() {
        let content = r#"
# This is a comment
# Another comment
"#;
        let env = EnvFile::parse(content).unwrap();
        assert!(env.entries.is_empty());
    }

    #[test]
    fn test_malformed_lines() {
        let content = r#"
INVALID_LINE_WITHOUT_EQUALS
VALID_KEY=value
ANOTHER_INVALID
"#;
        let env = EnvFile::parse(content).unwrap();
        assert_eq!(env.entries.len(), 1);
        assert!(env.contains_key("VALID_KEY"));
    }

    #[test]
    fn test_set_and_remove() {
        let mut env = EnvFile::new(PathBuf::from(".env"));
        
        // Test set
        env.set("NEW_KEY".to_string(), "new_value".to_string(), Some("A comment".to_string()));
        assert_eq!(env.get("NEW_KEY"), Some(&"new_value".to_string()));
        
        // Test remove
        let removed = env.remove("NEW_KEY");
        assert!(removed.is_some());
        assert!(!env.contains_key("NEW_KEY"));
        
        // Test remove non-existent
        let removed = env.remove("NON_EXISTENT");
        assert!(removed.is_none());
    }

    #[test]
    fn test_contains_key() {
        let content = r#"
KEY1=value1
KEY2=value2
"#;
        let env = EnvFile::parse(content).unwrap();
        assert!(env.contains_key("KEY1"));
        assert!(env.contains_key("KEY2"));
        assert!(!env.contains_key("KEY3"));
    }

    #[test]
    fn test_keys_iterator() {
        let content = r#"
A=1
B=2
C=3
"#;
        let env = EnvFile::parse(content).unwrap();
        let keys = env.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.iter().any(|k| k.as_str() == "A"));
        assert!(keys.iter().any(|k| k.as_str() == "B"));
        assert!(keys.iter().any(|k| k.as_str() == "C"));
    }

    #[test]
    fn test_save_to_string() {
        let content = r#"
DB_HOST=localhost
DB_PORT=5432
"#;
        let env = EnvFile::parse(content).unwrap();
        let output = env.to_string();
        
        // Check that output contains the keys and values
        assert!(output.contains("DB_HOST"));
        assert!(output.contains("localhost"));
        assert!(output.contains("DB_PORT"));
        assert!(output.contains("5432"));
    }

    #[test]
    fn test_parse_with_empty_lines() {
        let content = r#"
KEY1=value1

KEY2=value2


KEY3=value3
"#;
        let env = EnvFile::parse(content).unwrap();
        assert_eq!(env.entries.len(), 3);
    }

    #[test]
    fn test_parse_with_inline_comments() {
        let content = r#"
API_KEY=secret # this is an inline comment
PORT=8080
"#;
        let env = EnvFile::parse(content).unwrap();
        // Note: depending on implementation, inline comments might be part of value
        // This tests the current behavior
        assert!(env.contains_key("API_KEY"));
        assert!(env.contains_key("PORT"));
    }

    #[test]
    #[should_panic(expected = "ParseError")]
    fn test_empty_key_should_fail() {
        let content = "=value";
        let _ = EnvFile::parse(content).unwrap();
    }
}
