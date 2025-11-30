# Contributing

Thanks for your interest in contributing to md_crawler. Contributions can include bug reports, fixes, documentation improvements, tests, or feature requests.

How to contribute

1. Fork the repository and create a feature branch:

```bash
git clone <repo-url>
git checkout -b my-feature-branch
```

2. Make changes on your branch. Keep changes small and focused.

3. Add tests where appropriate and run `cargo test`.

4. Commit your changes with a clear message and push to your fork.

5. Open a pull request describing the change and why it is needed.

Coding guidelines

- Follow existing code style and minimal changes to unrelated files.
- Avoid adding large, unrelated dependencies without discussion.
- Prefer explicit `Result` error handling over panics (`unwrap()`), especially for library code.

Development notes

- Use `cargo build` / `cargo check` to verify the project builds.
- Use `cargo clippy` and `cargo fmt` (if available) to check lint and formatting.

Reporting issues

- Open an issue with a concise title, steps to reproduce, and expected vs actual behavior.

License

Contributions are under the same license as the project (see `LICENSE`).
