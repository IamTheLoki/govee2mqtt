# 💡 Govee to MQTT Bridge for Home Assistant

[![GitHub Container Registry](https://img.shields.io/badge/GHCR-ghcr.io%2Fiamtheloki%2Fgovee2mqtt-blue?style=for-the-badge&logo=github)](https://github.com/IamTheLoki/govee2mqtt/pkgs/container/govee2mqtt)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Home Assistant](https://img.shields.io/badge/Home_Assistant-MQTT-18BCF2?style=for-the-badge&logo=homeassistant&logoColor=white)](https://www.home-assistant.io/integrations/mqtt/)
[![Fork of wez/govee2mqtt](https://img.shields.io/badge/Fork_of-wez%2Fgovee2mqtt-lightgrey?style=for-the-badge&logo=github)](https://github.com/wez/govee2mqtt)

A [Govee](https://govee.com) to [Home Assistant](https://www.home-assistant.io/) bridge written in Rust. The `govee` executable talks to your devices via LAN, the Govee cloud and the AWS IoT channel, and publishes them through the [Home Assistant MQTT Integration](https://www.home-assistant.io/integrations/mqtt/).

This is a fork of [wez/govee2mqtt](https://github.com/wez/govee2mqtt) with support for the **H5192 WiFi meat thermometer**, a slimmed-down CI and container images published to this repository.

---

## ✨ Features

- **LAN-first design:** Devices that support the LAN API are controlled with the lowest latency, even when your internet connection is offline.
- **Modes and Scenes:** Per-device modes, scenes, DIY scenes, music modes and segment colors.
- **AWS IoT Support:** Uses the undocumented IoT interface for low latency status updates and One Click scenes.
- **Platform API Fallback:** Uses the [Platform API](https://developer.govee.com/reference/get-you-devices) when IoT or LAN control is unavailable.
- **Meat Thermometer (H5192):** Core and ambient temperature per probe, plus the alarm limits (see [below](#h5192)).
- **Home Assistant Discovery:** Entities show up automatically, including the room as area when you configure your Govee account.
- **Small Footprint:** Single static binary built with Rust and Tokio.

| Feature | Requires | Notes |
| :--- | :---: | :--- |
| DIY Scenes | API Key | Find in the list of Effects for the light in Home Assistant |
| Music Modes | API Key | Find in the list of Effects for the light in Home Assistant |
| Tap-to-Run / One Click Scene | IoT | Find in the overall list of Scenes in Home Assistant, as well as under the `Govee to MQTT` device |
| Live Device Status Updates | LAN and/or IoT | Devices typically report most changes within a couple of seconds |
| Segment Color | API Key | Find the `Segment 00X` light entities associated with your main light device in Home Assistant |
| Meat Thermometer H5192 | IoT | Polled via IoT, see [H5192 Meat Thermometer](#h5192) |

- `API Key` means that you have [applied for a key from Govee](https://developer.govee.com/reference/apply-you-govee-api-key) and have configured it for use in govee2mqtt.
- `IoT` means that you have configured your Govee account email and password, which makes govee2mqtt use the *undocumented and likely unsupported* AWS MQTT-based IoT service.
- `LAN` means that you have enabled the [Govee LAN API](https://app-h5.govee.com/user-manual/wlan-guide) on supported devices and that the LAN API protocol works on your network.

---

## 🚀 Quick Start

You need a running MQTT broker that is configured in Home Assistant ([instructions](https://www.home-assistant.io/integrations/mqtt/#configuration)).

### Option 1: Docker Compose (Recommended)

Add `govee2mqtt` to your `docker-compose.yml`:

```yaml
name: govee2mqtt
services:
  govee2mqtt:
    image: ghcr.io/iamtheloki/govee2mqtt:latest
    container_name: govee2mqtt
    restart: unless-stopped
    # Host networking is required for LAN discovery
    network_mode: host
    environment:
      GOVEE_EMAIL: "user@example.com"
      GOVEE_PASSWORD: "secret"
      GOVEE_API_KEY: "your-api-key"
      GOVEE_MQTT_HOST: "mqtt"
      GOVEE_MQTT_PORT: "1883"
      GOVEE_TEMPERATURE_SCALE: "C"
      TZ: "Europe/Berlin"
    # By default a Docker volume persists /data (cache and IoT key).
    # To mount it on your host instead:
    # volumes:
    #   - /path/to/data:/data
```

You can also keep the values in a `.env` file and reference it with `env_file`. See [docs/DOCKER.md](docs/DOCKER.md) for a full skeleton.

Run with:

```bash
docker compose up -d
docker logs govee2mqtt --follow
```

### Option 2: Docker CLI

```bash
docker run -d \
  --name govee2mqtt \
  --network host \
  -e GOVEE_EMAIL="user@example.com" \
  -e GOVEE_PASSWORD="secret" \
  -e GOVEE_API_KEY="your-api-key" \
  -e GOVEE_MQTT_HOST="mqtt" \
  ghcr.io/iamtheloki/govee2mqtt:latest
```

### Option 3: Building from Source

**Prerequisites:** [Rust & Cargo](https://rustup.rs/), a C toolchain and OpenSSL headers

```bash
# Clone the repository
git clone https://github.com/IamTheLoki/govee2mqtt.git
cd govee2mqtt

# Build release binary
cargo build --release

# Run the bridge
export GOVEE_EMAIL="user@example.com"
export GOVEE_PASSWORD="secret"
export GOVEE_MQTT_HOST="mqtt"
./target/release/govee serve
```

*Run it from the repository directory: the binary looks for `AmazonRootCA1.pem` and `assets/` relative to the working directory.*

---

## ⚙️ Configuration

Configure the bridge using environment variables (every option also exists as a CLI flag, see [docs/CONFIG.md](docs/CONFIG.md)):

| Environment Variable | Default | Required | Description |
| :--- | :---: | :---: | :--- |
| `GOVEE_MQTT_HOST` | — | **Yes** | Host name or IP address of your MQTT broker (the one Home Assistant uses) |
| `GOVEE_MQTT_PORT` | `1883` | No | Port of the MQTT broker |
| `GOVEE_MQTT_USER` | — | No | MQTT username, if your broker requires authentication |
| `GOVEE_MQTT_PASSWORD` | — | No | MQTT password, if your broker requires authentication |
| `GOVEE_EMAIL` | — | Recommended | Govee account email, enables IoT, room names and the H5192 |
| `GOVEE_PASSWORD` | — | Recommended | Govee account password |
| `GOVEE_API_KEY` | — | No | Platform API key, needed for scenes on non-LAN devices |
| `GOVEE_TEMPERATURE_SCALE` | `C` | No | Temperature unit shown in Home Assistant (`C` or `F`) |
| `GOVEE_LAN_SCAN` | — | No | Comma separated list of IPs or broadcast addresses to probe, e.g. `10.0.0.1,10.0.0.2` |
| `GOVEE_LAN_NO_MULTICAST` | `false` | No | Do not send multicast discovery packets |
| `GOVEE_LAN_BROADCAST_ALL` | `false` | No | Send discovery packets to the broadcast address of every network interface |
| `GOVEE_LAN_BROADCAST_GLOBAL` | `false` | No | Send discovery packets to `255.255.255.255` |
| `RUST_LOG` | `info` | No | Logging verbosity, e.g. `govee=debug` or `govee=trace` |
| `TZ` | — | No | Time zone for log timestamps |

---

<a id="h5192"></a>

## 🌡️ H5192 Meat Thermometer

The H5192 does not push its readings, it only answers requests. The bridge therefore polls it over the AWS IoT channel, which requires `GOVEE_EMAIL` and `GOVEE_PASSWORD`.

| Entity | Type | Description |
| :--- | :---: | :--- |
| `Probe 1 Core`, `Probe 2 Core` | Sensor | Core (food) temperature per probe |
| `Probe 1 Ambient`, `Probe 2 Ambient` | Sensor | Ambient temperature per probe |
| `Probe N Core Max/Min`, `Probe N Ambient Max/Min` | Diagnostic sensor | Alarm limits configured in the Govee app (read-only) |
| `Live Polling` | Switch | Polls every 15 seconds and refreshes the limits every minute. Off by default |

- An unplugged probe or an unset limit shows up as `unknown`.
- With `Live Polling` off, the thermometer is read every 15 minutes. Switch it on while cooking and off afterwards, as frequent polling costs battery.
- `Live Polling` is not persisted and starts as off after a restart.
- Alarm limits cannot be changed from Home Assistant yet, set them in the Govee app.

---

## 🖥️ Web UI & API

The bridge serves a small web UI and a REST API on port `8056`:

| URL | Description |
| :--- | :--- |
| `http://<host>:8056/` | Device list (redirects to `/assets/index.html`) |
| `http://<host>:8056/api/devices` | All devices with their state as JSON |
| `http://<host>:8056/api/device/<id>/power/on` | Switch a device on (`off` to switch off) |

*The web UI has no authentication. Do not expose it to untrusted networks.*

---

## ❓ Documentation

- [Running it in Docker](docs/DOCKER.md)
- [Configuration](docs/CONFIG.md)
- [Is my device supported?](docs/SKUS.md)
- [LAN API requirements](docs/LAN.md)
- [FAQ](docs/FAQ.md)
- [Privacy](docs/PRIVACY.md)
- [Changelog](CHANGELOG.md)

---

## 🤝 Credits

This is a fork of [wez/govee2mqtt](https://github.com/wez/govee2mqtt) by Wez Furlong, who also deserves your support: [GitHub Sponsors](https://github.com/sponsors/wez), [Patreon](https://patreon.com/WezFurlong), [Ko-fi](https://ko-fi.com/wezfurlong) and [Liberapay](https://liberapay.com/wez).

- The work is based on [Govee LAN Control](https://github.com/wez/govee-lan-hass/).
- The AWS IoT support was made possible by the work of @bwp91 in [homebridge-govee](https://github.com/bwp91/homebridge-govee/).
- The H5192 protocol was reverse-engineered in [lasswellt/govee-homeassistant#174](https://github.com/lasswellt/govee-homeassistant/issues/174).

---

## ☕ Support

If you like this fork, you can support it with a coffee:

[![Buy Me a Coffee](https://img.buymeacoffee.com/button-api/?text=Buy%20me%20a%20coffee&emoji=&slug=looki1980b&button_colour=FFDD00&font_colour=000000&font_family=Cookie&outline_colour=000000&coffee_colour=ffffff)](https://buymeacoffee.com/looki1980b)

---

## 📄 License

This project is licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.
