# Qoder Usage Quick Reference

## 🤖 Using Agents

Agents are autonomous AI processes for complex tasks.

### Available Agents

1. **Code Review Agent** (`.qoder/agents/code-review.md`)
   - Use when: "Review my recent changes" or "Analyze this PR"
   - Provides: Quality analysis, bug detection, suggestions

2. **Browser Automation Agent** (`.qoder/agents/browser-automation.md`)
   - Use when: "Test the login flow" or "Navigate to X and capture Y"
   - Provides: Web automation, screenshots, validation

3. **Environment Validator Agent** (`.qoder/agents/env-validator.md`)
   - Use when: "Check all .env files" or "Find broken references"
   - Provides: Validation reports, security scans, dependency checks

### How to Invoke

Simply ask:
- "Use the [agent name] to [task]"
- "Run [agent] on [file/folder]"
- "Analyze this with [agent]"

Example:
```
"Use the code review agent to analyze my Rust files"
"Run the env validator on .env.production"
```

---

## 🎯 Using Skills

Skills teach me specific workflows and best practices.

### Available Skills

1. **Git Commit** (`.qoder/skills/git-commit.md`)
   - Applied when: Creating commits
   - Format: `type(scope): subject`
   - Types: feat, fix, docs, style, refactor, test, chore

2. **API Development** (`.qoder/skills/api-development.md`)
   - Applied when: Building REST APIs
   - Covers: Endpoints, responses, validation, security

3. **Rust Project Management** (`.qoder/skills/rust-project-management.md`)
   - Applied when: Writing Rust code
   - Covers: Error handling, modules, testing, documentation

### How They Work

Skills are applied automatically when relevant:
- Making a commit? → Git commit skill activates
- Building an API? → API development skill guides
- Writing Rust? → Rust best practices apply

You don't need to explicitly invoke skills - I follow them automatically!

---

## 🛠️ Common Workflows

### Code Review Workflow
```
1. Make changes to code
2. Ask: "Use the code review agent to analyze my changes"
3. Review the feedback
4. Fix identified issues
5. Commit using conventional commit format
```

### Environment Validation Workflow
```
1. Update .env files
2. Ask: "Check for broken references in .env"
3. Review validation report
4. Fix any circular dependencies or missing variables
5. Commit changes
```

### New Feature Workflow
```
1. Plan the feature
2. Implement following Rust best practices
3. Test thoroughly
4. Ask for code review
5. Commit with proper format
```

---

## 📝 Examples

### Example 1: Getting a Code Review
```
You: "Can you review the changes I made to main.rs?"
Me: *References code-review.md agent* 
    *Analyzes your code*
    *Provides detailed feedback with line numbers*
```

### Example 2: Creating a Commit
```
You: "I want to commit these changes"
Me: *References git-commit.md skill*
    *Helps craft message: "feat(validation): add circular dependency detection"*
    *Explains what changed and why*
```

### Example 3: Validating Environments
```
You: "Check if my .env files have issues"
Me: *References env-validator.md agent*
    *Scans all .env files*
    *Reports: "Found circular dependency at line 45"*
    *Suggests fixes*
```

---

## 🚀 Pro Tips

1. **Be Specific**: "Review the error handling in validator.rs" is better than "Review my code"

2. **Chain Operations**: 
   - "Validate the .env files, then help me fix any issues"
   - "Review this PR and create a commit if it looks good"

3. **Ask for Explanations**: 
   - "Why is this considered a bad practice?"
   - "Show me how to apply the builder pattern here"

4. **Request Examples**:
   - "Show me an example of proper error handling"
   - "How should I document this function?"

---

## 📁 File Locations

```
.qoder/
├── agents/
│   ├── code-review.md          # Code analysis
│   ├── browser-automation.md   # Web testing
│   └── env-validator.md        # .env validation
└── skills/
    ├── git-commit.md           # Commit standards
    ├── api-development.md      # API best practices
    └── rust-project-management.md  # Rust conventions
```

---

## ❓ Need Help?

Just ask:
- "What agents do I have?"
- "How do I use the git commit skill?"
- "Show me available skills"
- "What's the best way to [task]?"

I'll reference the appropriate files and guide you!
