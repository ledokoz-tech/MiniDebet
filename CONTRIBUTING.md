# Contributing to MiniDebet

Thank you for your interest in contributing to MiniDebet! This document provides guidelines and procedures for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:
- Be respectful and inclusive
- Focus on constructive feedback
- Welcome newcomers and different perspectives
- Maintain professionalism in all interactions

## How to Contribute

### Reporting Issues

**Before submitting an issue:**
1. Check existing issues to avoid duplicates
2. Ensure you're using the latest version
3. Include steps to reproduce the issue
4. Provide relevant environment information

**Good bug reports include:**
- Clear title and description
- Steps to reproduce
- Expected vs actual behavior
- Screenshots/logs if applicable
- Environment details (OS, versions, etc.)

### Feature Requests

Submit feature requests as GitHub issues with:
- Clear problem statement
- Proposed solution description
- Benefits and use cases
- Potential alternatives considered

### Pull Requests

#### Getting Started
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Write/update tests
5. Update documentation
6. Submit pull request

#### PR Requirements
- **Code Quality**: Follow existing code style and conventions
- **Tests**: Include tests for new functionality
- **Documentation**: Update relevant documentation
- **Commits**: Use clear, descriptive commit messages
- **Branch Naming**: Use descriptive branch names

#### Commit Message Guidelines
Follow conventional commits format:
```
type(scope): brief description

Detailed explanation of changes (optional)

Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(auth): add password reset functionality`
- `fix(api): resolve invoice calculation rounding error`
- `docs(readme): update installation instructions`

## Development Standards

### Backend (Rust)
- Follow Rust naming conventions
- Use `clippy` for linting (`cargo clippy`)
- Format code with `rustfmt` (`cargo fmt`)
- Write unit tests for new functionality
- Document public APIs with rustdoc

### Frontend (TypeScript/React)
- Use TypeScript for type safety
- Follow React best practices
- Write component tests with React Testing Library
- Use ESLint and Prettier for code quality
- Maintain consistent component structure

### Database
- Write clear, reversible migrations
- Follow naming conventions for tables/columns
- Add appropriate indexes for performance
- Document schema changes

### Documentation
- Keep documentation up-to-date with code changes
- Use clear, concise language
- Include examples where helpful
- Update README files when adding new features

## Testing Requirements

### Before Submitting PR
- Run all tests locally
- Ensure test coverage meets minimum requirements
- Test functionality manually
- Verify no breaking changes

### Test Categories
1. **Unit Tests**: Individual function/component testing
2. **Integration Tests**: Component interaction testing
3. **End-to-End Tests**: Full workflow testing

## Review Process

### PR Review Checklist
- [ ] Code follows project standards
- [ ] Tests pass and cover new functionality
- [ ] Documentation is updated
- [ ] No breaking changes (or properly documented)
- [ ] Performance impact assessed
- [ ] Security considerations addressed

### Review Timeline
- Initial review: Within 2 business days
- Follow-up reviews: As needed based on feedback
- Merge: After approval and passing CI

## Getting Help

### Questions
- Check existing documentation first
- Search issues and discussions
- Join community channels if available
- Contact maintainers for complex questions

### Development Help
- Review development setup guide
- Check existing code for patterns
- Ask specific technical questions in issues
- Pair programming sessions available for major contributions

## Recognition

Contributors will be acknowledged in:
- Project README
- Release notes
- Contributor documentation
- Social media announcements (for significant contributions)

## License

By contributing to MiniDebet, you agree that your contributions will be licensed under the project's proprietary license. All contributions become property of Ledokoz GmbH.

## Contact

- **Email**: jamshaid@ledokoz.com
- **Issues**: GitHub issues tracker
- **Feature Requests**: Through proper channels

Thank you for helping make MiniDebet better!