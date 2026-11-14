# Contributing to APK-Builder

Thanks for your interest in improving APK-Builder! 🎉

## Getting started

1. Fork the repository and clone your fork.
2. Create a branch: `git checkout -b feature/my-change`.
3. Make your changes.
4. Make sure everything passes:

   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo test
   ```

5. Commit with a clear message and open a Pull Request.

## Guidelines

- Keep the project **dependency-free** unless a crate gives a clear benefit.
- Write code, comments and docs in **English**.
- Add unit tests for new helper functions.
- Keep pull requests focused: one change per PR.

## Reporting bugs

Open an [issue](https://github.com/JustRohams/APK-Builder/issues) with:

- Your OS, Rust, JDK and Gradle versions
- The exact steps you took
- The full terminal output

## Code of conduct

By participating you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).
