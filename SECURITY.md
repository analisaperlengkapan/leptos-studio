# Security Policy

## Supported Versions

Currently supported versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.2.x   | :white_check_mark: |
| < 0.2.0 | :x:                |

## Security Features

### Input Validation
- ✅ Component name validation (Rust identifier rules)
- ✅ HTML template validation
- ✅ XSS prevention through Leptos framework
- ✅ LocalStorage data sanitization

### Build Security
- ✅ Content Security Policy headers recommended
- ✅ WASM optimization for production
- ✅ No inline scripts in production build
- ✅ Dependency security auditing via CI/CD

### Recommended CSP Headers

For production deployment, add these Content Security Policy headers:

```
Content-Security-Policy: 
  default-src 'self';
  script-src 'self' 'wasm-unsafe-eval';
  style-src 'self' 'unsafe-inline' https://fonts.googleapis.com;
  font-src 'self' https://fonts.gstatic.com;
  img-src 'self' data: https:;
  connect-src 'self';
  base-uri 'self';
  form-action 'self';
  frame-ancestors 'none';
  upgrade-insecure-requests;
```

### LocalStorage Security

The application stores:
- Layout data (user-generated component structure)
- Custom components (user-defined templates)
- Component library configuration

**Best Practices:**
1. Data is stored locally only - never transmitted
2. Input validation prevents malicious code injection
3. Component templates are sanitized
4. Consider encryption for sensitive projects

## Reporting a Vulnerability

If you discover a security vulnerability, please follow these steps:

1. **DO NOT** open a public issue
2. Email the security team at: [security@leptos-studio.dev]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

### Response Timeline
- Initial response: Within 48 hours
- Status update: Within 7 days
- Fix timeline: Based on severity
  - Critical: 24-48 hours
  - High: 1 week
  - Medium: 2-4 weeks
  - Low: Next release cycle

## Security Best Practices for Users

### Development
1. Keep Rust toolchain updated
2. Run `cargo audit` regularly
3. Review custom component templates
4. Don't paste untrusted code into custom components
5. Use version control for your projects

### Deployment
1. Enable HTTPS only
2. Implement proper CSP headers
3. Use secure hosting providers
4. Enable HTTP security headers:
   - X-Frame-Options: DENY
   - X-Content-Type-Options: nosniff
   - Referrer-Policy: strict-origin-when-cross-origin
   - Permissions-Policy: geolocation=(), microphone=(), camera=()

### Production Checklist
- [ ] Build with `trunk build --release`
- [ ] Enable CSP headers
- [ ] Use HTTPS
- [ ] Implement rate limiting
- [ ] Regular security updates
- [ ] Monitor for vulnerabilities
- [ ] Regular backups of localStorage data
- [ ] Test in isolated environment first

## Known Security Considerations

### Client-Side Storage
LocalStorage is used for persistence. Users should be aware:
- Data is stored unencrypted in browser
- Cleared when browser cache is cleared
- Accessible to any script on the same origin
- Not suitable for highly sensitive data

**Mitigation:** For sensitive projects, consider:
- Using server-side storage
- Implementing client-side encryption
- Exporting and storing layouts externally

### Custom Component Templates
User-provided HTML templates are sanitized but users should:
- Only use trusted component sources
- Review custom component code
- Avoid executing untrusted templates

### Dependencies
We regularly audit dependencies using:
- `cargo audit` in CI/CD
- Dependabot for automatic updates
- Manual security reviews

## Security Updates

Security updates are announced via:
- GitHub Security Advisories
- Release notes (CHANGELOG.md)
- Project README badges

Stay updated by:
- Watch GitHub repository
- Subscribe to releases
- Follow project announcements

## Compliance

### OWASP Top 10 Coverage
- ✅ A03:2021 – Injection (prevented via Leptos + validation)
- ✅ A05:2021 – Security Misconfiguration (documented)
- ✅ A06:2021 – Vulnerable Components (audited)
- ✅ A07:2021 – Auth Failures (N/A - no authentication)
- ✅ A08:2021 – Data Integrity (localStorage validation)

## Contact

For security concerns, contact:
- Email: security@leptos-studio.dev
- GitHub Security Advisories
- Private disclosure preferred

Thank you for helping keep Leptos Studio secure! 🔒
