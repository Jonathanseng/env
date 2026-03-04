use crate::{EnvError, EnvFile, Result};
use std::collections::HashMap;
use colored::Colorize;

/// Validation rule for environment variables
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub key: String,
    pub required: bool,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub allowed_values: Option<Vec<String>>,
}

impl ValidationRule {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            required: false,
            pattern: None,
            min_length: None,
            max_length: None,
            allowed_values: None,
        }
    }
    
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    pub fn pattern(mut self, pattern: &str) -> Self {
        self.pattern = Some(pattern.to_string());
        self
    }
    
    pub fn min_length(mut self, len: usize) -> Self {
        self.min_length = Some(len);
        self
    }
    
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }
    
    pub fn allowed_values(mut self, values: Vec<&str>) -> Self {
        self.allowed_values = Some(values.iter().map(|s| s.to_string()).collect());
        self
    }
}

/// Validator for .env files
#[derive(Debug)]
pub struct EnvValidator {
    rules: HashMap<String, ValidationRule>,
}

impl EnvValidator {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    
    /// Add a validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.insert(rule.key.clone(), rule);
    }
    
    /// Validate an EnvFile against all rules
    pub fn validate(&self, env: &EnvFile) -> Result<()> {
        let mut errors = Vec::new();
        
        // Check required variables
        for (key, rule) in &self.rules {
            if rule.required && !env.contains_key(key) {
                errors.push(EnvError::MissingVariable(key.clone()));
                continue;
            }
            
            if let Some(value) = env.get(key) {
                // Validate length
                if let Some(min_len) = rule.min_length {
                    if value.len() < min_len {
                        errors.push(EnvError::ParseError {
                            line: 0,
                            message: format!(
                                "Value for '{}' is too short (min {} characters)",
                                key, min_len
                            ),
                        });
                    }
                }
                
                if let Some(max_len) = rule.max_length {
                    if value.len() > max_len {
                        errors.push(EnvError::ParseError {
                            line: 0,
                            message: format!(
                                "Value for '{}' is too long (max {} characters)",
                                key, max_len
                            ),
                        });
                    }
                }
                
                // Validate pattern
                if let Some(ref pattern) = rule.pattern {
                    let regex = regex::Regex::new(pattern).unwrap();
                    if !regex.is_match(value) {
                        errors.push(EnvError::ParseError {
                            line: 0,
                            message: format!("Value for '{}' does not match pattern", key),
                        });
                    }
                }
                
                // Validate allowed values
                if let Some(ref allowed) = rule.allowed_values {
                    if !allowed.contains(value) {
                        errors.push(EnvError::ParseError {
                            line: 0,
                            message: format!(
                                "Value for '{}' must be one of: {}",
                                key,
                                allowed.join(", ")
                            ),
                        });
                    }
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.into_iter().next().unwrap())
        }
    }
    
    /// Create common validation rules for typical environments
    pub fn with_common_rules() -> Self {
        let mut validator = Self::new();
        
        // Database rules
        validator.add_rule(
            ValidationRule::new("DB_HOST").required()
        );
        validator.add_rule(
            ValidationRule::new("DB_PORT")
                .pattern(r"^\d+$")
        );
        validator.add_rule(
            ValidationRule::new("DB_USER").required()
        );
        
        // Environment mode
        validator.add_rule(
            ValidationRule::new("NODE_ENV")
                .allowed_values(vec!["development", "production", "staging", "test"])
        );
        
        // Port numbers
        validator.add_rule(
            ValidationRule::new("PORT")
                .pattern(r"^\d+$")
                .min_length(1)
                .max_length(5)
        );
        
        validator
    }
}

impl Default for EnvValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Security scanner for sensitive data
pub struct SecurityScanner;

impl SecurityScanner {
    /// Check for potentially sensitive variable names that might need extra protection
    pub fn scan_for_secrets(env: &EnvFile) -> Vec<String> {
        let sensitive_patterns = [
            "password", "passwd", "pwd",
            "secret", "private", "token",
            "api_key", "apikey", "api-key",
            "auth", "credential", "cred",
            "access_key", "secret_key",
        ];
        
        let mut sensitive_keys = Vec::new();
        
        for key in env.keys() {
            let key_lower = key.to_lowercase();
            for pattern in &sensitive_patterns {
                if key_lower.contains(pattern) {
                    sensitive_keys.push(key.clone());
                    break;
                }
            }
        }
        
        sensitive_keys
    }
    
    /// Generate security recommendations
    pub fn generate_security_report(env: &EnvFile) -> String {
        let sensitive_keys = Self::scan_for_secrets(env);
        
        let mut report = String::from("🔒 Security Report\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');
        
        if sensitive_keys.is_empty() {
            report.push_str("✓ No obviously sensitive variable names detected\n");
        } else {
            report.push_str(&format!(
                "⚠ Found {} potentially sensitive variable(s):\n\n",
                sensitive_keys.len()
            ));
            
            for key in &sensitive_keys {
                report.push_str(&format!("  - {}\n", key));
            }
            
            report.push_str("\nRecommendations:\n");
            report.push_str("  • Ensure these values are in .gitignore\n");
            report.push_str("  • Consider using a secrets manager in production\n");
            report.push_str("  • Use strong, unique values for each environment\n");
        }
        
        report
    }
}

/// Reference checker for detecting broken references and circular dependencies
pub struct ReferenceChecker;

impl ReferenceChecker {
    /// Find all variable references in a value
    fn find_references(value: &str) -> Vec<String> {
        let regex = regex::Regex::new(r"\$\{([^}]+)\}|\$([A-Za-z_][A-Za-z0-9_]*)").unwrap();
        regex.captures_iter(value)
            .filter_map(|cap| {
                cap.get(1).or(cap.get(2)).map(|m| m.as_str().to_string())
            })
            .collect()
    }

    /// Check for broken references (references to non-existent variables)
    pub fn check_broken_references(env: &EnvFile) -> Vec<(String, String)> {
        let mut broken_refs = Vec::new();
        
        for (key, entry) in &env.entries {
            let refs = Self::find_references(&entry.value);
            for referenced_var in refs {
                if !env.contains_key(&referenced_var) {
                    broken_refs.push((key.clone(), referenced_var));
                }
            }
        }
        
        broken_refs
    }

    /// Check for circular dependencies
    pub fn check_circular_dependencies(env: &EnvFile) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        
        fn detect_cycle(
            key: &str,
            env: &EnvFile,
            visited: &mut std::collections::HashSet<String>,
            rec_stack: &mut std::collections::HashSet<String>,
            path: &mut Vec<String>,
            cycles: &mut Vec<Vec<String>>,
        ) {
            visited.insert(key.to_string());
            rec_stack.insert(key.to_string());
            path.push(key.to_string());
            
            if let Some(entry) = env.entries.get(key) {
                let refs = ReferenceChecker::find_references(&entry.value);
                for ref_var in refs {
                    if !visited.contains(&ref_var) {
                        detect_cycle(&ref_var, env, visited, rec_stack, path, cycles);
                    } else if rec_stack.contains(&ref_var) {
                        // Found a cycle
                        let cycle_start = path.iter().position(|x| x == &ref_var).unwrap();
                        let mut cycle = path[cycle_start..].to_vec();
                        cycle.push(ref_var.clone());
                        cycles.push(cycle);
                    }
                }
            }
            
            path.pop();
            rec_stack.remove(key);
        }
        
        for key in env.keys() {
            if !visited.contains(key) {
                let mut path = Vec::new();
                detect_cycle(
                    key,
                    env,
                    &mut visited,
                    &mut rec_stack,
                    &mut path,
                    &mut cycles,
                );
            }
        }
        
        cycles
    }

    /// Check for unused variables (defined but never referenced)
    pub fn check_unused_variables(env: &EnvFile) -> Vec<String> {
        let mut referenced_vars = std::collections::HashSet::new();
        
        // Collect all referenced variables
        for entry in env.entries.values() {
            let refs = Self::find_references(&entry.value);
            for ref_var in refs {
                referenced_vars.insert(ref_var);
            }
        }
        
        // Find variables that are never referenced
        let mut unused = Vec::new();
        for key in env.keys() {
            if !referenced_vars.contains(key) {
                // Check if it's a "root" variable (common patterns that don't need to be referenced)
                let is_root = key == "NODE_ENV" || key == "PORT" || key == "DEBUG" 
                    || key.starts_with("DB_") || key.starts_with("API_") 
                    || key.starts_with("MAIL_");
                
                if !is_root {
                    unused.push(key.clone());
                }
            }
        }
        
        unused
    }

    /// Generate comprehensive reference report
    pub fn generate_reference_report(env: &EnvFile) -> String {
        let mut report = String::from("🔍 Reference Check Report\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');
        
        // Broken references
        let broken = Self::check_broken_references(env);
        if broken.is_empty() {
            report.push_str("✓ No broken references found\n");
        } else {
            report.push_str(&format!(
                "✗ Found {} broken reference(s):\n\n",
                broken.len()
            ));
            for (key, ref_var) in &broken {
                report.push_str(&format!("  - {} references {} (not found)\n", key.red(), ref_var.yellow()));
            }
            report.push('\n');
        }
        
        // Circular dependencies
        let cycles = Self::check_circular_dependencies(env);
        if cycles.is_empty() {
            report.push_str("✓ No circular dependencies found\n");
        } else {
            report.push_str(&format!(
                "✗ Found {} circular dependency chain(s):\n\n",
                cycles.len()
            ));
            for cycle in &cycles {
                report.push_str(&format!("  - {}\n", cycle.join(" → ").red()));
            }
            report.push('\n');
        }
        
        // Unused variables
        let unused = Self::check_unused_variables(env);
        if unused.is_empty() {
            report.push_str("✓ All variables are used or are root variables\n");
        } else {
            report.push_str(&format!(
                "ℹ Found {} potentially unused variable(s):\n\n",
                unused.len()
            ));
            for key in &unused {
                report.push_str(&format!("  - {} (consider removing if not needed)\n", key.yellow()));
            }
            report.push('\n');
        }
        
        report
    }
}

/// Linter for checking code quality and best practices
pub struct EnvLinter;

impl EnvLinter {
    /// Check for naming convention issues
    pub fn check_naming_conventions(env: &EnvFile) -> Vec<(String, String)> {
        let mut issues = Vec::new();
        
        for key in env.keys() {
            // Check if key is uppercase with underscores (standard for env vars)
            if !key.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric()) {
                issues.push((
                    key.clone(),
                    "Variable names should be UPPERCASE_WITH_UNDERSCORES".to_string(),
                ));
            }
            
            // Check for common typos
            let lower_key = key.to_lowercase();
            if lower_key.contains("pasword") {
                issues.push((key.clone(), "Possible typo: 'pasword' should be 'password'".to_string()));
            }
            if lower_key.contains("secrect") {
                issues.push((key.clone(), "Possible typo: 'secrect' should be 'secret'".to_string()));
            }
        }
        
        issues
    }

    /// Check for duplicate values
    pub fn check_duplicate_values(env: &EnvFile) -> Vec<(String, String)> {
        let mut value_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for (key, entry) in &env.entries {
            // Skip empty values and very short values (likely to be duplicates by chance)
            if entry.value.len() > 3 {
                value_map
                    .entry(entry.value.clone())
                    .or_default()
                    .push(key.clone());
            }
        }
        
        let mut duplicates = Vec::new();
        for (value, keys) in value_map {
            if keys.len() > 1 {
                duplicates.push((keys.join(", "), format!("Duplicate value: '{}'", value)));
            }
        }
        
        duplicates
    }

    /// Check for empty values
    pub fn check_empty_values(env: &EnvFile) -> Vec<String> {
        env.entries
            .iter()
            .filter(|(_, entry)| entry.value.is_empty())
            .map(|(key, _)| key.clone())
            .collect()
    }

    /// Check for placeholder values that should be replaced
    pub fn check_placeholder_values(env: &EnvFile) -> Vec<(String, String)> {
        let placeholder_patterns = [
            "your_", "changeme", "xxx", "yyy", "zzz", "todo", "fixme",
            "placeholder", "example", "replace_me", "insert_",
        ];
        
        let mut placeholders = Vec::new();
        
        for (key, entry) in &env.entries {
            let lower_value = entry.value.to_lowercase();
            for pattern in &placeholder_patterns {
                if lower_value.contains(pattern) {
                    placeholders.push((
                        key.clone(),
                        format!("Placeholder value detected: '{}'", entry.value),
                    ));
                    break;
                }
            }
        }
        
        placeholders
    }

    /// Check for weak passwords
    pub fn check_weak_passwords(env: &EnvFile) -> Vec<(String, String)> {
        let sensitive_patterns = ["password", "passwd", "pwd", "secret"];
        let weak_passwords = ["password", "123456", "admin", "default", "changeme", "password123"];
        
        let mut weak = Vec::new();
        
        for (key, entry) in &env.entries {
            let lower_key = key.to_lowercase();
            if sensitive_patterns.iter().any(|p| lower_key.contains(p)) {
                let lower_value = entry.value.to_lowercase();
                if weak_passwords.iter().any(|w| lower_value == *w) || entry.value.len() < 8 {
                    weak.push((
                        key.clone(),
                        "Weak password detected (too short or common)".to_string(),
                    ));
                }
            }
        }
        
        weak
    }

    /// Generate comprehensive lint report
    pub fn generate_lint_report(env: &EnvFile) -> String {
        let mut report = String::from("📝 Lint Report\n");
        report.push_str(&"=".repeat(50));
        report.push('\n');
        
        let mut total_issues = 0;
        
        // Naming conventions
        let naming_issues = Self::check_naming_conventions(env);
        if !naming_issues.is_empty() {
            report.push_str(&format!(
                "\n{} Naming Convention Issues ({})\n",
                "⚠️".yellow(),
                naming_issues.len()
            ));
            report.push_str(&"-".repeat(40));
            report.push('\n');
            for (key, issue) in &naming_issues {
                report.push_str(&format!("  • {}: {}\n", key.yellow(), issue));
            }
            total_issues += naming_issues.len();
        } else {
            report.push_str("\n✓ No naming convention issues\n");
        }
        
        // Empty values
        let empty_values = Self::check_empty_values(env);
        if !empty_values.is_empty() {
            report.push_str(&format!(
                "\n{} Empty Values ({})\n",
                "⚠️".yellow(),
                empty_values.len()
            ));
            report.push_str(&"-".repeat(40));
            report.push('\n');
            for key in &empty_values {
                report.push_str(&format!("  • {} has no value\n", key.yellow()));
            }
            total_issues += empty_values.len();
        } else {
            report.push_str("\n✓ No empty values");
            report.push('\n');
        }
        
        // Placeholder values
        let placeholders = Self::check_placeholder_values(env);
        if !placeholders.is_empty() {
            report.push_str(&format!(
                "\n{} Placeholder Values ({})\n",
                "⚠️".yellow(),
                placeholders.len()
            ));
            report.push_str(&"-".repeat(40));
            report.push('\n');
            for (key, issue) in &placeholders {
                report.push_str(&format!("  • {}: {}\n", key.yellow(), issue));
            }
            total_issues += placeholders.len();
        } else {
            report.push_str("\n✓ No placeholder values detected");
            report.push('\n');
        }
        
        // Weak passwords
        let weak_passwords = Self::check_weak_passwords(env);
        if !weak_passwords.is_empty() {
            report.push_str(&format!(
                "\n{} Security Warnings - Weak Passwords ({})\n",
                "🔴".red(),
                weak_passwords.len()
            ));
            report.push_str(&"-".repeat(40));
            report.push('\n');
            for (key, issue) in &weak_passwords {
                report.push_str(&format!("  • {}: {}\n", key.red(), issue));
            }
            total_issues += weak_passwords.len();
        } else {
            report.push_str("\n✓ No weak passwords detected");
            report.push('\n');
        }
        
        // Duplicate values
        let duplicates = Self::check_duplicate_values(env);
        if !duplicates.is_empty() {
            report.push_str(&format!(
                "\nℹ️ Duplicate Values ({})\n",
                duplicates.len()
            ));
            report.push_str(&"-".repeat(40));
            report.push('\n');
            for (keys, issue) in &duplicates {
                report.push_str(&format!("  • {}: {}\n", keys.blue(), issue));
            }
            report.push_str("  (This might be intentional)\n");
        } else {
            report.push_str("\n✓ No duplicate values");
            report.push('\n');
        }
        
        // Summary
        report.push('\n');
        report.push_str(&"=".repeat(50));
        report.push('\n');
        if total_issues == 0 {
            report.push_str(&format!("{} Lint passed! No issues found\n", "✓".green()));
        } else {
            report.push_str(&format!(
                "{} Found {} issue(s) that may need attention\n",
                "Σ".yellow(),
                total_issues
            ));
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validator_required_fields() {
        let content = r#"
DB_HOST=localhost
DB_USER=admin
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        validator.add_rule(ValidationRule::new("DB_HOST").required());
        validator.add_rule(ValidationRule::new("DB_PASSWORD").required());
        
        // Should fail because DB_PASSWORD is missing
        assert!(validator.validate(&env).is_err());
    }
    
    #[test]
    fn test_validator_pattern() {
        let content = r#"
PORT=3000
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        validator.add_rule(ValidationRule::new("PORT").pattern(r"^\d+$"));
        
        assert!(validator.validate(&env).is_ok());
    }
    
    #[test]
    fn test_security_scanner() {
        let content = r#"
DB_PASSWORD=secret123
API_KEY=key-abc-123
NORMAL_VAR=value
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let sensitive = SecurityScanner::scan_for_secrets(&env);
        
        assert_eq!(sensitive.len(), 2);
        assert!(sensitive.contains(&"DB_PASSWORD".to_string()));
        assert!(sensitive.contains(&"API_KEY".to_string()));
    }

    #[test]
    fn test_validation_rule_builder() {
        let rule = ValidationRule::new("TEST_KEY")
            .required()
            .pattern(r"^[A-Z]+$")
            .min_length(5)
            .max_length(10)
            .allowed_values(vec!["VALUE1", "VALUE2"]);
        
        assert!(rule.required);
        assert_eq!(rule.pattern, Some(r"^[A-Z]+$".to_string()));
        assert_eq!(rule.min_length, Some(5));
        assert_eq!(rule.max_length, Some(10));
        assert!(rule.allowed_values.is_some());
    }

    #[test]
    fn test_validator_min_max_length() {
        let content = r#"
SHORT=ab
LONG=abcdefghijk
GOOD=abcde
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        
        validator.add_rule(ValidationRule::new("SHORT").min_length(3));
        validator.add_rule(ValidationRule::new("LONG").max_length(10));
        validator.add_rule(ValidationRule::new("GOOD").min_length(3).max_length(10));
        
        // SHORT should pass (length 2 < 3)
        assert!(validator.validate(&env).is_err());
    }

    #[test]
    fn test_validator_allowed_values() {
        let content = r#"
ENV=production
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        
        validator.add_rule(
            ValidationRule::new("ENV")
                .allowed_values(vec!["development", "staging", "production"])
        );
        
        assert!(validator.validate(&env).is_ok());
    }

    #[test]
    fn test_validator_invalid_allowed_value() {
        let content = r#"
ENV=invalid_env
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        
        validator.add_rule(
            ValidationRule::new("ENV")
                .allowed_values(vec!["development", "staging", "production"])
        );
        
        assert!(validator.validate(&env).is_err());
    }

    #[test]
    fn test_validator_multiple_rules() {
        let content = r#"
PORT=3000
HOST=localhost
DEBUG=true
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        
        validator.add_rule(ValidationRule::new("PORT").required().pattern(r"^\d+$"));
        validator.add_rule(ValidationRule::new("HOST").required());
        validator.add_rule(ValidationRule::new("DEBUG").required());
        
        assert!(validator.validate(&env).is_ok());
    }

    #[test]
    fn test_validator_missing_multiple() {
        let content = r#"
PORT=3000
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let mut validator = EnvValidator::new();
        
        validator.add_rule(ValidationRule::new("PORT").required());
        validator.add_rule(ValidationRule::new("HOST").required());
        validator.add_rule(ValidationRule::new("DEBUG").required());
        
        let result = validator.validate(&env);
        assert!(result.is_err());
    }

    #[test]
    fn test_security_scanner_empty() {
        let content = r#"
NORMAL_VAR=value
ANOTHER_VAR=test
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let sensitive = SecurityScanner::scan_for_secrets(&env);
        
        assert_eq!(sensitive.len(), 0);
    }

    #[test]
    fn test_security_scanner_common_patterns() {
        let content = r#"
AWS_SECRET_KEY=aws_secret_123
PRIVATE_KEY=my_private_key
DATABASE_PASSWORD=db_pass
JWT_SECRET=jwt_secret_xyz
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let sensitive = SecurityScanner::scan_for_secrets(&env);
        
        assert_eq!(sensitive.len(), 4);
        assert!(sensitive.contains(&"AWS_SECRET_KEY".to_string()));
        assert!(sensitive.contains(&"PRIVATE_KEY".to_string()));
        assert!(sensitive.contains(&"DATABASE_PASSWORD".to_string()));
        assert!(sensitive.contains(&"JWT_SECRET".to_string()));
    }

    #[test]
    fn test_linter_naming_conventions() {
        let content = r#"
valid_var=value
INVALID-VAR=value2
another_invalid=value3
GOOD_VAR=test
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let issues = EnvLinter::check_naming_conventions(&env);
        
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|(k, _)| k == "valid_var"));
        assert!(issues.iter().any(|(k, _)| k == "INVALID-VAR"));
        assert!(issues.iter().any(|(k, _)| k == "another_invalid"));
    }

    #[test]
    fn test_linter_duplicate_values() {
        let content = r#"
VAR1=same_value
VAR2=different_value
VAR3=same_value
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let duplicates = EnvLinter::check_duplicate_values(&env);
        
        // Should detect duplicate values
        assert!(!duplicates.is_empty());
    }

    #[test]
    fn test_reference_checker_circular_dependency() {
        let content = r#"
VAR_A=${VAR_B}
VAR_B=${VAR_C}
VAR_C=${VAR_A}
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let circular = ReferenceChecker::check_circular_dependencies(&env);
        
        assert!(!circular.is_empty());
    }

    #[test]
    fn test_reference_checker_broken_reference() {
        let content = r#"
VALID_VAR=value
BROKEN_VAR=${NON_EXISTENT}
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let broken = ReferenceChecker::check_broken_references(&env);
        
        assert!(!broken.is_empty());
        assert!(broken.iter().any(|(k, _)| k == "BROKEN_VAR"));
    }

    #[test]
    fn test_reference_checker_valid_references() {
        let content = r#"
BASE_URL=https://example.com
API_URL=${BASE_URL}/api
FULL_URL=${API_URL}/v1
"#;
        
        let env = EnvFile::parse(content).unwrap();
        
        let circular = ReferenceChecker::check_circular_dependencies(&env);
        let broken = ReferenceChecker::check_broken_references(&env);
        
        assert!(circular.is_empty());
        assert!(broken.is_empty());
    }

    #[test]
    fn test_reference_checker_simple_chain() {
        let content = r#"
A=1
B=${A}
C=${B}
D=${C}
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let broken = ReferenceChecker::check_broken_references(&env);
        let circular = ReferenceChecker::check_circular_dependencies(&env);
        
        assert!(broken.is_empty());
        assert!(circular.is_empty());
    }

    #[test]
    fn test_security_scanner_weak_passwords() {
        let content = r#"
DB_PASSWORD=password
API_KEY=123456
SECRET=secret
ADMIN_PASS=admin
"#;
        
        let env = EnvFile::parse(content).unwrap();
        let sensitive = SecurityScanner::scan_for_secrets(&env);
        
        // Should detect at least 3 sensitive variables
        assert!(sensitive.len() >= 3);
    }
}
