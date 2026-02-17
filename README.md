<div align="center">

# ⛰ sysinfo-art

**Your terminal becomes a living painting.**

*System metrics transformed into a breathing ASCII landscape — oceans that surge with your CPU, rain that falls from disk I/O, fire that rises with temperature, and wind that carries your network traffic.*

[![crates.io](https://img.shields.io/crates/v/sysinfo-art?style=flat-square&color=fc8d62&logo=rust)](https://crates.io/crates/sysinfo-art)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/shariquetelco/sysinfo-art/ci.yml?style=flat-square)](https://github.com/shariquetelco/sysinfo-art/actions)
[![GitHub Stars](https://img.shields.io/github/stars/shariquetelco/sysinfo-art?style=flat-square&color=yellow)](https://github.com/shariquetelco/sysinfo-art/stargazers)

</div>

---

```
╭────────────────────────────────────────────────────────────────────────────────╮
│ ⛰  sysinfo-art │ CPU 73%  RAM 61%  TEMP 68°C  DISK 45%  NET ↓2.1 ↑0.4 MB/s  │
╰────────────────────────────────────────────────────────────────────────────────╯

╭─ CPU OCEAN — 73.2% ────────────────────────────────────────────────────────────╮
│                ~≈∿~≈~    ~≈∿~≈~    ~≈∿~≈~    ≈~∿≈~≈    ~≈∿~≈~   ≈~∿≈~≈      │
│          ▁▂▂▃▄▄▅▅▆▆▇▇████████████████████▇▇▆▆▅▅▄▄▃▃▂▂▁▁▂▂▃▃▄▄▅▅▆▆▇▇███      │
│     ▁▂▃▄▄▅▅▆▆▇▇████████████████████████████████████████▇▇▆▆▅▅▄▄▃▃▂▂▁▁▂▂▃    │
│  ▂▃▄▄▅▆▆▇▇████████████████████████████████████████████████████▇▇▆▆▅▅▄▄▃▃▂▂   │
│ ▂▃▄▄▅▅▆▆▇▇██████████████████████████████████████████████████████████▇▇▆▆▅▅▄  │
│ ████████████████████████████████████████████████████████████████████████████  │
╰────────────────────────────────────────────────────────────────────────────────╯

╭─ DISK RAIN — 45% ──╮ ╭─ RAM TANK — 9.8/16.0 GB ──╮ ╭─ ♨ TEMP — 68°C ──────╮
│   │  │      ╷       │ │   〜∿≈~〜∿≈~〜∿≈~〜       │ │   *  s  s  s  *  *   │
│  ╎   │  ╷   │  ╷    │ │ ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  │ │  sS  S  s xS  x  S  │
│ · ○· ╎  │  ╎│  │    │ │ ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │ │ SSS  #$#  S  xS  S  │
│   ·   · │   │  ╎    │ │ ▓▓▓▓○▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  │ │  S  #$$#  xS #$#    │
│  ╷      │  ╎│  │    │ │ ████████████████████████  │ │ s #$$$#  S#$$$#  S   │
╰────────────────────╯ ╰───────────────────────────╯ ╰──────────────────────╯

╭─ NET WIND ↓2.1 ↑0.4 MB/s ─────────────────────────────────────────────────────╮
│  ⟿━━━━━━━━≡≡──   ⟿━━━━━━━━≡≡──     ─⟿━━━━━━━━━━≡≡──   ·  ·   ·      ·     │
│   ⟿━━━━━━━━━━≡≡──   · ⟿━━━━━━━━≡≡──   ⟿━━━━━━━━≡≡──  ·  ·   ·     ·        │
╰────────────────────────────────────────────────────────────────────────────────╯
```

---

## What is sysinfo-art?

`sysinfo-art` is a **Rust terminal application** that reads your system's live hardware metrics and renders them as an animated ASCII landscape.

| Metric | Visual |
|--------|--------|
| 🌊 **CPU** | Ocean waves — calmer seas = idle CPU, violent storms = 100% load |
| 🌧 **Disk** | Rainfall — light drizzle to torrential downpour |
| 🪣 **RAM** | Water tank — filling up as memory is consumed, bubbles rising |
| 🔥 **Temperature** | Fire intensity (hot chips) or ice crystals (cool system) |
| 💨 **Network** | Wind streaks — gentle breeze to gale-force gusts |

The terminal **feels alive** — every frame is unique and reacts to what your system is actually doing.

---

## Install

### From source (recommended right now)

```bash
git clone https://github.com/shariquetelco/sysinfo-art
cd sysinfo-art
cargo build --release
./target/release/sysinfo-art
```

### From crates.io *(coming soon)*

```bash
cargo install sysinfo-art
```

---

## Usage

```bash
sysinfo-art                # default 150ms refresh
sysinfo-art --rate 100     # faster refresh (ms)
sysinfo-art --rate 500     # slower refresh (ms)
```

**Keyboard shortcuts:**

| Key | Action |
|-----|--------|
| `q` or `Esc` | Quit |
| `Ctrl+C` | Quit |
| `?` or `h` | Toggle help |

---

## Requirements

- Rust 1.70+
- A terminal with 256-color or true-color support
- A screen at least 80×24 characters

Works on **Linux**, **macOS**, and **Windows** (via Windows Terminal).

---

## Architecture

```
sysinfo-art/
├── src/
│   ├── main.rs            # Entry point, event loop
│   ├── app.rs             # Application state
│   ├── metrics.rs         # System metrics (sysinfo crate)
│   ├── ui.rs              # ratatui layout & rendering
│   └── animations/
│       ├── ocean.rs       # Wave simulation (CPU)
│       ├── rain.rs        # Raindrop physics (Disk)
│       ├── water.rs       # Fluid fill with bubbles (RAM)
│       ├── fire.rs        # Fire/ice simulation (Temperature)
│       └── wind.rs        # Particle streaks (Network)
├── Cargo.toml
└── README.md
```

**Tech stack:**
- [`ratatui`](https://github.com/ratatui-org/ratatui) — terminal UI framework
- [`crossterm`](https://github.com/crossterm-rs/crossterm) — terminal backend
- [`sysinfo`](https://github.com/GuillaumeGomez/sysinfo) — system metrics
- [`clap`](https://github.com/clap-rs/clap) — CLI arguments

---

## Roadmap

- [ ] Demo GIF in README
- [ ] Publish to crates.io
- [ ] Config file (`~/.config/sysinfo-art/config.toml`)
- [ ] Themes: `cyberpunk`, `nature`, `monochrome`
- [ ] Per-core CPU waves
- [ ] Historical graphs layered under animations
- [ ] `--record` flag to export frames
- [ ] macOS + Windows CI

---

## Contributing

PRs and issues are welcome! If you've got an idea for a new animation or metric, open an issue first to discuss.

```bash
git clone https://github.com/shariquetelco/sysinfo-art
cd sysinfo-art
cargo run         # dev build
cargo clippy      # lints
cargo test        # tests
```

---

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">

**If this made your terminal feel alive, give it a ⭐**

Made with ❤️ and Rust.

</div>
