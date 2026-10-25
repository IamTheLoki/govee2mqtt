# Project Rules for govee2mqtt

## Automatic File Updates on Changes
- **Cargo.toml / Versioning:** On code changes, new features or bug fixes, always update the version number in `Cargo.toml` according to the SemVer convention.
- **Versioning:** Follow the SemVer convention for new releases:
  - **MAJOR:** Breaking changes or incompatible API changes
  - **MINOR:** New features (backwards compatible)
  - **PATCH:** Bug fixes and minor adjustments (backwards compatible)
- **CHANGELOG.md:** On every functional change or release, automatically update `CHANGELOG.md`. Add new features under `[Unreleased]` or the new version number.
- **Badges:** Keep the badges directly below the README title (GHCR, Rust, Home Assistant, Fork) up to date, e.g. on a new Rust edition (see `Cargo.toml`) or a changed image registry
- **Code style:** Strictly follow idiomatic Rust (Rust 2021 edition, see `Cargo.toml`).
- **Notification:** Explicitly inform the user about all automatic updates to Cargo.toml, CHANGELOG.md and other files according to these rules.
- **Clippy / Format:** Before every commit, `cargo clippy -- -D warnings` and `cargo fmt --all -- --check` must pass cleanly, otherwise the CI workflow fails.
