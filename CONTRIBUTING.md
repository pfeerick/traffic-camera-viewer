# Contributing

Thanks for your interest in contributing to Traffic Camera Viewer.

## Development Setup

### Requirements

- Rust toolchain (stable) via `rustup`
- [Bun](https://bun.sh) runtime (JavaScript/TypeScript)
  ```bash
  # Windows
  winget install OvenSh.Bun
  # macOS / Linux
  curl -fsSL https://bun.sh/install | bash
  ```
- Node.js **22.12+** (required by Vite 8). The repo ships a `.node-version` file pinned to `22`.
  Recommended: use [fnm](https://github.com/Schniz/fnm) for automatic version switching.
  ```bash
  # Windows
  winget install Schniz.fnm
  # macOS / Linux
  curl -fsSL https://fnm.vercel.app/install | bash
  ```
  After installing fnm, add shell integration (see fnm docs), then `fnm install 22` once.
  fnm will auto-switch to Node 22 whenever you enter this directory.
- On Windows: VS Build Tools (included with Visual Studio)

### Git configuration (all platforms)

Run these once after cloning:
```bash
git config --local push.followTags true   # push tags alongside commits automatically
git config --local push.atomic true       # push branch + tags atomically
```

Also run these to prevent spurious CRLF warnings from `cz bump`:
```bash
git config --local core.autocrlf input   # normalise to LF on commit
git config --local core.safecrlf false   # silence conversion warnings
```

### Platform-specific build dependencies

**Windows** — works out of the box after installing the Rust toolchain, VS Build Tools, and Bun.

**Linux (Debian/Ubuntu):**
```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev
```

**macOS** — works out of the box; Xcode command-line tools required.

### Build and Run

```bash
# Install frontend dependencies
bun install

# Tauri desktop development (Vite dev server + Tauri window)
bun run tauri:dev

# Tauri desktop release build
bun run tauri:build

# Frontend only (browser preview at http://localhost:5173)
bun run dev

# Frontend production build
bun run build

# Web server (serve built frontend + API proxy)
bun run build && bun run server

# Rust unit tests (from src-tauri/)
cd src-tauri && cargo test

# TypeScript check
bun tsc --noEmit
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
- **`cargo fmt`** — formats changed Rust files (in `src-tauri/`) on every commit
- **conventional-pre-commit** — rejects commits whose message doesn't follow the
  [Conventional Commits](https://www.conventionalcommits.org) spec

To bump the version and generate a changelog entry:
```bash
cz bump
```

**Note:** `cz bump` automatically updates the version in `src-tauri/Cargo.toml`,
`src-tauri/tauri.conf.json`, and `package.json` as configured in `.cz.toml`.

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
3. Run `cd src-tauri && cargo check && cargo test` and `bun tsc --noEmit`.
4. Update docs when behavior changes.
5. Open a pull request with a clear summary.

## Coding Guidelines

### Rust (src-tauri/)

- Prefer small, incremental changes.
- **Lint:** `cargo clippy -- -D warnings` must pass. The project also targets zero warnings under
  `-W clippy::pedantic -W clippy::nursery`. Prefer fixing the code over silencing lints; when a
  suppression is unavoidable use a narrow inline `#[allow(...)]` on the exact line with a comment.

### TypeScript / Svelte (src/)

- Use Svelte stores for shared state; `$state` / `$derived` runes for local component state.
- Keep the API abstraction layer (`src/lib/api/`) clean — route calls through `api.someMethod()`,
  never call Tauri `invoke()` or `fetch()` directly from components.
- TypeScript strict mode is enabled; `bun tsc --noEmit` must pass with no errors.

## UI Changes

When changing UI behavior, verify in both modes:

**Tauri desktop (`bun run tauri:dev`):**
1. Camera grid renders and refreshes correctly.
2. Settings panel opens/closes; Apply saves config, Cancel reverts.
3. Context menu (right-click) refresh/hide works.
4. Status bar countdown and "Refresh Now" work.

**Web (`bun run build && bun run server`):**
1. Same functional checks as above.
2. DevTools Network confirms all camera fetches hit `/api/*`, not QLD domains directly.
3. Config persists across page reloads.

## Windows Notes

If tools run in shell environments that cannot find `cargo`, ensure PATH includes the Cargo bin directory.
The Rust crate is in `src-tauri/` — always run `cargo` commands from that directory.

## License

By contributing, you agree that your contributions are licensed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for details.
