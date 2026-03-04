# Code Review Agent

This agent performs comprehensive code reviews on pull requests and code changes.

## Capabilities

- Analyzes code quality and best practices
- Checks for potential bugs and security issues
- Reviews code style and consistency
- Suggests improvements and optimizations
- Validates error handling patterns

## Usage

When activated, this agent will:
1. Examine the changed files in a pull request
2. Analyze each change for potential issues
3. Provide constructive feedback with specific suggestions
4. Flag critical issues that need immediate attention
5. Acknowledge good practices and clean code

## Configuration

```json
{
  "enabled": true,
  "auto_review_pr": true,
  "severity_levels": ["critical", "warning", "suggestion"],
  "ignore_patterns": ["*.md", "*.lock"]
}
```

## Example Output

```markdown
## Code Review Results

### ✅ Good Practices
- Proper error handling implemented
- Clear variable naming conventions

### ⚠️ Suggestions
- Consider adding input validation at line 45
- This function could benefit from early return pattern

### 🚨 Critical Issues
- Potential null pointer exception at line 78