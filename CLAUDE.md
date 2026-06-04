# Traffic Camera Viewer — Claude Code Notes

<!-- SELF-MAINTENANCE: Update this file whenever significant changes are made to the project
     (new modules, changed tooling, new workflows, dependency updates, architectural decisions).
     At the start of each session, verify this file reflects current project state. -->

## Architecture

Tauri v2 desktop app + Bun web server sharing a Svelte frontend.

```
Desktop:  Svelte SPA ←→ Tauri invoke() ←→ Rust commands  (WebView2 + single .exe)
Web:      Svelte SPA ←→ fetch('/api/...') ←→ Bun server   (bun server/index.ts)
```

## Project Structure

```
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
  icons/            — app icon set (generated)
  Cargo.toml
  build.rs
  tauri.conf.json

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

# Web server (after bun run build)
bun run server
# or: bun server/index.ts
```

## Commit Convention

All commits must follow [Conventional Commits](https://www.conventionalcommits.org).
The `commit-msg` pre-commit hook enforces this. See CONTRIBUTING.md for the full type list.

```
<type>(<optional scope>): <short summary>
```

## Tooling

### Pre-commit hooks (activate once after cloning)

```bash
pre-commit install
pre-commit install --hook-type commit-msg
```

Hooks that run on every commit:
- `cargo fmt` — auto-formats Rust source (runs in `src-tauri/`)
- `conventional-pre-commit` — rejects non-conventional commit messages

### Semantic versioning

```bash
cz bump        # scan commits → determine next semver → update Cargo.toml → commit + tag
cz changelog   # regenerate CHANGELOG.md without bumping
```

Config in `.cz.toml`; version is sourced from `src-tauri/Cargo.toml`.

**Note:** Update version in both `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`.

**Windows:** Git for Windows sets `core.autocrlf=true` globally, which conflicts with this
repo's `eol=lf` policy. Fix once per clone:
```bash
git config --local core.autocrlf input
git config --local core.safecrlf false
```

## Lint Policy

CI enforces `cargo clippy -- -D warnings` (from `src-tauri/`). The project also targets zero
warnings under `-W clippy::pedantic -W clippy::nursery`.

Prefer fixing the code over silencing lints. Use narrow inline `#[allow(...)]` with a comment.
There is no `[lints.clippy]` block in `Cargo.toml`.

## CI / GitHub Actions

| Workflow | Trigger | Jobs |
|----------|---------|------|
| `ci.yml` | every push + PR (all branches) | lint-frontend, lint-rust, test-rust, build-tauri (3 platforms); artifacts on `main` only |
| `release.yml` | `workflow_run` on `ci.yml` completing for `main` | downloads artifacts; detects version tag; publishes rolling `latest` or versioned release |

Push to `main` (no tag) → rolling `latest` pre-release updated.
Push a `v*` tag (via `cz bump`) → versioned release created.

Release platforms: Windows x86_64, Linux x86_64, macOS arm64.

## Windows Setup (Claude Code shell)

`cargo` must be on PATH for Claude Code. Configured globally in `~/.claude/settings.json`:

```json
{
  "env": {
    "PATH": "C:\\Users\\<you>\\.cargo\\bin;${PATH}"
  }
}
```

The Rust crate is in `src-tauri/` — always run `cargo` commands from that directory.
