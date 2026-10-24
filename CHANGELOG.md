# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This is a fork of [wez/govee2mqtt](https://github.com/wez/govee2mqtt).

## [2.0.2] - 2026-10-24

Security release: resolves all open Dependabot alerts. No functional changes.

### Security
- Updated transitive dependencies in `Cargo.lock` to patched versions (via `cargo update`):
  - `rustls` 0.23.35 -> 0.23.46 (TLS 1.3 handshake messages accepted across encryption
    level boundaries)
  - `rustls-webpki` 0.103.8 -> 0.103.16 (DoS via panic on a malformed CRL, CRL distribution
    point matching, name constraint handling)
  - `bytes` 1.11.0 -> 1.12.1 (integer overflow in `BytesMut::reserve`)
  - `quinn-proto` 0.11.14 -> 0.11.19 (remote memory exhaustion) and `rand` 0.9.2 -> 0.10.3
    (unsound with a custom logger): both are only present in the lockfile through the
    optional HTTP/3 support of `reqwest` and are not compiled into the application

### Changed
- Updated `parking_lot` 0.12 -> 0.13
- Updated the remaining locked dependencies to their latest compatible versions

## [2.0.1] - 2026-10-22

Dependency update release. No functional changes.

### Changed
- Updated 17 cargo dependencies, among them `reqwest` 0.13.5, `openssl` 0.10.81,
  `csscolorparser` 0.9 and `tower-http` 0.7
- Updated GitHub Actions: `actions/checkout` 7, `peter-evans/create-pull-request` 8,
  `docker/setup-buildx-action` 4

## [2.0.0] - 2026-10-22

First release of this fork of [wez/govee2mqtt](https://github.com/wez/govee2mqtt).

### Added
- Support for the Govee H5192 WiFi meat thermometer (2 probes) via the AWS IoT channel
  - Per probe: core and ambient temperature sensors
  - Per probe: alarm limits (core max/min, ambient max/min) as read-only diagnostic sensors
  - Switch "Live Polling" (off by default): polls the device every 15 seconds and
    refreshes the limits every minute; otherwise the device is read every 15 minutes
- GitHub workflows:
  - `push.yml`: build, test, clippy (`-D warnings`) and format check on every push
  - `release.yml`: container image on release
  - `cargo-update.yml`: weekly `cargo update` pull request
- `AGENTS.md` with project rules and this `CHANGELOG.md`
- README rewritten in the style of the tado-exporter project, with an H5192 section
- Docs: FAQ entry for the H5192, H5192 in `docs/SKUS.md`, FAQ screenshot stored in `docs/images/`

### Changed
- Container image is published to `ghcr.io/iamtheloki/govee2mqtt`, for `linux/amd64` only,
  on release (tags `latest` and the release version)
- `dependabot.yml`: cargo and docker daily and grouped, github-actions weekly
- `docker-compose.yml` and the OCI source label point to this fork
- Home Assistant device info of the bridge: manufacturer and URL point to this fork
- Docs: fork image name and example time zone in `docs/DOCKER.md`, issue links in
  `docs/FAQ.md` point to this fork, "AddOn" column and add-on hints removed from
  `docs/CONFIG.md` and `docs/LAN.md`

### Fixed
- IoT packets without a `state` field no longer fail to parse
- All `cargo clippy -- -D warnings` findings, including the `double_must_use`
  false positive of clippy 1.99 caused by `async_trait`

### Removed
- Home Assistant add-on packaging (`addon/`, `repository.yaml`, `.devcontainer/`,
  `docs/ADDON.md`, release scripts) and its CI jobs
- Upstream funding links, issue templates and the no-response workflow
- `build.yml` (replaced by `release.yml`) and `pr.yml` (replaced by `push.yml`)
