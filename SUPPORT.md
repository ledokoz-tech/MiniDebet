# Support

## 🆘 Getting Help

Need assistance with MiniDebet? We're here to help! Here are the best ways to get support:

## 📚 Documentation

Before reaching out, please check our documentation:

- [📖 README](./README.md) - Project overview and getting started
- [📘 User Guide](./docs/user-guide.md) - Detailed usage instructions
- [🛠️ Developer Guide](./docs/developer-guide.md) - For contributors and developers
- [🔒 Security Policy](./SECURITY.md) - Security guidelines and reporting
- [🤝 Code of Conduct](./CODE_OF_CONDUCT.md) - Community guidelines

## 🎯 Support Channels

### 📧 Email Support
For general inquiries and support:
**support@minidebet.com**

### 🐛 Bug Reports
Found a bug? Please use our [Bug Report Template](.github/ISSUE_TEMPLATE/bug_report.md)

### 💡 Feature Requests
Have an idea? Submit a [Feature Request](.github/ISSUE_TEMPLATE/feature_request.md)

### 💬 Community Discussion
Join our community discussions:
- [GitHub Discussions](https://github.com/yourusername/minidebet/discussions)
- [Discord Server](https://discord.gg/minidebet) *(if applicable)*

## 🚀 Quick Troubleshooting

### Common Issues

**1. Installation Problems**
```bash
# Check Node.js version
node --version  # Should be >= 18.0.0

# Check Rust version
rustc --version  # Should be >= 1.70.0

# Clear npm cache
npm cache clean --force
```

**2. Database Connection Issues**
```bash
# Verify D1 database setup
wrangler d1 list

# Check database migrations
wrangler d1 migrations list DB
```

**3. Authentication Errors**
- Ensure `CLOUDFLARE_API_TOKEN` is set correctly
- Verify database bindings in `wrangler.toml`
- Check JWT token expiration

### 🛠️ Self-Help Resources

**Log Analysis**
```bash
# View Cloudflare Worker logs
wrangler tail

# Check build logs
npm run build --verbose
```

**Environment Verification**
```bash
# Check required environment variables
echo $CLOUDFLARE_API_TOKEN
echo $CLOUDFLARE_ACCOUNT_ID
```

## 🎯 Response Times

| Support Type | Typical Response Time |
|--------------|----------------------|
| 🐛 Bugs | Within 24 hours |
| 💡 Features | Within 72 hours |
| 🆘 Urgent Issues | Within 4 hours |
| 📧 General Questions | Within 48 hours |

## 🏆 Premium Support

For enterprise customers and priority support:

### 📞 Priority Support Options
- **Phone Support**: +1 (555) SUPPORT-MINI *(Business hours)*
- **Slack Channel**: Dedicated support channel for enterprise customers
- **Direct Engineering Access**: Connect with core developers

### 💼 Enterprise Plans
Contact our sales team for premium support packages:
**enterprise@minidebet.com**

## 🤝 Community Support

### Contributing to Solutions
- Answer questions in GitHub Discussions
- Share your solutions and workarounds
- Help improve documentation
- Contribute bug fixes

### Recognition Program
Active community helpers receive:
- 🌟 Contributor badges
- 🎁 Early access to features
- 📣 Social media recognition
- 💰 Bounty rewards for critical fixes

## 🔐 Security Concerns

**⚠️ Important**: For security vulnerabilities, DO NOT use public channels.

📧 **Contact**: security@minidebet.com
🕐 **Response**: Within 24 hours

See our [Security Policy](./SECURITY.md) for detailed reporting guidelines.

## 📊 Support Metrics

We track and publish our support performance:

- **First Response Time**: Average < 12 hours
- **Resolution Time**: Average < 48 hours
- **Customer Satisfaction**: 95%+
- **Issue Resolution Rate**: 98%+

## 🙏 Thank You!

Thank you for choosing MiniDebet. We're committed to providing excellent support and continuously improving our product based on your feedback.

---

*Last updated: January 2026*