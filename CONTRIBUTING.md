# Contributing

Thanks for your interest in contributing to Traffic Camera Viewer.

## Development Setup

### Requirements

This repo uses [mise](https://mise.jdx.dev) as the single toolchain source of truth. It provides
Rust (via rustup), Bun, [commitizen](https://commitizen-tools.github.io/commitizen/) and
[pre-commit](https://pre-commit.com) — no separate installs of any of those needed.

```bash
# Install mise itself
# Windows
winget install jdx.mise
# macOS / Linux
curl https://mise.run | sh
```

Then, from the repo root:
```bash
mise install       # installs the pinned Rust, Bun, commitizen, pre-commit
mise run setup     # bun install + activates the pre-commit git hooks
```

- On Windows: VS Build Tools with the C++ workload and the Windows SDK are also required (not
  something mise can provide). Without them, `link.exe` resolves to GNU coreutils' `link` and
  every build script fails with `link: extra operand`.
  ```powershell
  winget install --id Microsoft.VisualStudio.2022.BuildTools --override "--wait --quiet --norestart --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
  ```
- On Linux (Debian/Ubuntu), WebKit + GTK dev packages are also required:
  ```bash
  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev \
    libayatana-appindicator3-dev librsvg2-dev
  ```
- On macOS — works out of the box; Xcode command-line tools required.

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

And this one, because the AI-guidance pointer files (`CLAUDE.md`, `AGENTS.md`, etc.) are real
git symlinks to `.ai/instructions.md`:
```bash
git config --local core.symlinks true
```
Without it, git checks those files out as plain text files containing the target path instead of
real symlinks, and a later edit-and-commit from that clone would silently convert them into real
files for everyone else. **Windows contributors need Developer Mode enabled before cloning** —
unprivileged symlink creation requires it.

**Committing from an editor UI (not a terminal) on Windows:** mise's shims dir
(`%LOCALAPPDATA%\mise\shims`, or `~/.local/share/mise/shims` on Linux/macOS) needs to be on your
*persistent* PATH. `mise activate` only updates PATH when a shell prompt is drawn, which an
editor's built-in commit UI never triggers — so the `cz` commit-msg hook resolves nothing from
PATH and the commit fails there, even though the exact same commit works fine from a terminal.
Restart the editor fully after changing PATH.

### Build and Run

```bash
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

# Frontend lint + format (Biome + Prettier)
bun run check
bun run check:fix

# Frontend + server type checking (svelte-check + server tsc)
bun run typecheck
```

## Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com) for formatting and commit linting,
and [commitizen](https://commitizen-tools.github.io/commitizen/) for semantic versioning. Both
are installed by `mise install` and activated by `mise run setup` — no separate install step.

Hooks that run automatically:
- **`cargo-fmt`** — formats changed Rust files (in `src-tauri/`) on every commit
- **pre-commit-hooks hygiene suite** — whitespace/EOL/YAML/JSON/TOML checks, private-key
  detection, merge-conflict markers
- **`biome`** — runs `bun run check:fix` (Biome + Prettier) on every commit
- **`commitizen`** (`cz check`, commit-msg stage) — rejects commits whose message doesn't follow
  the [Conventional Commits](https://www.conventionalcommits.org) spec

To bump the version and generate a changelog entry:
```bash
cz bump
```

**Note:** `cz bump` automatically updates the version in `src-tauri/Cargo.toml`,
`src-tauri/tauri.conf.json`, and `package.json` as configured in `.cz.toml`, then commits, tags,
and pushes the result.

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
3. Run `cd src-tauri && cargo check && cargo test`, then `bun run check` and `bun run typecheck`.
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
- TypeScript strict mode is enabled; `bun run typecheck` (svelte-check + server tsc) must pass
  with no errors. `bun run check` (Biome + Prettier) must also pass clean.

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

If a shell environment can't find `cargo`, `bun`, `cz` or `pre-commit`, make sure mise's shims dir
(`%LOCALAPPDATA%\mise\shims`) is on PATH — see the editor-PATH warning above; this applies to any
non-interactive shell, not just editor commit UIs.
The Rust crate is in `src-tauri/` — always run `cargo` commands from that directory.

## License

By contributing, you agree that your contributions are licensed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for details.
