# Rust Project Management Skill

This skill teaches best practices for managing Rust projects, specifically for the env_manager codebase.

## Purpose

Maintain clean, efficient, and idiomatic Rust code following project conventions.

## Project Structure

```
env_manager/
├── src/
│   ├── main.rs      # CLI entry point and command parsing
│   ├── lib.rs       # Core types and EnvFile implementation
│   └── validator.rs # Validation rules and linters
├── Cargo.toml        # Dependencies and metadata
└── Cargo.lock        # Locked dependency versions
```

## Code Style Guidelines

### Error Handling
- Use `thiserror` for custom error types
- Return `Result<T>` from fallible functions
- Provide context in error messages
- Use `?` operator for error propagation

Example:
```rust
#[derive(Error, Debug)]
pub enum EnvError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

### Builder Pattern
Use builder pattern for configurable objects:

```rust
impl ValidationRule {
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }
}
```

### Module Organization
- Keep modules focused on single responsibility
- Re-export frequently used types in lib.rs
- Use `pub mod` for public modules
- Document module purpose

### Testing Standards

Write tests for:
1. Happy path scenarios
2. Error conditions
3. Edge cases (empty files, malformed input)
4. Boundary conditions

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_env() {
        let content = "KEY=value\n";
        let result = EnvFile::parse(content);
        assert!(result.is_ok());
    }
}
```

## Cargo Best Practices

### Dependencies
- Pin major versions in Cargo.toml
- Run `cargo update` regularly
- Remove unused dependencies
- Use workspace for multi-crate projects

### Build Profiles
```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
```

## Documentation Requirements

For public APIs:
1. Use `///` for doc comments
2. Include examples in docs
3. Document panics and errors
4. Add `# Panics`, `# Errors` sections where relevant

Example:
```rust
/// Load and parse a .env file from disk
/// 
/// # Arguments
/// * `path` - Path to the .env file
/// 
/// # Returns
/// * `Ok(EnvFile)` - Successfully parsed file
/// * `Err(EnvError)` - File not found or parse error
/// 
/// # Example
/// ```
/// let env = EnvFile::load(".env")?;
/// ```
pub fn load<P: AsRef<Path>>(path: P) -> Result<Self>
```

## Performance Considerations

- Use `&str` instead of `String` for function parameters when possible
- Avoid unnecessary cloning
- Use iterators for lazy evaluation
- Profile before optimizing

## Common Patterns

### HashMap Operations
```rust
// Check and get value
if let Some(value) = map.get(key) {
    // use value
}

// Insert or modify
map.insert(key.clone(), value);

// Remove
map.remove(&key);
```

### File I/O
```rust
// Read entire file
let content = fs::read_to_string(path)?;

// Write file
fs::write(path, content)?;
```

## Validation Checklist for PRs

- [ ] Code compiles without warnings (`cargo build`)
- [ ] All tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Formatted properly (`cargo fmt`)
- [ ] Error types are descriptive
- [ ] Public functions have documentation
- [ ] New features include tests
- [ ] Dependencies are necessary
