# Security Policy

## Supported Versions

Currently supported versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2.0 | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability, please follow responsible disclosure:

1. **DO NOT** open a public GitHub issue
2. Email the maintainers at: security@leptos-studio.dev (or repository owner)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

### Response Timeline

- **Initial response**: Within 72 hours
- **Status update**: Within 1 week
- **Fix timeline**: Based on severity
  - Critical: 24-48 hours
  - High: 1 week
  - Medium: 2-4 weeks
  - Low: Next release cycle

## Security Features

### Input Validation

- **Component Names**: Validated to be valid Rust identifiers
- **HTML Templates**: Validated for custom components
- **User Input**: Sanitized through Leptos framework
- **LocalStorage Data**: Validated on load

### Framework Security

Leptos Studio benefits from Leptos framework's built-in security:
- **XSS Prevention**: Automatic HTML escaping in `view!` macro
- **Type Safety**: Rust's type system prevents many common bugs
- **Memory Safety**: No buffer overflows or use-after-free bugs

### Client-Side Storage

The application uses browser localStorage for persistence:
- Layout data (user-generated component structure)
- Custom components (user-defined templates)
- Application preferences

**Important Notes:**
- Data is stored locally in the browser only
- No data is transmitted to external servers
- localStorage is unencrypted and accessible to scripts on same origin
- Data persists until browser cache is cleared

**Recommendations:**
- Don't store sensitive or confidential information in layouts
- Use external version control for important projects
- Export layouts regularly as backups
- Clear browser data when using shared computers

## Security Best Practices

### For Developers

**During Development:**
- Keep Rust toolchain updated: `rustup update`
- Run security audit: `cargo audit`
- Review custom component templates before adding
- Don't paste untrusted code into custom components
- Use version control for your work

**Code Review:**
- Check for unsafe code blocks
- Validate user input handling
- Review dependency updates
- Test security-related changes thoroughly

### For Deployment

**Required:**
- Enable HTTPS only (no HTTP)
- Implement Content Security Policy headers
- Use secure hosting provider
- Keep dependencies updated

**Recommended Security Headers:**

```
Content-Security-Policy: default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self';
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: geolocation=(), microphone=(), camera=()
```

**nginx example:**

```nginx
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:;" always;
add_header X-Frame-Options "DENY" always;
add_header X-Content-Type-Options "nosniff" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;
```

## Known Security Considerations

### LocalStorage Limitations

**Risk**: Data in localStorage is unencrypted and accessible to JavaScript on the same origin.

**Mitigation:**
- Don't store sensitive information
- Use server-side storage for sensitive projects
- Consider client-side encryption for sensitive data
- Export and store important layouts externally

### Custom Component Templates

**Risk**: User-provided HTML templates could contain malicious code if sourced from untrusted locations.

**Mitigation:**
- Only use trusted component sources
- Review all custom component code before adding
- Leptos framework escapes HTML by default
- Don't execute untrusted component templates

### Dependency Security

The project uses several dependencies. We mitigate risks by:
- Running `cargo audit` in CI/CD pipeline
- Reviewing dependency updates before merging
- Using stable, well-maintained crates
- Monitoring for security advisories

**Current dependencies** include:
- leptos 0.8.12
- web-sys 0.3
- serde/serde_json 1.0
- wasm-bindgen 0.2

Run `cargo audit` to check for known vulnerabilities:

```bash
cargo install cargo-audit
cargo audit
```

## OWASP Top 10 Considerations

### A01:2021 – Broken Access Control
**Status**: N/A - No authentication or authorization system

### A02:2021 – Cryptographic Failures
**Status**: ⚠️ - localStorage is unencrypted
**Mitigation**: Don't store sensitive data, document limitations

### A03:2021 – Injection
**Status**: ✅ - Protected by Leptos framework's HTML escaping

### A04:2021 – Insecure Design
**Status**: ✅ - Simple, focused design with clear security boundaries

### A05:2021 – Security Misconfiguration
**Status**: ⚠️ - Depends on deployment configuration
**Mitigation**: Document secure headers and deployment practices

### A06:2021 – Vulnerable and Outdated Components
**Status**: ✅ - Automated security auditing in CI/CD

### A07:2021 – Identification and Authentication Failures
**Status**: N/A - No authentication system

### A08:2021 – Software and Data Integrity Failures
**Status**: ✅ - localStorage validation, file hashing in builds

### A09:2021 – Security Logging and Monitoring Failures
**Status**: ⚠️ - Limited logging (client-side application)
**Mitigation**: Add error tracking in production deployments

### A10:2021 – Server-Side Request Forgery
**Status**: N/A - No server-side requests

## Production Security Checklist

Before deploying to production:

- [ ] Enable HTTPS with valid certificate
- [ ] Configure Content Security Policy headers
- [ ] Add security headers (X-Frame-Options, X-Content-Type-Options, etc.)
- [ ] Run `cargo audit` with no vulnerabilities
- [ ] Update all dependencies to latest secure versions
- [ ] Test in isolated environment first
- [ ] Set up error monitoring
- [ ] Document security configuration
- [ ] Create incident response plan
- [ ] Regular security updates schedule

## Updates and Announcements

Security updates are announced via:
- GitHub Security Advisories
- CHANGELOG.md
- Repository README

Stay informed:
- Watch the GitHub repository
- Subscribe to releases
- Enable GitHub security alerts

## Additional Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Leptos Security](https://leptos.dev/)
- [Web Security Basics](https://developer.mozilla.org/en-US/docs/Web/Security)

## Contact

For security concerns:
- Email: security@leptos-studio.dev
- GitHub: Private security advisories preferred

Thank you for helping keep Leptos Studio secure! 🔒
