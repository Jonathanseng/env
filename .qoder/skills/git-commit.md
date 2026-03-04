# Git Commit Skill

This skill guides the AI through creating proper git commits following conventional commit standards.

## Purpose

Ensure all commits follow a consistent, descriptive format that makes version history clear and enables automated changelog generation.

## Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

## Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without behavior change
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks, dependencies

## Rules

1. **Subject Line**
   - Maximum 50 characters
   - Start with lowercase letter
   - Use imperative mood ("add" not "added")
   - No period at the end

2. **Body** (optional)
   - Wrap at 72 characters
   - Explain what and why, not how
   - Use blank line after subject

3. **Footer** (optional)
   - Reference issues/PRs
   - Breaking changes notation
   - Co-authored-by credits

## Examples

✅ Good:
```
feat(auth): add password reset functionality

Implement password reset via email with secure token generation.
Token expires after 24 hours for security.

Closes #123
```

❌ Bad:
```
fixed some stuff
```

## Workflow

1. Stage relevant changes
2. Analyze what was changed
3. Determine appropriate type and scope
4. Write concise subject line
5. Add body if context is needed
6. Include footer references if applicable
7. Create commit with formatted message

## Validation Checklist

- [ ] Subject line ≤ 50 characters
- [ ] Uses correct type prefix
- [ ] Imperative mood in subject
- [ ] No trailing period
- [ ] Body wrapped at 72 chars (if present)
- [ ] References included (if applicable)