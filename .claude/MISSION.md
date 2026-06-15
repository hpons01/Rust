# Mission: Rust

## Why
Learn Rust to build embedded systems projects — coming from C. Target: write firmware for ESP32 in `no_std` and eventually work as a Rust embedded engineer.

## End goals
- **Project 1** — ESP32 Weather Station: `no_std`, `esp-hal` + Embassy, BME280 driver with type-state, zero alloc
- **Project 2** — Polyphonic Synthesizer: real-time DMA/I2S, fixed-point DSP, lock-free dual-core
- **Project 3** — Hardware Pentest Probe: multi-crate workspace, UART/I2C/SPI sniffer, firmware + TUI (capstone)
- Intermediate: CLI tool parsing SPI/UART captures from PulseView CSV exports (Bloc 3 deliverable)
## Learning path — 5 Blocs (~36 sessions)
| Bloc | Theme | Cadence |
|---|---|---|
| 🔵 Bloc 1 | Syntax basics | Chainable |
| 🟣 Bloc 2 | Ownership ← **current wall** | 1 session/day max |
| 🟠 Bloc 3 | Modules, collections, errors | Chainable |
| 🔴 Bloc 4 | Generics, traits, lifetimes | Chainable (lifetimes: 1/day) |
| ⚫ Bloc 5 | Smart pointers, concurrency, async | Chainable |

**Golden rule:** Rustlings *after* each session, never before. Quiz < 80% or Rustlings > 15 min → slow down, sleep on it.

## Resources
- **Primary:** Brown Book (rust-book.cs.brown.edu) + Rustlings
- **Embedded:** The Embedded Rust Book, The Rust on ESP Book, Embassy, embedded-hal
- **ESP32:** esp-hal, espup, espflash, probe-rs, defmt

## Constraints
- Background: C (embedded) — leverage register/DMA/ISR knowledge, avoid writing "C in Rust"
- 4 mental shifts to internalize: ownership vs raw pointers · Option/Result vs error codes · type-state for drivers · message-passing concurrency
- Sessions tracked in Notion

## Out of scope (until Bloc 5+)
- Web Rust, Wasm, GUI, game dev