# Env Manager - Setup & Usage Guide 🚀

A powerful CLI tool for managing .env files with features like validation, encryption, backup/restore, merging, and more.

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Command Reference](#command-reference)
5. [Examples](#examples)
6. [Advanced Features](#advanced-features)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software
- **Rust** (version 1.70 or higher)
  - Install from: https://rustup.rs/
  - Or check installation: `rustc --version`

### Optional (for development)
- Git (for version control)
- Code editor (VS Code, RustRover, etc.)

---

## Installation

### Option 1: Build from Source (Recommended)

1. **Navigate to the project directory:**
   ```bash
   cd env_manager
   ```

2. **Build in release mode:**
   ```bash
   cargo build --release
   ```

3. **The executable will be located at:**
   - Windows: `env_manager\target\release\env_manager.exe`
   - macOS/Linux: `env_manager/target/release/env_manager`

4. **(Optional) Add to your PATH:**
   
   **Windows (PowerShell):**
   ```powershell
   $env:Path += ";C:\path\to\env_manager\target\release"
   ```
   
   **macOS/Linux:**
   ```bash
   export PATH="/path/to/env_manager/target/release:$PATH"
   ```

### Option 2: Run Directly with Cargo

You can run the tool without building:
```bash
cargo run -- <command> [arguments]
```

---

## Quick Start

### 1. Create Your First .env File

```bash
# Initialize with a template
env_manager init
```

This creates a `.env` file with common variables.

### 2. View All Variables

```bash
env_manager list
```

### 3. Add a New Variable

```bash
env_manager set API_KEY "your-secret-key-here"
```

### 4. Validate Your Configuration

```bash
env_manager validate
```

### 5. Check for Issues

```bash
env_manager lint
```

---

## Command Reference

### Core Management Commands

#### `list` - Display all environment variables
```bash
env_manager list [--env <environment>]
```
**Options:**
- `--env, -e`: Environment name (default: local)

**Example:**
```bash
env_manager list --env production
```

#### `get` - Retrieve a specific variable
```bash
env_manager get <KEY> [--env <environment>]
```
**Example:**
```bash
env_manager get DB_HOST
```

#### `set` - Create or update a variable
```bash
env_manager set <KEY> <VALUE> [--comment "description"] [--env <environment>]
```
**Example:**
```bash
env_manager set DB_PASSWORD "supersecret" --comment "Database password"
```

#### `remove` - Delete a variable
```bash
env_manager remove <KEY> [--env <environment>]
```
**Example:**
```bash
env_manager remove OLD_API_KEY
```

---

### Validation & Quality Commands

#### `validate` - Check against validation rules
```bash
env_manager validate [--env <environment>]
```
Validates required fields, patterns, and value constraints.

#### `lint` - Code quality check
```bash
env_manager lint [--env <environment>]
```
Checks for:
- Naming convention violations
- Empty values
- Placeholder values
- Weak passwords
- Duplicate values

#### `check` - Reference validation
```bash
env_manager check [--env <environment>]
```
Detects:
- Broken references (variables referencing non-existent variables)
- Circular dependencies
- Unused variables

#### `stats` - Environment statistics
```bash
env_manager stats [--env <environment>]
```
Shows:
- Total variables and percentages
- Value length analysis
- Category breakdown
- Security insights

---

### Advanced Features

#### `backup` - Create timestamped backup
```bash
env_manager backup [--env <environment>]
```
Creates backup in `.env_backups/` directory.

#### `restore` - Restore from backup
```bash
# List available backups
env_manager restore --list [--env <environment>]

# Restore specific backup
env_manager restore --file <backup_filename> [--env <environment>]
```

#### `merge` - Combine two .env files
```bash
env_manager merge <env1> <env2> [--output <filename>] [--prefer-first]
```
**Example:**
```bash
env_manager merge local production --output .env.merged
```

#### `tree` - Show dependency tree
```bash
env_manager tree [--env <environment>]
```
Visualizes variable references like `${VAR}`.

#### `encrypt` - Encrypt sensitive values
```bash
env_manager encrypt <encryption_key> [--env <environment>]
```
Encrypts variables containing: password, secret, key, token, auth.

#### `decrypt` - Decrypt a value
```bash
env_manager decrypt <key> <encrypted_value>
```
**Example:**
```bash
env_manager decrypt mykey ENC:base64encodedvalue
```

#### `batch` - Bulk operations
```bash
# Delete multiple variables
env_manager batch delete KEY1 KEY2 KEY3 [--env <environment>]

# Update multiple variables
env_manager batch update "KEY1=value1" "KEY2=value2" [--env <environment>]

# Rename a variable
env_manager batch rename OLD_KEY NEW_KEY [--env <environment>]
```

#### `test` - Schema validation
```bash
env_manager test [--schema <schema_file>] [--env <environment>]
```
Runs validation tests and reports pass/fail status.

#### `interactive` - Interactive mode
```bash
env_manager interactive
```
Displays a menu-driven interface (basic implementation).

#### `theme` - Set color theme
```bash
# Show available themes
env_manager theme

# Set a theme
env_manager theme --name dark
```

#### `template` - Create from extended templates
```bash
env_manager template [--kind <type>]
```
**Available templates:**
- `nodejs` / `javascript`
- `python` / `django`
- `docker`
- `database` / `db`

**Example:**
```bash
env_manager template --kind python
```

---

### Utility Commands

#### `search` - Find variables
```bash
env_manager search <query> [--key] [--value] [--regex] [--env <environment>]
```
**Examples:**
```bash
# Search in both keys and values
env_manager search "localhost"

# Search only in keys
env_manager search "DB_" --key

# Use regex pattern
env_manager search "^API_" --regex
```

#### `format` - Organize .env file
```bash
env_manager format [--sort] [--group] [--check] [--env <environment>]
```
**Options:**
- `--sort`: Sort alphabetically
- `--group`: Group by category prefix
- `--check`: Only check if formatting needed

**Example:**
```bash
env_manager format --sort --group
```

#### `expand` - Expand variable references
```bash
env_manager expand [--env <environment>]
```
Replaces `${VAR}` with actual values.

#### `security` - Security scan
```bash
env_manager security
```
Scans all environments for sensitive variable names.

#### `diff` - Compare two environments
```bash
env_manager diff <env1> <env2>
```
**Example:**
```bash
env_manager diff local production
```

#### `export` - Export to different formats
```bash
env_manager export [--format <type>] [--output <file>] [--env <environment>]
```
**Supported formats:**
- `json` (default)
- `yaml`
- `shell` / `bash`
- `dotenv` / `env`

**Examples:**
```bash
# Export as JSON to console
env_manager export

# Export as shell script to file
env_manager export --format shell --output vars.sh

# Export production env as YAML
env_manager export --format yaml --env production
```

#### `all` - Display all environment files
```bash
env_manager all
```
Shows variables from all .env files at once.

#### `init` - Create new .env file
```bash
env_manager init [--env-type <type>]
```
Creates a basic template with common variables.

---

## Examples

### Common Workflows

#### 1. Setting Up a New Project
```bash
# Create .env from Node.js template
env_manager template --kind nodejs

# Review what was created
env_manager list

# Add missing variables
env_manager set JWT_SECRET "your-jwt-secret"
env_manager set REDIS_URL "redis://localhost:6379"

# Validate configuration
env_manager validate

# Create a backup
env_manager backup
```

#### 2. Preparing for Production Deployment
```bash
# Check for placeholder values
env_manager lint

# Verify no broken references
env_manager check

# View statistics
env_manager stats --env production

# Export for deployment
env_manager export --format shell --env production > deploy.env
```

#### 3. Merging Development and Production Configs
```bash
# Merge configs (production takes precedence)
env_manager merge local production --output .env.combined --prefer-first

# Review the merged result
env_manager list --env custom

# Format nicely
env_manager format --sort --group
```

#### 4. Cleaning Up Old Variables
```bash
# See statistics to identify unused vars
env_manager stats

# Check for unused variables
env_manager check

# Remove them in batch
env_manager batch delete UNUSED_VAR1 UNUSED_VAR2

# Verify changes
env_manager lint
```

#### 5. Securing Sensitive Data
```bash
# Identify sensitive variables
env_manager security

# Encrypt them (use a strong key!)
env_manager encrypt "your-encryption-key-123"

# Later, decrypt when needed
env_manager decrypt "your-encryption-key-123" ENC:base64value
```

---

## Advanced Features

### Variable References

You can reference variables within your .env file:

```bash
BASE_URL=http://localhost:3000
API_URL=${BASE_URL}/api/v1
DOCS_URL=$API_URL/docs
```

Use `env_manager expand` to resolve all references.

### Multiple Environments

Manage different environments easily:

```bash
# Work with production
env_manager list --env production
env_manager set DB_HOST "prod.example.com" --env production

# Compare environments
env_manager diff local production

# Backup all environments
env_manager backup --env local
env_manager backup --env staging
env_manager backup --env production
```

### Custom Environment Types

Create custom environment types:

```bash
env_manager set API_KEY "dev-key" --env development
env_manager set API_KEY "test-key" --env testing
env_manager set API_KEY "prod-key" --env production
```

File naming convention: `.env.<type>` (e.g., `.env.development`, `.env.testing`)

---

## Troubleshooting

### Common Issues

#### "File not found: .env"
**Solution:** The tool looks for `.env` in the current directory. Use `--path` to specify a different location:
```bash
env_manager list --path /path/to/project
```

#### "Validation failed"
**Solution:** Check which rule failed and fix it:
```bash
env_manager validate  # Shows specific errors
env_manager lint      # Shows all issues
```

#### "Broken reference detected"
**Solution:** A variable references another that doesn't exist:
```bash
env_manager check  # Identifies the broken reference
```
Fix by either creating the missing variable or updating the reference.

#### "Circular dependency found"
**Solution:** Variables form a loop (A→B→A):
```bash
env_manager tree  # Visualizes dependencies
```
Break the cycle by removing one reference.

#### Encryption/Decryption Not Working
**Notes:**
- Uses XOR-based encryption (demo purposes)
- Must use the same key for encrypt and decrypt
- For production, use a proper secrets manager

**Solution:** Ensure you're using the exact same encryption key.

### Getting Help

View all commands:
```bash
env_manager --help
```

View help for specific command:
```bash
env_manager <command> --help
```

---

## Best Practices

### ✅ Do's

1. **Always backup before major changes:**
   ```bash
   env_manager backup
   ```

2. **Run lint regularly:**
   ```bash
   env_manager lint
   ```

3. **Use meaningful comments:**
   ```bash
   env_manager set DB_HOST "localhost" --comment "Local database host"
   ```

4. **Validate before deployment:**
   ```bash
   env_manager validate && env_manager check
   ```

5. **Keep production secrets separate:**
   ```bash
   env_manager set API_KEY "prod-key" --env production
   ```

### ❌ Don'ts

1. **Don't commit .env files with real secrets to Git**
   - Add `.env*` to your `.gitignore`
   - Only commit `.env.example` with placeholder values

2. **Don't use weak encryption for production**
   - The built-in encryption is for demo purposes
   - Use proper secrets management in production

3. **Don't ignore validation errors**
   - Fix them immediately to prevent issues later

---

## Next Steps

Now that you're set up, explore:

- **All available commands:** `env_manager --help`
- **Feature roadmap:** See `beyond-env.md` for all implemented features
- **Source code:** Check out `env_manager/src/` to understand implementation

Happy coding! 🎉
