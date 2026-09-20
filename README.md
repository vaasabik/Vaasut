# ⚙ Vaasut Engine

A cross-platform game engine and level editor written in Rust.

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust |
| Graphics | wgpu (WebGPU / Vulkan / Metal / DX12) |
| UI | egui (Immediate Mode GUI) |
| Framework | eframe |
| Physics | rapier (planned) |
| ECS | bevy_ecs (planned) |

## Platforms

- 🖥 Windows / macOS / Linux (native)
- 📱 Android / iOS (native via wgpu)
- 🌐 Web (WASM + WebGPU/WebGL)

## Quick Start

### Web (recommended for testing)
Just push to `main` and GitHub Actions will deploy to GitHub Pages automatically.

### Desktop
```bash
cargo run
```

## License

MIT OR Apache-2.0
