# Traffic Camera Viewer

[![CI](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/ci.yml)
[![Release](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/release.yml/badge.svg)](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/release.yml)

A Tauri v2 desktop app (and optional Bun web server) for viewing Queensland traffic cameras in a configurable grid with automatic refresh.

## Features

- Fetches camera list from the QLD Traffic GeoJSON feed.
- Configurable district and camera filtering with custom display order.
- Configurable grid layout, spacing, and image aspect ratio.
- Optional camera title display with configurable size, colour, and alignment.
- Configurable grid background colour.
- Auto-refresh with countdown status bar.
- Manual "Refresh Now" and per-camera refresh actions.
- Optional rolling image snapshots saved to disk.
- Skips image updates when camera bytes are unchanged (hash check).
- Runs as a native desktop app (Tauri + WebView2) or as a browser app (Bun server).

## Build and Run

```bash
# Install frontend dependencies
bun install

# Desktop development (Vite dev server + Tauri window)
bun run tauri:dev

# Desktop release build
bun run tauri:build

# Web mode (build frontend then start Bun server)
bun run build && bun run server

# Rust unit tests
cd src-tauri && cargo test
```

For full contributor setup, workflow, and platform notes, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Usage

1. Start the app with `bun run tauri:dev` (desktop) or `bun run build && bun run server` (web).
2. Open **Settings** from the status bar.
3. Select one or more districts.
4. Adjust display and refresh options.
5. Click **Apply** to persist changes.

## Data Source

Camera metadata is loaded from:

- `https://data.qldtraffic.qld.gov.au/webcameras.geojson`

## License

This project is licensed under the GNU General Public License v3.0.
See [LICENSE](LICENSE) for the full text.

## Attribution

Application icon — [CCTV](https://pictogrammers.com/library/mdi/icon/cctv/) by
[Material Design Icons](https://pictogrammers.com/library/mdi/), Apache 2.0.
See [src-tauri/icons/ATTRIBUTION.md](src-tauri/icons/ATTRIBUTION.md) for full details.
