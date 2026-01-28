# Contributing to MiniDebet

Welcome! We're excited that you're interested in contributing to MiniDebet. This document will help you get started with our contribution process.

## 🎯 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before contributing.

## 🚀 Getting Started

### Prerequisites

Before you begin, ensure you have:

- **Node.js** 18+ installed
- **Rust** 1.70+ with `wasm32-unknown-unknown` target
- **Git** for version control
- **Cloudflare Account** (for full testing)

### Setting Up Development Environment

1. **Fork and Clone**
```bash
git clone https://github.com/yourusername/minidebet.git
cd minidebet
```

2. **Install Dependencies**
```bash
# Frontend
cd frontend && npm install

# Shared types
cd ../shared && npm install && npm run build

# Backend (ensure Rust is installed)
cd ../backend && rustup target add wasm32-unknown-unknown
```

3. **Environment Setup**
```bash
# Copy environment files
cp frontend/.env.example frontend/.env
# Configure your Cloudflare credentials
```

4. **Verify Setup**
```bash
npm run test  # Run tests to verify everything works
```

## 🛠️ Development Workflow

### Branch Naming Convention

Use descriptive branch names:
- `feature/new-feature-name`
- `bugfix/issue-description`
- `hotfix/critical-fix`
- `docs/documentation-update`

### Making Changes

1. **Create a branch**
```bash
git checkout -b feature/amazing-new-feature
```

2. **Make your changes**
- Follow the existing code style
- Write clear, concise commit messages
- Add tests for new functionality
- Update documentation as needed

3. **Run tests**
```bash
# Frontend tests
cd frontend && npm run test

# Backend tests
cd backend && cargo test

# Integration tests
npm run test:integration
```

### Code Style Guidelines

#### Frontend (TypeScript/React)
- Use TypeScript for all new code
- Follow ESLint/Prettier configuration
- Component structure: Props interface, component function, exports
- Use functional components with hooks
- Proper error boundaries and loading states

#### Backend (Rust)
- Follow Rust naming conventions
- Use `clippy` for linting: `cargo clippy`
- Write comprehensive documentation comments
- Handle errors properly with `Result<T, E>`
- Use async/await for I/O operations

#### General Principles
- **DRY**: Don't Repeat Yourself
- **KISS**: Keep It Simple, Stupid
- **YAGNI**: You Aren't Gonna Need It
- **SOLID** principles where applicable

### Testing Requirements

All contributions must include appropriate tests:

- **Unit Tests**: Test individual functions/components
- **Integration Tests**: Test API endpoints and workflows
- **UI Tests**: Test user interface interactions
- **Performance Tests**: For performance-critical code

### Documentation Updates

Update relevant documentation when:
- Adding new features
- Changing APIs
- Modifying configuration
- Updating dependencies

Documentation locations:
- `README.md` - Project overview
- `docs/` directory - Detailed guides
- Inline code comments - Technical details
- `CONTRIBUTING.md` - This file

## 📤 Submitting Changes

### Pull Request Process

1. **Ensure your branch is up to date**
```bash
git fetch origin
git rebase origin/main
```

2. **Run final checks**
```bash
npm run lint        # Code quality checks
npm run test        # All tests pass
npm run build       # Successful build
```

3. **Submit PR with complete information**
- Clear title following conventional commits
- Detailed description of changes
- Reference related issues
- Include screenshots for UI changes
- Mention breaking changes if any

### Pull Request Review Process

1. **Automated Checks**
   - CI pipeline runs tests
   - Code quality scans
   - Security vulnerability checks
   - Build verification

2. **Manual Review**
   - Code review by maintainers
   - Functional testing
   - Security assessment
   - Performance evaluation

3. **Merge Requirements**
   - All CI checks pass
   - At least one approval from maintainers
   - No unresolved comments
   - Proper documentation updates

## 🐛 Reporting Issues

### Before Creating an Issue

1. Check existing issues
2. Verify it's not already documented
3. Ensure you're using the latest version
4. Try to reproduce in a fresh environment

### Issue Templates

Use appropriate templates:
- **Bug Report**: For software defects
- **Feature Request**: For new functionality
- **Support Question**: For help requests

### Good Bug Reports Include

- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details
- Screenshots/videos if applicable
- Error messages/logs

## 🎨 Design Contributions

### UI/UX Guidelines

- Maintain consistent design language
- Follow accessibility standards (WCAG 2.1)
- Mobile-first responsive design
- Consistent color scheme and typography
- Intuitive user flows

### Design Process

1. Discuss design ideas in issues first
2. Create mockups/wireframes
3. Get feedback from team
4. Implement with proper testing
5. Iterate based on user feedback

## 🔐 Security Contributions

### Responsible Disclosure

For security vulnerabilities:
1. **DO NOT** create public issues
2. Contact security@minidebet.com
3. Follow our [Security Policy](SECURITY.md)
4. Allow time for proper remediation

### Security Best Practices

- Validate all user inputs
- Sanitize database queries
- Use secure authentication methods
- Implement proper authorization
- Keep dependencies updated
- Follow OWASP guidelines

## 📈 Performance Optimization

### Performance Guidelines

- Profile before optimizing
- Focus on user-perceived performance
- Optimize critical paths
- Cache appropriately
- Minimize bundle sizes
- Lazy load non-critical resources

### Monitoring and Metrics

- Track performance metrics
- Monitor error rates
- Measure user engagement
- Analyze resource usage
- Set performance budgets

## 🎯 Community Engagement

### Ways to Contribute Beyond Code

- **Documentation**: Improve guides and tutorials
- **Testing**: Help with QA and user testing
- **Translation**: Localize for different languages
- **Design**: UI/UX improvements
- **Community**: Answer questions, mentor newcomers
- **Feedback**: Share user insights and suggestions

### Recognition

Contributors are recognized through:
- GitHub contributor list
- Release notes mentions
- Community spotlight features
- Swag rewards for significant contributions
- Early access to new features

## 📚 Learning Resources

### Project-Specific
- [Architecture Documentation](docs/architecture)
- [API Reference](docs/api)
- [Development Guides](docs/development)

### External Resources
- [React Documentation](https://reactjs.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)
- [Tailwind CSS Docs](https://tailwindcss.com/)

## 🆘 Getting Help

### Support Channels

- **GitHub Issues**: For bugs and feature requests
- **Discussion Forums**: For general questions
- **Community Chat**: Real-time help (Discord/Slack)
- **Email Support**: contributors@minidebet.com

### Mentorship Program

New contributors can request:
- Code review pairing sessions
- Architecture walkthroughs
- Guidance on contribution process
- Help with complex issues

## 📜 License Notice

By contributing to MiniDebet, you agree that your contributions will be licensed under the project's proprietary license. See [LICENSE](LICENSE) for details.

## 🙏 Thank You!

Thank you for considering contributing to MiniDebet. Your efforts help make this project better for everyone!

---

*Last updated: January 2026*