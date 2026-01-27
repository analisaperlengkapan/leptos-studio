# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 🚨 DO NOT

- **Do not** open a public GitHub issue
- **Do not** discuss the vulnerability in public forums or social media
- **Do not** exploit the vulnerability

### ✅ DO

1. **Email us privately** at: [INSERT_SECURITY_EMAIL_HERE]
   - Include detailed information about the vulnerability
   - Provide steps to reproduce if possible
   - Include any proof-of-concept code

2. **Expect a response** within 48 hours acknowledging receipt

3. **Work with us** to understand and fix the issue

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity (critical issues prioritized)

### What to Include

When reporting a vulnerability, please include:

- Type of vulnerability (XSS, CSRF, etc.)
- Full paths of source file(s) affected
- Location of the affected code (tag/branch/commit or direct URL)
- Step-by-step instructions to reproduce
- Proof-of-concept or exploit code (if possible)
- Impact of the issue (what can an attacker do?)
- Any suggested fixes

### Disclosure Policy

- We will confirm the vulnerability and determine its severity
- We will release a fix as soon as possible
- We will credit you for the discovery in the release notes (unless you prefer to remain anonymous)
- We ask that you do not publicly disclose the issue until we've had a chance to address it

### Safe Harbor

We support safe harbor for security researchers who:

- Make a good faith effort to avoid privacy violations, data destruction, and service interruption
- Only interact with accounts you own or with explicit permission of the account holder
- Do not exploit a vulnerability beyond what is necessary to demonstrate it
- Report vulnerabilities promptly

## Security Best Practices for Users

As a WASM application running entirely in the browser:

1. **Keep Your Browser Updated**: Always use the latest version of your browser
2. **Be Cautious with Imports**: Only import project JSON files from trusted sources
3. **Review Custom Components**: Carefully review any custom HTML components before adding them
4. **HTTPS Only**: Only access Leptos Studio via HTTPS in production
5. **Clear Storage**: Regularly clear browser storage if you work with sensitive data

## Known Security Considerations

### Client-Side Storage

- Project data is stored in browser LocalStorage
- This data is accessible to any script running on the same origin
- Do not store sensitive information in projects

### Custom Components

- Custom components use HTML templates
- While we sanitize input, always review custom HTML before adding
- Avoid using custom components from untrusted sources

### Export Functionality

- Exported code should be reviewed before use in production
- Generated code is based on user input and should be validated

## Security Updates

Security updates will be:

- Released as soon as possible after verification
- Announced in GitHub releases with `[SECURITY]` tag
- Documented in CHANGELOG.md
- Communicated to users via GitHub security advisories

## Questions?

If you have security-related questions that aren't about a vulnerability, please:

- Open a GitHub Discussion
- Include `[SECURITY]` in the title
- We'll respond publicly (unless the topic requires confidentiality)

Thank you for helping keep Leptos Studio and its users safe! 🔒
