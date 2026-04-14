# Contributing

Thanks for your interest in contributing to Traffic Camera Viewer.

## Development Setup

### Requirements

- Rust toolchain (stable) via `rustup`
- `cargo` available on PATH

### Platform-specific build dependencies

**Windows** — works out of the box after installing the Rust toolchain and VS Build Tools.

Git for Windows defaults to `core.autocrlf=true`, which conflicts with this repository's
`eol=lf` policy and causes spurious warnings from tools like `cz bump`. Fix it once per clone:
```bash
git config --local core.autocrlf input
```

**Linux (Debian/Ubuntu):**
```bash
sudo apt install libgtk-3-dev libxkbcommon-dev \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev cmake
```

**macOS:**
```bash
brew install cmake
```

### Build and Run

```bash
cargo build
cargo build --release
cargo run
cargo test
cargo check
```

## Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com) for formatting and commit linting,
and [commitizen](https://commitizen-tools.github.io/commitizen/) for semantic versioning.

Install [uv](https://docs.astral.sh/uv/getting-started/installation/) if not already present:
```bash
# Windows
winget install astral-sh.uv

# Linux / macOS
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Then install the tools and activate the hooks:
```bash
uv tool install pre-commit
uv tool install commitizen
pre-commit install
pre-commit install --hook-type commit-msg
```

Hooks that run automatically:
- **`cargo fmt`** — formats changed Rust files on every commit
- **conventional-pre-commit** — rejects commits whose message doesn't follow the
  [Conventional Commits](https://www.conventionalcommits.org) spec

To bump the version and generate a changelog entry:
```bash
cz bump
```

## Commit Message Format

This project follows the [Conventional Commits](https://www.conventionalcommits.org) spec.
The `commit-msg` pre-commit hook enforces this automatically.

```
<type>(<optional scope>): <short summary>

<optional body>
```

Allowed types:

| Type | Use for |
|------|---------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only |
| `style` | Formatting, whitespace (no logic change) |
| `refactor` | Code restructuring without feature/fix |
| `test` | Adding or updating tests |
| `perf` | Performance improvements |
| `build` | Build system or dependency changes |
| `ci` | CI/CD configuration changes |
| `chore` | Maintenance tasks that don't fit above |

`cz bump` uses these types to determine the next semantic version:
`feat` → minor bump, `fix` → patch bump, breaking change footer (`BREAKING CHANGE:`) → major bump.

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
