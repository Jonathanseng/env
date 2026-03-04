# Env Manager - Quick Reference Guide

## 🚀 Common Commands

### Viewing Variables
```bash
# List all variables in current environment
env_manager list

# List variables from specific environment
env_manager list --env production
env_manager list -p staging

# Get a single variable
env_manager get DB_HOST
env_manager get API_KEY --env production

# View all environments at once
env_manager all
```

### Modifying Variables
```bash
# Set a variable
env_manager set KEY value

# Set with comment
env_manager set SECRET_KEY abc123 --comment "API secret"

# Remove a variable
env_manager remove TEMP_VAR

# Create new environment template
env_manager init
env_manager init --env-type staging
```

### Validation & Security
```bash
# Validate current environment
env_manager validate

# Validate specific environment
env_manager validate --env production

# Generate security report
env_manager security
```

### Advanced Operations
```bash
# Expand all variable references
env_manager expand

# Compare two environments
env_manager diff local production
env_manager diff staging production

# Work in different directory
env_manager --path /path/to/project list
```

## 📋 Environment Type Aliases

You can use these shorthand aliases:

| Full Name | Aliases |
|-----------|---------|
| `local` | `dev`, `development` |
| `production` | `prod` |
| `staging` | `stage` |
| `test` | (none) |

Examples:
```bash
env_manager list --env dev      # Same as local
env_manager list --env prod     # Same as production
env_manager list --env stage    # Same as staging
```

## 🔧 File Format Support

### Basic Syntax
```env
# This is a comment
KEY=value
PORT=3000
```

### Quoted Values
```env
STRING="hello world"
SINGLE='value with spaces'
SPECIAL="contains = and # symbols"
```

### Variable Expansion
```env
BASE_URL=http://localhost:3000
API_URL=${BASE_URL}/api/v1
FULL_URL=$API_URL/users
```

### Special Characters
```env
PASSWORD="my$ecret#pass!"
CONNECTION="host=localhost;port=5432"
JSON='{"key": "value"}'
```

## 💡 Pro Tips

### 1. Template Workflow
```bash
# Start with template
env_manager init

# Add your variables
env_manager set REDIS_URL redis://localhost:6379
env_manager set DATABASE_URL postgres://localhost/mydb

# Validate
env_manager validate

# Check security
env_manager security
```

### 2. Multi-Environment Workflow
```bash
# Setup all environments
env_manager init --env-type staging

# Configure each
env_manager set DB_HOST localhost --env local
env_manager set DB_HOST prod-db.example.com --env production

# Compare them
env_manager diff local production
```

### 3. Security Best Practices
```bash
# Always check for sensitive data
env_manager security

# Never commit .env files
git add .env  # ❌ Don't do this!
git add .env.example  # ✅ Do this instead
```

## ⚙️ Configuration Options

### Command-Line Options
```bash
env_manager [OPTIONS] <COMMAND>

Options:
  -p, --path <PATH>    Directory with .env files (default: .)
  -h, --help           Print help
  -V, --version        Print version
```

### Supported File Names
```
.env                    # Default/local
.env.local              # Local development
.env.development        # Development
.env.test               # Testing
.env.staging            # Staging
.env.production         # Production
.env.prod               # Production (alias)
.env.<custom>           # Custom environments
```

## 🐛 Troubleshooting

### Variable Not Expanding?
```bash
# Make sure referenced variable exists
env_manager get BASE_URL

# Run expand command
env_manager expand

# Check result
env_manager get API_URL
```

### Validation Failing?
```bash
# Check required fields
env_manager list | findstr "DB_HOST"

# Verify patterns (e.g., PORT must be numeric)
env_manager get PORT

# Check allowed values
env_manager get NODE_ENV  # Must be: development/production/staging/test
```

### Security Warnings?
```bash
# View full security report
env_manager security

# Review flagged variables
# These are informational - not errors
# Just ensure they're in .gitignore
```

## 📊 Example Session

```bash
# Initialize project
$ env_manager init
✓ Created .env with template

# Add custom variables
$ env_manager set REDIS_HOST localhost
✓ Set REDIS_HOST=localhost

$ env_manager set REDIS_PORT 6379 --comment "Redis port"
✓ Set REDIS_PORT=6379

# View all
$ env_manager list

Environment Variables (local)
==================================================
  # Application port
  PORT=3000
  # Database host
  DB_HOST=localhost
  # Redis port
  REDIS_PORT=6379
  REDIS_HOST=localhost
  ...

# Validate
$ env_manager validate
✓ Validation passed!

# Security check
$ env_manager security

🔒 Security Report
==================================================
⚠ Found 2 potentially sensitive variable(s):
  - DB_PASSWORD
  - API_KEY

Recommendations:
  • Ensure these values are in .gitignore
  • Consider using a secrets manager in production
  • Use strong, unique values for each environment
```

## 🎯 Quick Comparison

| Task | Command |
|------|---------|
| See what you have | `env_manager list` |
| Find specific value | `env_manager get KEY` |
| Add new variable | `env_manager set KEY value` |
| Remove variable | `env_manager remove KEY` |
| Check correctness | `env_manager validate` |
| Resolve references | `env_manager expand` |
| Security audit | `env_manager security` |
| Compare configs | `env_manager diff env1 env2` |
| See everything | `env_manager all` |
| Start fresh | `env_manager init` |

---

**Need more help?** Run `env_manager --help` or `env_manager <command> --help`
