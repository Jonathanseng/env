# New Features Added! 🎉

## Three Awesome Features Just Added to Env Manager

Think of your env_manager like a Swiss Army knife 🔪 - it just got three new blades!

---

## 1. 📤 Export Feature - Share Your Configs Easily

### What It Does
Exports your environment variables to different formats, like photocopying your sticker collection to share with friends!

### Supported Formats
- **JSON** - Perfect for web apps and APIs
- **YAML** - Great for configuration files
- **Shell/Bash** - Use in scripts or terminal
- **Dotenv** - Standard .env format

### How to Use

```bash
# Export to JSON (shows on screen)
env_manager export --format json

# Export to YAML
env_manager export --format yaml

# Export to shell script
env_manager export --format shell

# Export and save to file
env_manager export --format json --output my-config.json

# Export specific environment
env_manager export --format yaml --env production
```

### Example Outputs

**JSON Format:**
```json
{
  "DB_HOST": "localhost",
  "DB_PORT": "5432",
  "API_KEY": "your-api-key-here"
}
```

**YAML Format:**
```yaml
---
DB_HOST: localhost
DB_PORT: "5432"
API_KEY: your-api-key-here
```

**Shell Format:**
```bash
#!/bin/bash
export DB_HOST="localhost"
export DB_PORT="5432"
export API_KEY="your-api-key-here"
```

---

## 2. 🔍 Search Feature - Find Variables Fast

### What It Does
Helps you find specific variables quickly, like using the "Find" function in a book!

### Search Options
- Search in **keys** (variable names)
- Search in **values** (variable contents)
- Search in **both** (default)
- Use **regular expressions** for advanced patterns

### How to Use

```bash
# Search everywhere (keys and values)
env_manager search "localhost"

# Search only in keys
env_manager search "DB" --key

# Search only in values
env_manager search "secret" --value

# Use regex pattern (find all DB_ variables)
env_manager search "^DB_" --regex

# Case-insensitive search (built-in)
env_manager search "mail"
```

### Examples

**Find all database variables:**
```bash
$ env_manager search "DB"

🔍 Searching for 'DB'
==================================================

  # Database Configuration
  DB_HOST=localhost

  DB_USER=admin

  DB_PASSWORD=supersecretpassword

  DB_PORT=5432

  DB_NAME=myapp_db

Σ Found 5 match(es)
```

**Find all variables starting with MAIL_:**
```bash
$ env_manager search "^MAIL_" --regex

🔍 Searching for '^MAIL_'
==================================================

  MAIL_HOST=smtp.mailtrap.io
  MAIL_PORT=587
  MAIL_USER=your-mail-username
  MAIL_PASS=your-mail-password

Σ Found 4 match(es)
```

**Find variables containing "localhost":**
```bash
$ env_manager search "localhost" --value

🔍 Searching for 'localhost'
==================================================

  DB_HOST=localhost
  BASE_URL=http://localhost:3000
  API_URL=http://localhost:${PORT}/api/v1

Σ Found 3 match(es)
```

---

## 3. 🎨 Format Feature - Keep Things Tidy

### What It Does
Organizes your .env file neatly, like arranging your toys in labeled boxes!

### Formatting Options
- **Sort** - Alphabetical order
- **Group** - Group by category (DB_, API_, MAIL_, etc.)
- **Check** - See if formatting is needed without changing

### How to Use

```bash
# Sort alphabetically
env_manager format --sort

# Group by category
env_manager format --group

# Both sort and group
env_manager format --sort --group

# Check if already formatted
env_manager format --check
```

### Examples

**Before Formatting:**
```env
PORT=3000
MAIL_PORT=587
NODE_ENV=development
DB_HOST=localhost
API_KEY=abc123
DB_PORT=5432
```

**After `env_manager format --sort`:**
```env
API_KEY=abc123
DB_HOST=localhost
DB_PORT=5432
MAIL_PORT=587
NODE_ENV=development
PORT=3000
```

**After `env_manager format --group`:**
```env
API_KEY=abc123
DB_HOST=localhost
DB_PORT=5432
MAIL_PORT=587
NODE_ENV=development
PORT=3000
```

**Check Mode:**
```bash
$ env_manager format --check
✓ File is already formatted

# OR

✗ File needs formatting
  Run without --check to format
```

---

## 🎯 Cool Combinations

### Export + Search = Super Powers!

**Find all secrets and export them:**
```bash
# Search for sensitive data
env_manager search "KEY|SECRET|PASSWORD" --regex

# Then export to JSON for backup
env_manager export --format json --output backup.json
```

**Format for production deployment:**
```bash
# Group variables nicely
env_manager format --group

# Export as shell script for deployment
env_manager export --format shell --output deploy.sh
```

---

## 📊 Quick Command Reference

| Command | What It Does | Example |
|---------|--------------|---------|
| `export` | Convert to different formats | `env_manager export --format json` |
| `search` | Find variables quickly | `env_manager search "DB" --key` |
| `format` | Organize your .env file | `env_manager format --group` |

---

## 🚀 Real-World Use Cases

### 1. Sharing Configs with Team
```bash
# Export clean JSON for developers
env_manager export --format json --output team-config.json

# They can import and use it
```

### 2. Deployment Scripts
```bash
# Create shell script for server
env_manager export --format shell --output production-vars.sh

# Source it in deployment
source production-vars.sh
```

### 3. Finding Security Issues
```bash
# Find all passwords
env_manager search "PASSWORD|PASSWD|PWD" --regex

# Find all API keys
env_manager search "API.*KEY|SECRET" --regex
```

### 4. Cleaning Up Messy Files
```bash
# Check if messy
env_manager format --check

# Tidy it up!
env_manager format --sort --group
```

---

## 💡 Pro Tips

1. **Use search before you add** - Avoid duplicates!
   ```bash
   env_manager search "NEW_VAR"
   ```

2. **Export before big changes** - Safety first!
   ```bash
   env_manager export --format json --output backup-before-change.json
   ```

3. **Format regularly** - Keep things clean!
   ```bash
   env_manager format --group
   ```

4. **Combine with grep** - Even more powerful!
   ```bash
   env_manager search "localhost" | grep -i "port"
   ```

---

## 🎮 Try These Fun Commands!

```bash
# Find everything database-related
env_manager search "DB|DATABASE|POSTGRES|MYSQL" --regex

# Export production config as YAML
env_manager export --format yaml --env production

# Make your local file super organized
env_manager format --sort --group

# Check what has "secret" in the name
env_manager search "secret" --key
```

---

## ✅ All Features Working!

All three new features are:
- ✅ Built and compiled
- ✅ Tested and working
- ✅ Ready to use
- ✅ Documented here!

---

**Your env_manager just got THREE times more powerful!** 🚀

Think of it like upgrading from a regular bike to a turbo-charged racing bike! 🚴‍♂️💨
