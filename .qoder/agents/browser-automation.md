# Browser Automation Agent

This agent automates browser-based tasks and web application testing.

## Capabilities

- Navigate through web pages and applications
- Fill forms and interact with UI elements
- Capture screenshots for visual verification
- Validate page content and element states
- Perform end-to-end workflow testing

## Available Actions

- `navigate(url)` - Navigate to a specific URL
- `click(selector)` - Click on an element
- `type(selector, text)` - Type text into an input field
- `screenshot(name)` - Capture a screenshot
- `waitFor(selector)` - Wait for an element to appear
- `getText(selector)` - Extract text from an element
- `scrollTo(selector)` - Scroll to an element

## Usage Example

```yaml
task: "Test login flow"
steps:
  - navigate: "https://example.com/login"
  - type: "#email, user@example.com"
  - type: "#password, secret123"
  - click: "button[type='submit']"
  - waitFor: ".dashboard"
  - screenshot: "login-success"
```

## Configuration

```json
{
  "viewport": {
    "width": 1920,
    "height": 1080
  },
  "timeout": 30000,
  "headless": true,
  "waitForNetworkIdle": true
}
```

## Best Practices

- Always wait for elements before interacting
- Use specific CSS selectors when possible
- Take screenshots at critical checkpoints
- Handle errors gracefully with fallback actions