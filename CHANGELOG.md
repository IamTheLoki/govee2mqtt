# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This is a fork of [wez/govee2mqtt](https://github.com/wez/govee2mqtt).

## [Unreleased]

### Added
- Support for the Govee H5192 WiFi meat thermometer (2 probes) via the AWS IoT channel
  - Sensors per probe: core and ambient temperature, plus the alarm limits (core max/min, ambient max/min) as diagnostic sensors
  - Switch "Live Polling" (off by default): polls the device every 15 seconds and refreshes the limits every minute; otherwise the device is read every 15 minutes
- `AGENTS.md` with project rules
- README rewritten in the style of the tado-exporter project, with an H5192 section; H5192 listed in `docs/SKUS.md`
- FAQ entry for the H5192; the FAQ screenshot is now stored in `docs/images/`
- GitHub workflows `push.yml` (build, test, clippy, fmt on every push), `release.yml` (container image on release) and `cargo-update.yml` (weekly `cargo update` pull request)

### Changed
- Container image is published to `ghcr.io/iamtheloki/govee2mqtt` for `linux/amd64` only, on release (tags `latest` and the release version)
- `dependabot.yml`: cargo and docker daily and grouped, github-actions weekly
- `docker-compose.yml` and the OCI source label point to this fork
- IoT packets without a `state` field no longer fail to parse
- Home Assistant device info of the bridge: manufacturer and URL point to this fork instead of upstream
- Fixed all `cargo clippy -- -D warnings` findings
- Docs: image name and example time zone in `docs/DOCKER.md`, issue links in `docs/FAQ.md` point to this fork
- Docs: removed the "AddOn" column and add-on hints from `docs/CONFIG.md` and `docs/LAN.md`

### Removed
- Home Assistant add-on packaging (`addon/`, `repository.yaml`, `.devcontainer/`, `docs/ADDON.md`, release scripts) and its CI jobs
- Upstream funding links, issue templates and the no-response workflow
- `build.yml` (replaced by `release.yml`) and `pr.yml` (replaced by `push.yml`)
