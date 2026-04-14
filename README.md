# Traffic Camera Viewer

[![CI](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/ci.yml/badge.svg)](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/ci.yml)
[![Release](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/release.yml/badge.svg)](https://github.com/pfeerick/traffic-camera-viewer/actions/workflows/release.yml)

A Rust desktop app for viewing Queensland traffic cameras in a configurable grid with automatic refresh.

## Features

- Fetches camera list from QLD traffic GeoJSON feed.
- Configurable district filtering.
- Configurable grid layout, spacing, and image aspect ratio.
- Optional camera title display with configurable size and color.
- Configurable grid background color.
- Auto-refresh with countdown status bar.
- Manual "Refresh Now" action.
- Optional rolling image snapshots saved to disk.
- Skips texture updates when camera image bytes are unchanged.

## Build and Run

```bash
cargo build
cargo build --release
cargo run
cargo test
```

For contributor setup, workflow, and platform notes, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Usage

1. Start the app with `cargo run`.
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
