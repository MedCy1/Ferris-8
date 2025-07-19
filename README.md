# Ferris-8

A Chip-8 emulator written in Rust and compiled to WebAssembly to run in the browser.

![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)
![WebAssembly](https://img.shields.io/badge/WebAssembly-Ready-blue.svg)

## Why this project?

I wanted to learn WebAssembly and Rust at the same time, so I decided to build a Chip-8 emulator. It's a good beginner project: simple enough to actually finish, but with enough interesting technical challenges.

The web interface is modern with a dark design and some nice effects. It works well and runs pretty fast.

## What works

- All standard Chip-8 instructions (35 opcodes)
- 64x32 display with upscaling so you can actually see it
- Basic sound (a beep when needed)
- Virtual keyboard and physical keyboard support
- A few built-in test ROMs
- Debugger to see what's happening in real-time
- Responsive interface that works on mobile

## Installation

You'll need Rust and wasm-pack installed.

```bash
git clone <your-repo>
cd Ferris-8

# Script that does everything automatically
./launch.sh
```

Or manually:
```bash
wasm-pack build --target web --out-dir pkg --release
cd web
python3 -m http.server 8000
```

Then open http://localhost:8000

## How it works

The architecture is pretty standard:
- `src/` contains the Rust code (CPU, memory, display, etc.)
- `web/` contains the interface (HTML/CSS/JS)
- Everything is compiled to WebAssembly

Chip-8 to QWERTY keyboard mapping:
```
1 2 3 C    →    1 2 3 4
4 5 6 D    →    Q W E R  
7 8 9 E    →    A S D F
A 0 B F    →    Z X C V
```

## Test ROMs

I've included a few simple ROMs for testing:
- A blinking pixel (to test display)
- A moving square (to test sprites)
- Text display (to test fonts)

## Limitations

- No complex audio, just basic beeps
- Some advanced ROMs might not work perfectly
- The debugger is functional but could be more detailed
- Interface not necessarily optimal on very small screens

## Code structure

```
src/
├── lib.rs          # WebAssembly entry point
├── cpu.rs          # The emulator core
├── memory.rs       # 4KB memory management
├── display.rs      # Sprite rendering
└── input.rs        # Keyboard handling

web/
├── index.html      # User interface
├── style.css       # Interface design
└── main.js         # JavaScript logic
```

## Performance

It runs at 60 FPS without issues on a modern computer. The WebAssembly bundle is about 12KB when optimized, which is decent.

Emulation speed is configurable (100-1000Hz) depending on your preferences.

## Development

If you want to contribute or just look at the code:

```bash
# Unit tests
cargo test

# To rebuild in dev mode
wasm-pack build --target web --dev
```

The code isn't perfect but it's readable and commented. I tried to follow Rust best practices as much as possible.

## What could be improved

- More sophisticated audio with Web Audio API
- Support for more Chip-8 variants (Super-Chip, XO-Chip...)
- Better debug tools (disassembler, breakpoints...)
- Drag & drop for ROMs
- Save states
- More built-in test ROMs

## License

Licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

This means you can use, modify, and distribute the code, but you must:
- Keep the copyright notice
- Document any changes you make
- Include the Apache license in derivative works

---

A fun project for learning Rust and WebAssembly. Source code is available if you're interested.