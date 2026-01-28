# Security Policy

## 🛡️ Reporting a Vulnerability

We take the security of MiniDebet seriously. If you discover a security vulnerability, please follow these steps:

### How to Report
1. **DO NOT** create a public GitHub issue
2. Email us directly at: **security@minidebet.com**
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline
- **24 hours**: Initial acknowledgment of your report
- **7 days**: Preliminary assessment and classification
- **30 days**: Fix development and release coordination
- **60 days**: Public disclosure (if applicable)

## 🔒 Security Measures

### Authentication & Authorization
- JWT-based authentication with secure token handling
- Role-based access control
- Password hashing with industry-standard algorithms

### Data Protection
- End-to-end encryption for sensitive data
- Secure database connections
- Regular security audits

### Infrastructure Security
- Cloudflare Workers with built-in DDoS protection
- Automated security scanning in CI/CD pipeline
- Regular dependency vulnerability checks

## 📋 Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | ✅ Active          |
| < 1.0.0 | ❌ Unsupported     |

## 🎯 Security Best Practices

### For Developers
- Keep dependencies up to date
- Use environment variables for secrets
- Validate all user inputs
- Implement proper error handling
- Follow OWASP security guidelines

### For Users
- Use strong, unique passwords
- Enable two-factor authentication when available
- Keep your software updated
- Be cautious of phishing attempts

## 🤝 Responsible Disclosure

We believe in responsible disclosure and appreciate security researchers who:
- Give us reasonable time to address vulnerabilities
- Avoid privacy violations and data destruction
- Respect our users and infrastructure
- Follow coordinated disclosure practices

## 📞 Contact

- **Security Team**: security@minidebet.com
- **General Inquiries**: support@minidebet.com
- **Emergency Hotline**: +1 (555) SECURITY

---

*Last updated: January 2026*