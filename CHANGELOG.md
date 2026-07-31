# Changelog

## v0.7.1 (2026-07-31)

### Fix

- **release**: sync Cargo.lock version and keep it updated on bump

## v0.7.0 (2026-07-31)

### Feat

- migrate from egui to Tauri v2 + Svelte + Bun (#2)

## v0.6.0 (2026-04-15)

### Feat

- **ui**: add camera title alignment control (left/center/right)

### Refactor

- log config load failures and add startup version log

## v0.5.3 (2026-04-15)

### Refactor

- remove all clippy suppressions by fixing root causes

## v0.5.2 (2026-04-14)

### Fix

- **ci**: checkout before artifact download to prevent workspace wipe

## v0.5.1 (2026-04-14)

### Fix

- **ci**: merge release workflow and gate publish on lint/test

## v0.5.0 (2026-04-14)

### Feat

- **ui**: add camera reordering and collapse settings sections by default
- **ui**: add per-camera hide/show via context menu and settings
- **ui**: add right-click context menu with per-camera refresh

### Fix

- persist window position, size, and maximised state across sessions

## v0.4.0 (2026-04-14)

### Feat

- add application icon

## v0.3.0 (2026-04-14)

### Feat

- show version and build date in window title and settings panel
- add Release status badge to README
- add CI status badge to README

## v0.2.1 (2026-04-14)

### Fix

- add x11/wayland eframe features and libwayland-dev for Linux builds

## v0.2.0 (2026-04-14)

### Feat

- avoid reloading the displayed texture when image unchanged

### Fix

- resolve clippy warnings in fetcher and settings

## v0.1.0 (2026-04-14)
