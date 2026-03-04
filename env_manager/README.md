# Env Manager 🦀

A powerful Rust-based command-line tool for managing `.env` files across multiple environments.

## Features ✨

- **Parse and Read .env Files**: Full support for key-value pairs, comments, quoted values, and special characters
- **Variable Expansion**: Support for `${VAR}` and `$VAR` syntax with chained expansion
- **Multiple Environment Types**: Built-in support for local, production, staging, and test environments
- **Validation**: Validate required fields, patterns, and value constraints
- **Security Scanning**: Automatically detect sensitive variable names (passwords, API keys, etc.)
- **Environment Comparison**: Compare variables between different environments
- **Template Generation**: Create new .env files with sensible defaults

## Installation

### From Source

```bash
git clone <repository-url>
cd env_manager
cargo build --release
```

The binary will be available at `target/release/env_manager`.

## Usage

### Basic Commands

#### List all variables
```bash
env_manager list
env_manager list --env production
```

#### Get a specific variable
```bash
env_manager get DB_HOST
env_manager get API_KEY --env production
```

#### Set a variable
```bash
env_manager set MY_VAR my_value
env_manager set SECRET_KEY abc123 --comment "API secret key"
```

#### Remove a variable
```bash
env_manager remove TEMP_VAR
```

### Advanced Commands

#### Validate environment file
```bash
env_manager validate
env_manager validate --env production
```

#### Expand variables
```bash
env_manager expand
```

This will expand all variable references like `${BASE_URL}` to their actual values.

#### Security report
```bash
env_manager security
```

Scans for potentially sensitive variable names and provides security recommendations.

#### Create template
```bash
env_manager init
env_manager init --env-type staging
```

Creates a new .env file with template configuration.

#### Compare environments
```bash
env_manager diff local production
env_manager diff staging production
```

Shows differences between two environment files.

#### View all environments
```bash
env_manager all
```

Displays all available environment files in the directory.

### Options

- `-p, --path <PATH>`: Specify the directory containing .env files (default: current directory)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## Supported .env File Features

### Key-Value Pairs
```env
DB_HOST=localhost
PORT=3000
```

### Comments
```env
# Database configuration
DB_HOST=localhost
```

### Quoted Values
```env
API_KEY="secret-key-123"
PASSWORD='my$password'
MESSAGE="Hello World"
```

### Variable Expansion
```env
BASE_URL=http://localhost:3000
API_URL=${BASE_URL}/api/v1
FULL_URL=$API_URL/users
```

### Special Characters
```env
DB_PASSWORD="my$ecreT#password!"
CONNECTION_STRING="host=localhost;port=5432"
```

## Environment Types

The tool recognizes these standard environment types:

- `.env` or `.env.local` - Local development
- `.env.production` - Production environment
- `.env.staging` - Staging/pre-production
- `.env.test` - Testing environment
- `.env.<custom>` - Custom environments

## Validation Rules

Built-in validation includes:

- Required fields (DB_HOST, DB_USER, etc.)
- Pattern matching (ports must be numeric)
- Value constraints (NODE_ENV must be development/production/staging/test)
- Length limits

## Security Features

The security scanner identifies variables that may contain sensitive data:

- Passwords (`PASSWORD`, `PASSWD`, `PWD`)
- Secrets (`SECRET`, `PRIVATE`, `TOKEN`)
- API Keys (`API_KEY`, `APIKEY`)
- Authentication (`AUTH`, `CREDENTIAL`)
- Access Keys (`ACCESS_KEY`, `SECRET_KEY`)

## Examples

### Example 1: Setting up a new project
```bash
# Create a template
env_manager init

# Add custom variables
env_manager set REDIS_URL redis://localhost:6379
env_manager set JWT_SECRET my-super-secret-key --comment "JWT signing secret"

# Validate
env_manager validate
```

### Example 2: Managing multiple environments
```bash
# View all environments
env_manager all

# Compare local and production
env_manager diff local production

# Check security across all environments
env_manager security
```

### Example 3: Working with variable expansion
```bash
# Create .env with variables
echo 'BASE_URL=http://localhost:3000' > .env
echo 'API_URL=${BASE_URL}/api/v1' >> .env

# Expand all variables
env_manager expand

# Verify
env_manager get API_URL
# Output: API_URL=http://localhost:3000/api/v1
```

## Building from Source

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build Commands
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with optimizations
cargo run --release -- help
```

## Dependencies

- `clap`: Command-line argument parsing
- `serde`: Serialization/deserialization
- `regex`: Regular expression matching for variable expansion
- `thiserror`: Error handling
- `colored`: Colored terminal output

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License

## Safety

This tool modifies .env files. Always:
- Backup important .env files before using
- Review changes before committing
- Never commit .env files with secrets to version control

---

Built with ❤️ using Rust
