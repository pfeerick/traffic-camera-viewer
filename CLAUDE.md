# Traffic Camera Viewer — Claude Code Notes

## Build & Run

```bash
cargo build              # debug build
cargo build --release    # release build (no console window on Windows)
cargo run                # build + run debug
cargo test               # run unit tests
```

## Windows Setup

`cargo` must be on PATH for Claude Code's bash shell to find it. Add the following to
`~/.claude/settings.json` (global user settings, not committed to this repo):

```json
{
  "env": {
    "PATH": "C:\\Users\\<you>\\.cargo\\bin;${PATH}"
  }
}
```

This is already configured for the primary developer on this machine. On Linux/macOS, `rustup`
adds `~/.cargo/bin` to the shell profile automatically and no extra config is needed.
