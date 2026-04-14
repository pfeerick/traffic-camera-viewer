# Traffic Camera Viewer — Claude Code Notes

<!-- SELF-MAINTENANCE: Update this file whenever significant changes are made to the project
     (new modules, changed tooling, new workflows, dependency updates, architectural decisions).
     At the start of each session, verify this file reflects current project state. -->

## Build & Run

```bash
cargo build              # debug build
cargo build --release    # release build (no console window on Windows)
cargo run                # debug run
cargo test               # unit tests
cargo clippy -- -D warnings  # lint (must pass clean before committing)
cargo fmt --check            # formatting check (CI enforces this)
```

## Project Structure

```
src/
  main.rs       — tokio runtime setup, eframe::run_native
  app.rs        — AppState, eframe::App impl, refresh timer, promise polling
  camera.rs     — CameraInfo, CameraState, ImageState enum
  config.rs     — AppConfig (confy-persisted TOML), load/save
  fetcher.rs    — HttpClient, async GeoJSON + image fetch, rolling disk save
  ui/
    mod.rs
    grid.rs     — scrollable camera image grid
    settings.rs — settings side panel (Districts, Display, Disk Save)
    statusbar.rs — refresh controls, countdown progress bar
```

Config is persisted by `confy` at:
- **Windows:** `%APPDATA%\traffic-camera-viewer\config\traffic-camera-viewer.toml`
- **Linux:** `~/.config/traffic-camera-viewer/traffic-camera-viewer.toml`
- **macOS:** `~/Library/Application Support/traffic-camera-viewer/traffic-camera-viewer.toml`

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
- `cargo fmt` — auto-formats Rust source in place
- `conventional-pre-commit` — rejects non-conventional commit messages

### Semantic versioning

```bash
cz bump        # scan commits → determine next semver → update Cargo.toml → commit + tag
cz changelog   # regenerate CHANGELOG.md without bumping
```

Config in `.cz.toml`; version is sourced directly from `Cargo.toml`.

## CI / GitHub Actions

| Workflow | Trigger | Jobs |
|----------|---------|------|
| `ci.yml` | every push + PR | lint, test, build (all 3 platforms) |
| `release.yml` | push to `main` or `v*` tag | build artifacts → publish GitHub Release |

Push to `main` → updates rolling `latest` pre-release.
Push a `v*` tag (via `cz bump`) → creates a versioned release with auto-generated notes.

Release platforms: Windows x86_64, Linux x86_64, macOS arm64.
macOS Intel (`macos-15-intel`) is commented out in the matrix — uncomment to re-enable.

## Windows Setup (Claude Code shell)

`cargo` must be on PATH for Claude Code's bash shell. This is configured globally in
`~/.claude/settings.json` (not committed to this repo):

```json
{
  "env": {
    "PATH": "C:\\Users\\<you>\\.cargo\\bin;${PATH}"
  }
}
```

Already configured on the primary developer's machine.
On Linux/macOS, `rustup` adds `~/.cargo/bin` automatically.
