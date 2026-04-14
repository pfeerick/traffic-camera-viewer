# Contributing

Thanks for your interest in contributing to Traffic Camera Viewer.

## Development Setup

### Requirements

- Rust toolchain (stable) via `rustup`
- `cargo` available on PATH

### Build and Run

```bash
cargo build
cargo build --release
cargo run
cargo test
cargo check
```

## Project Workflow

1. Create a feature branch.
2. Make focused changes.
3. Run `cargo check` and `cargo test`.
4. Update docs when behavior changes.
5. Open a pull request with a clear summary.

## Coding Guidelines

- Prefer small, incremental changes.
- Keep UI behavior configurable where practical.
- Avoid reformatting unrelated files.
- Add comments only where logic is not obvious.

## UI Changes

When changing UI behavior:

1. Verify desktop layout.
2. Verify settings panel open/close behavior.
3. Verify refresh/timer behavior in the status bar.
4. Verify no visual regressions in camera grid scrolling.

## Windows Notes

If tools run in shell environments that cannot find `cargo`, ensure PATH includes Cargo bin directory.

## License

By contributing, you agree that your contributions are licensed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for details.
