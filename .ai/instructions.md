# Traffic Camera Viewer — AI Assistant Notes

<!-- SELF-MAINTENANCE: Update this file whenever significant changes are made to the project
     (new modules, changed tooling, new workflows, dependency updates, architectural decisions).
     At the start of each session, verify this file reflects current project state. -->

This is the canonical agent-guidance file. `CLAUDE.md`, `AGENTS.md`, `GEMINI.md`, `.cursorrules`,
`.windsurfrules` and `.github/copilot-instructions.md` are all real git symlinks pointing here —
edit **this file**, never one of those pointers.

## What lives where

- **Agent/AI guidance** (this file) — tooling, build commands, lint policy, CI shape, traps.
- **User-facing docs** — `README.md` (what the app is, how to run it) and `CONTRIBUTING.md`
  (how to set up a dev environment and submit changes). Don't duplicate content between the two
  sets; if something belongs in both, link rather than copy.

## Architecture

Tauri v2 desktop app + Bun web server sharing a Svelte frontend.

```
Desktop:  Svelte SPA ←→ Tauri invoke() ←→ Rust commands  (WebView2 + single .exe)
Web:      Svelte SPA ←→ fetch('/api/...') ←→ Bun server   (bun server/index.ts)
```

## Project Structure

```
.ai/
  instructions.md   — this file (canonical agent guidance; the other pointers are symlinks to it)

src-tauri/          — Tauri v2 Rust crate (backend)
  src/
    main.rs         — entry point
    lib.rs          — AppState, tauri builder, command registration
    camera.rs       — CameraInfo struct
    config.rs       — AppConfig, JSON load/save
    fetcher.rs      — HTTP fetch (GeoJSON + images), disk save
    error.rs        — AppError (serde-serializable for IPC)
    commands/       — Tauri command handlers
      camera.rs     — get_camera_list, refresh_camera_list, fetch_image
      config.rs     — get_config, save_config
      disk.rs       — clear_cache, get_cache_info
  capabilities/     — Tauri v2 permission grants
  icons/            — app icon set (generated) + ATTRIBUTION.md
  Cargo.toml
  build.rs
  tauri.conf.json

public/             — static assets served as-is by Vite (favicon.png etc.)

src/                — Svelte frontend (shared by Tauri + web)
  App.svelte        — root component: load config → load cameras → start timer
  main.ts           — Svelte mount
  app.css           — global styles
  lib/
    api/
      index.ts      — environment detection: `isTauri`, exports `api`
      types.ts      — TypeScript mirrors of Rust structs
      tauri.ts      — invoke() wrappers (Tauri mode)
      web.ts        — fetch('/api/...') wrappers (web mode)
    stores/
      config.ts     — appConfig, pendingConfig, applyConfig(), cancelConfig()
      cameras.ts    — allCameras, visibleCameras, cameraImages
      refresh.ts    — lastRefresh, countdown, triggerRefreshAll()
    components/
      CameraGrid.svelte
      CameraCell.svelte     — idle/loading/ready/error + context menu
      StatusBar.svelte
      SettingsPanel.svelte
      settings/
        DistrictsSection.svelte
        CamerasSection.svelte
        DisplaySection.svelte
        DiskSaveSection.svelte

server/             — Bun HTTP server (web mode only)
  index.ts          — Bun.serve() + static file serving from dist/
  routes/           — API route handlers
  services/         — fetcher, disk save, config persistence
```

Config is persisted as JSON:
- **Tauri (desktop):** `%APPDATA%\au.com.pfeerick.traffic-camera-viewer\config.json` (Windows)
- **Web (Bun server):** `~/.config/traffic-camera-viewer/config.json`

## Build & Run

```bash
# One-time setup: installs the pinned toolchain (rust, bun, commitizen, pre-commit via mise),
# then `bun install` + activates the pre-commit hooks
mise install && mise run setup

# Frontend development (Vite dev server)
bun run dev

# Build frontend for production
bun run build

# Tauri desktop development (launches Vite + Tauri window)
bun run tauri:dev

# Tauri desktop release build
bun run tauri:build

# Rust unit tests (run from src-tauri/)
cd src-tauri && cargo test

# Rust lint
cd src-tauri && cargo clippy -- -D warnings
cd src-tauri && cargo fmt --check

# Frontend lint + format (Biome + Prettier)
bun run check
bun run check:fix

# Frontend + server type checking (svelte-check + server tsc)
bun run typecheck

# Web server (after bun run build)
bun run server
# or: bun server/index.ts
```

## Commit Convention

All commits must follow [Conventional Commits](https://www.conventionalcommits.org).
The `commit-msg` pre-commit hook (a local `commitizen` hook, see `.pre-commit-config.yaml`)
enforces this. See CONTRIBUTING.md for the full type list.

```
<type>(<optional scope>): <short summary>
```

## Tooling

Toolchain versions are pinned in `.mise.toml` (Rust via rustup, Bun, `pipx:commitizen`,
`pipx:pre-commit`). Run `mise install && mise run setup` once per clone — this installs
dependencies and activates the git hooks below.

### Pre-commit hooks

`mise run setup` runs `pre-commit install --hook-type pre-commit --hook-type commit-msg`, so
this normally doesn't need doing by hand. Hooks that run on every commit (`.pre-commit-config.yaml`):
- `cargo-fmt` — auto-formats Rust source (`src-tauri/`)
- pre-commit-hooks hygiene suite — `mixed-line-ending`, `check-yaml`, `check-toml`, `check-json`,
  `detect-private-key`, `end-of-file-fixer`, `trailing-whitespace`, `check-merge-conflict`
- `biome` — `bun run check:fix` (Biome + Prettier, covers both halves of the frontend)
- `commitizen` (`cz check`, commit-msg stage) — rejects non-conventional commit messages

### Semantic versioning

```bash
cz bump        # scan commits → determine next semver → update version files → commit + tag + push
cz changelog   # regenerate CHANGELOG.md without bumping
```

Config in `.cz.toml`. `version_files` covers `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`
and `package.json`, so `cz bump` keeps all three in sync — no manual step needed.
`post_bump_hooks` already pushes the bump commit and tag atomically
(`git push --atomic --follow-tags origin HEAD`); never add a separate push step.

**Windows:** Git for Windows sets `core.autocrlf=true` globally, which conflicts with this
repo's `eol=lf` policy. Fix once per clone:
```bash
git config --local core.autocrlf input
git config --local core.safecrlf false
git config --local core.symlinks true
```
The last one matters here specifically: the AI-guidance pointer files (`CLAUDE.md`, `AGENTS.md`,
etc.) are real symlinks (mode `120000`) to `.ai/instructions.md`. Without `core.symlinks true`,
git writes them out as plain text files containing the target path, and a later edit-and-commit
from that clone would silently convert the symlink into a real file for everyone else. Requires
Windows Developer Mode enabled (for unprivileged symlink creation) — turn it on *before* cloning.

If you commit from an editor UI (rather than a terminal) on Windows, mise's shims dir
(`%LOCALAPPDATA%\mise\shims`) needs to be on the *persistent* PATH — `mise activate` only updates
PATH when a shell prompt is drawn, which an editor's own commit UI never does. Without it, the
`cz` commit-msg hook resolves nothing from PATH and the commit fails there while working fine
from a terminal. Restart the editor fully after changing PATH.

## Lint Policy

**Rust** (`src-tauri/`): CI enforces `cargo fmt --check` and `cargo clippy -- -D warnings`. The
project also targets zero warnings under `-W clippy::pedantic -W clippy::nursery`. Prefer fixing
the code over silencing lints. Use narrow inline `#[allow(...)]` with a comment. There is no
`[lints.clippy]` block in `Cargo.toml`.

**Frontend** (`src/`, `server/`): Biome owns `.ts`/`.js`/`.json`/`.css` (lint + format, config in
`biome.json`). Prettier + `prettier-plugin-svelte` owns `.svelte` files end-to-end — markup,
`<script>` and `<style>` blocks alike (config in `.prettierrc`). The two tools claim **disjoint**
globs; `biome.json`'s `files.includes` excludes `**/*.svelte` so they never fight over the same
file. `bun run check` runs both; `bun run check:fix` applies fixes from both. Type correctness is
covered separately by `bun run typecheck` (`svelte-check` for `src/`, including `.svelte` files;
`tsc -p server/tsconfig.json` for `server/`, which needs `types: ["bun"]`).

## CI / GitHub Actions

| Workflow | Trigger | Jobs |
|----------|---------|------|
| `ci.yml` | every push + PR (all branches), ignoring changes that only touch `**.md`/`LICENSE`/`release.yml` | `hooks` (pre-commit hygiene suite), `commit-messages` (PR-only, Conventional Commits check via `cz check`), `lint` (Biome + Prettier), `typecheck` (svelte-check + server tsc), `lint-rust` (fmt + clippy), `test-rust`, `build` (3 platforms, gated on all of the above); artifacts uploaded on `main` only |
| `release.yml` | `workflow_run` on `ci.yml` completing for `main` | downloads artifacts; detects version tag; publishes rolling `latest` or versioned release, with notes extracted from `CHANGELOG.md` |

Push to `main` (no tag) → rolling `latest` pre-release updated.
Push a `v*` tag (via `cz bump`) → versioned release created.

Release platforms: Windows x86_64, Linux x86_64, macOS arm64.

Every `jdx/mise-action` step in CI passes `install_args` (e.g. `bun`, `pipx:pre-commit`,
`pipx:commitizen`) — `.mise.toml` also lists `rust`, and a bare `jdx/mise-action@v4` call would
install the entire Rust toolchain in jobs that don't need it. Jobs that do need Rust
(`lint-rust`, `test-rust`, `build`) keep `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`
instead of mise, for better caching and cross-platform target handling.

## Traps found while building this tooling

- `biome.json` is strict JSON, **not** JSONC — a `//` comment doesn't error, it makes Biome
  silently discard the whole config and fall back to tab indent.
- Biome and Prettier own disjoint globs; never let both claim `.svelte`. Biome's Svelte support
  is experimental and covers only the `<script>` block, leaving markup and `<style>` untouched.
- `.cz.toml` carries an inline `version` (not `version_provider = "cargo"`) because commitizen's
  cargo provider reads the **root** `Cargo.toml`, which is workspace-only
  (`[workspace] members = ["src-tauri"]`) — it raises `NonExistentKey` on a manifest with no
  `[package]` table.
- `post_bump_hooks` already pushes the bump commit and tag atomically — never add a separate
  push step, or the tag can end up pushed without the commit (or vice versa) on a race.
- `.mise.toml`'s `rust` entry is rustup-backed and only bootstraps rustup; `rust-toolchain.toml`
  stays the source of truth for the actual channel. Keep the two in agreement.
- Every `jdx/mise-action` call in CI must pass `install_args`, or it installs the whole Rust
  toolchain even in jobs (like `lint` or `hooks`) that only need Bun or a pipx tool.
- Building on Windows needs VS Build Tools with the C++ workload plus the Windows SDK. Without
  them, `link.exe` resolves to GNU coreutils' `link` instead of the MSVC linker, and every build
  script fails with `link: extra operand`.

The Rust crate is in `src-tauri/` — always run `cargo` commands from that directory.
