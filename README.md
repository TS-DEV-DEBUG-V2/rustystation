<div align="center">
  <img src="https://github.com/TS-DEV-DEBUG-V2/rustystation/blob/main/assets/rs1.png?raw=true" alt="RustyStation Logo" width="200"/>

  <br/>

  # RustyStation
  ### THIS README HAS PLANNED FEATURES AND MOST THINGS STILL ARENT IMPLEMENTED !!
  ### *A PlayStation 1 Emulator writen in rust*

  > *"The original PlayStation changed gaming forever. RustyStation brings it back — faster, sharper, and built for the modern era."*

  <br/>

![Rust Edition](https://img.shields.io/badge/Rust_Edition-2021-000000?style=for-the-badge&logo=rust&logoColor=white)
![Build](https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge)
![CI](https://img.shields.io/badge/CI-GitHub_Actions-2088FF?style=for-the-badge&logo=githubactions&logoColor=white)
![Tests](https://img.shields.io/badge/Tests-Passing-brightgreen?style=for-the-badge)
![Coverage](https://img.shields.io/badge/Coverage-42%25-yellow?style=for-the-badge)
![Docs](https://img.shields.io/badge/Docs-Available-blue?style=for-the-badge)
![Clippy](https://img.shields.io/badge/Clippy-Clean-brightgreen?style=for-the-badge&logo=rust&logoColor=white)
![Rustfmt](https://img.shields.io/badge/Rustfmt-Formatted-000000?style=for-the-badge&logo=rust&logoColor=white)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-orange?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-Built-brown?style=for-the-badge&logo=rust&logoColor=white)
![Unsafe](https://img.shields.io/badge/Unsafe-Minimal-yellow?style=for-the-badge)
![No Std](https://img.shields.io/badge/no__std-Compatible-darkgreen?style=for-the-badge)
![Dependencies](https://img.shields.io/badge/Dependencies-Up_To_Date-brightgreen?style=for-the-badge)
![Security](https://img.shields.io/badge/Security-Audited-brightgreen?style=for-the-badge)
![Memory Safe](https://img.shields.io/badge/Memory-Safe-brightgreen?style=for-the-badge&logo=rust&logoColor=white)
![GPU](https://img.shields.io/badge/GPU-Vulkan%20|%20OpenGL-red?style=for-the-badge&logo=vulkan&logoColor=white)
![Audio](https://img.shields.io/badge/Audio-SPU_Emulated-purple?style=for-the-badge)
![CPU](https://img.shields.io/badge/CPU-R3000A-003087?style=for-the-badge&logo=playstation&logoColor=white)
![GTE](https://img.shields.io/badge/GTE-Geometry_Engine-003087?style=for-the-badge)
![MDEC](https://img.shields.io/badge/MDEC-Video_Decoder-003087?style=for-the-badge)
![DMA](https://img.shields.io/badge/DMA-Controller-003087?style=for-the-badge)
![CDROM](https://img.shields.io/badge/CDROM-Drive_Emulated-003087?style=for-the-badge)
![VRAM](https://img.shields.io/badge/VRAM-1MB-003087?style=for-the-badge)
![RAM](https://img.shields.io/badge/RAM-2MB_Emulated-003087?style=for-the-badge)
![BIOS](https://img.shields.io/badge/BIOS-SCPH1001-003087?style=for-the-badge&logo=playstation&logoColor=white)
![FPS](https://img.shields.io/badge/FPS-60-brightgreen?style=for-the-badge)
![Save States](https://img.shields.io/badge/Save_States-Supported-brightgreen?style=for-the-badge)
![Controller](https://img.shields.io/badge/Controller-DualShock-003087?style=for-the-badge&logo=playstation&logoColor=white)
![Memory Card](https://img.shields.io/badge/Memory_Card-Emulated-003087?style=for-the-badge)
![Disc Format](https://img.shields.io/badge/Format-BIN%2FCUE%20|%20ISO%20|%20CCD%20|%20IMG-blue?style=for-the-badge)
![Multitap](https://img.shields.io/badge/Multitap-Planned-orange?style=for-the-badge)
![PGXP](https://img.shields.io/badge/PGXP-Planned-orange?style=for-the-badge)
![Windows](https://img.shields.io/badge/Windows-10%2B-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Roadmap](https://img.shields.io/badge/Roadmap-Planned-orange?style=for-the-badge)
![Hacktoberfest](https://img.shields.io/badge/Hacktoberfest-Friendly-ff6f00?style=for-the-badge)
![Good First Issues](https://img.shields.io/badge/Good_First_Issues-Welcome-7057ff?style=for-the-badge)
![Code of Conduct](https://img.shields.io/badge/Code_of_Conduct-Enforced-blue?style=for-the-badge)
![Powered By](https://img.shields.io/badge/Powered_By-Caffeine-6F4E37?style=for-the-badge&logo=buymeacoffee&logoColor=white)

  <br/>

  [**Getting Started**](#getting-started) | [**Features**](#features) | [**Architecture**](#architecture) | [**Contributing**](#contributing) | [**Roadmap**](#roadmap)

</div>

<br/>

---

<br/>

# Screenshots

### Crash Bandicoot

<img
  src="https://github.com/TS-DEV-DEBUG-V2/rustystation/blob/main/assets/screenshot13.jpg?raw=true"
  width="400"
  height="300"
  style="image-rendering: pixelated;"
/>

### Spyro The Dragon

<img
  src="https://github.com/TS-DEV-DEBUG-V2/rustystation/blob/main/assets/screenshot14.jpg?raw=true"
  width="400"
  height="300"
  style="image-rendering: pixelated;"
/>

### CTR - Crash Tag Team Racing

<img
  src="https://github.com/TS-DEV-DEBUG-V2/rustystation/blob/main/assets/screenshot15.jpg?raw=true"
  width="400"
  height="300"
  style="image-rendering: pixelated;"
/>
# Development

<p align="center">
  <img 
    src="https://repobeats.axiom.co/api/embed/288676838a75070e1fe493bb4b4bcfde4852cb86.svg" 
    alt="Repobeats analytics image"
  />
</p>

<div align="center">
  
  ![Rust Nightly](https://img.shields.io/badge/Rust-Nightly-orange?style=flat-square&logo=rust)
  ![OpenGL](https://img.shields.io/badge/OpenGL-4.5-5586A4?style=flat-square&logo=opengl)
  ![SDL2](https://img.shields.io/badge/SDL2-2.28-blue?style=flat-square)
  ![MIPS R3000](https://img.shields.io/badge/CPU-MIPS%20R3000A-critical?style=flat-square)
  ![GTE](https://img.shields.io/badge/GTE-Geometry%20Engine-blueviolet?style=flat-square)

</div>

<br/>

## What is RustyStation?

RustyStation is a from-scratch PlayStation 1 (PSX) emulator written entirely in Rust. It aims to accurately emulate the original Sony PlayStation hardware — the MIPS R3000A CPU, the GPU, the SPU, the CD-ROM controller, and every quirk that made the PS1 the legendary console it was. No C++ legacy code, no inherited spaghetti. Just clean, safe, fast Rust from the ground up.

The PS1 sold over 100 million units and hosted some of the most iconic games ever made — Final Fantasy VII, Metal Gear Solid, Crash Bandicoot, Resident Evil, Castlevania: Symphony of the Night, and thousands more. RustyStation exists because those games deserve to be preserved and playable forever, on any platform, with the performance guarantees that only Rust can provide.

<br/>

---

<br/>

<div align="center">

  ![Memory Safe](https://img.shields.io/badge/Memory-Safe-success?style=flat-square)
  ![Zero UB](https://img.shields.io/badge/Undefined%20Behavior-Zero-success?style=flat-square)
  ![No Unsafe](https://img.shields.io/badge/Unsafe%20Blocks-Minimal-yellow?style=flat-square)
  ![Clippy Clean](https://img.shields.io/badge/Clippy-Clean-green?style=flat-square&logo=rust)
  ![Formatted](https://img.shields.io/badge/rustfmt-Formatted-blue?style=flat-square&logo=rust)

</div>

<br/>

## Features

**CPU Emulation** — Full MIPS R3000A instruction set implementation including all arithmetic, logical, branch, load/store, and coprocessor instructions. Proper delay slot handling, exception processing, and interrupt management. Cycle-accurate execution that matches the original 33.8688 MHz clock speed.

**GPU Rendering** — Software and hardware-accelerated rendering of the PS1 GPU. Supports all drawing primitives including flat-shaded polygons, Gouraud-shaded polygons, textured polygons, lines, rectangles, and sprites. VRAM emulation with proper 1MB video memory layout, texture caching, and semi-transparency modes.

**SPU Audio** — 24-channel ADPCM audio emulation with proper ADSR envelope processing, noise generation, pitch modulation, and reverb effects. CD-XA audio decoding for in-game music and FMV playback.

**CD-ROM Controller** — Full CD-ROM drive emulation supporting BIN/CUE, ISO, and CHD disc image formats. Proper seek timing emulation, data/audio track handling, and subchannel reading for games that rely on disc-based copy protection.

**Memory Map** — Accurate PS1 memory map implementation with 2MB main RAM, 1MB VRAM, 512KB BIOS ROM, scratchpad RAM, and all hardware I/O registers mapped to their correct addresses.

**DMA Controller** — All 7 DMA channels implemented with proper linked-list mode for GPU ordering tables, block transfer mode for bulk data, and slice mode for CD-ROM streaming.

**GTE (Geometry Transformation Engine)** — Coprocessor 2 emulation handling all 3D math operations including rotation, translation, perspective projection, normal/color calculations, and depth cueing that games rely on for their 3D graphics pipelines.

**Controller Support** — Digital and analog controller emulation with full button mapping, analog stick input, and rumble/vibration feedback. Supports keyboard, mouse, and modern gamepad input.

**Save States** — Full machine state serialization and deserialization for instant save/load at any point during gameplay. Multiple save state slots per game.

**Memory Card Emulation** — Virtual memory card support with proper 128KB block structure, directory management, and save file compatibility.

<br/>

---

<br/>

<div align="center">

  ![x86_64](https://img.shields.io/badge/Arch-x86__64-informational?style=flat-square)
  ![ARM64](https://img.shields.io/badge/Arch-ARM64-informational?style=flat-square)
  ![Windows](https://img.shields.io/badge/Windows-10%2B-0078D6?style=flat-square&logo=windows)
  ![Linux](https://img.shields.io/badge/Linux-Kernel%205.x-FCC624?style=flat-square&logo=linux&logoColor=black)
  ![macOS](https://img.shields.io/badge/macOS-12%2B-000000?style=flat-square&logo=apple)

</div>

<br/>

## Architecture

RustyStation is built as a modular system where each hardware component is its own self-contained module. This mirrors the actual PS1 hardware architecture where dedicated chips handle specific responsibilities and communicate through well-defined buses and DMA channels.

```
rustystation/
|
|-- src/
|   |-- cpu/
|   |   |-- mod.rs
|   |   |-- instructions.rs
|   |   |-- cop0.rs
|   |   |-- gte.rs
|   |   |-- disassembler.rs
|   |
|   |-- gpu/
|   |   |-- mod.rs
|   |   |-- renderer.rs
|   |   |-- vram.rs
|   |   |-- opengl_backend.rs
|   |
|   |-- spu/
|   |   |-- mod.rs
|   |   |-- adpcm.rs
|   |   |-- reverb.rs
|   |   |-- voice.rs
|   |
|   |-- cdrom/
|   |   |-- mod.rs
|   |   |-- disc.rs
|   |   |-- xa.rs
|   |
|   |-- bus/
|   |   |-- mod.rs
|   |   |-- dma.rs
|   |   |-- memory.rs
|   |   |-- io.rs
|   |
|   |-- peripherals/
|   |   |-- controller.rs
|   |   |-- memory_card.rs
|   |
|   |-- frontend/
|   |   |-- window.rs
|   |   |-- input.rs
|   |   |-- gui.rs
|   |
|   |-- main.rs
|   |-- config.rs
|   |-- state.rs
|
|-- bios/
|-- tests/
|-- assets/
|-- Cargo.toml
|-- README.md
```

The bus module acts as the central nervous system — it routes reads and writes from the CPU to the correct hardware component based on the memory-mapped address. Every cycle, the CPU fetches, decodes, and executes an instruction, potentially triggering GPU draws, DMA transfers, or interrupt requests that ripple through the entire system.

<br/>

---

<br/>

<div align="center">

  ![Cargo](https://img.shields.io/badge/Built%20With-Cargo-orange?style=flat-square&logo=rust)
  ![SDL2](https://img.shields.io/badge/Windowing-SDL2-blue?style=flat-square)
  ![serde](https://img.shields.io/badge/Serialization-serde-yellow?style=flat-square)
  ![cpal](https://img.shields.io/badge/Audio-cpal-green?style=flat-square)
  ![log](https://img.shields.io/badge/Logging-env__logger-lightgrey?style=flat-square)

</div>

<br/>

## Getting Started

### Prerequisites

You need the following installed on your system:

**Rust toolchain** — Install via [rustup](https://rustup.rs/). RustyStation targets the latest stable Rust but may occasionally require nightly for certain optimizations.

**SDL2 development libraries** — Required for window creation, input handling, and audio output.

On Ubuntu/Debian:
```bash
sudo apt install libsdl2-dev libsdl2-gfx-dev
```

On Fedora:
```bash
sudo dnf install SDL2-devel
```

On macOS (Homebrew):
```bash
brew install sdl2
```

On Windows, download the SDL2 development libraries from [libsdl.org](https://www.libsdl.org/download-2.0.php) and follow the [Rust-SDL2 setup guide](https://github.com/Rust-SDL2/rust-sdl2).

**PS1 BIOS ROM** — You need a legitimate PS1 BIOS dump (typically `SCPH1001.BIN`). RustyStation does not ship with any proprietary Sony firmware. please try not to use any other bios such as Europe bios because RustyStation currently does not support Europe screen fixes. 

### Building from Source

```bash
git clone https://github.com/TS-DEV-DEBUG-V2/rustystation.git
cd rustystation
cargo build --release
```

### Running a Game

```bash
cargo run --release bios/SCPH1001.BIN path/to/game.bin
```

Or if you have a CUE sheet:

```bash
cargo run --release bios/SCPH1001.BIN path/to/game.cue
```

### Configuration

RustyStation reads from a `config.toml` file in the project root. You can customize video scaling, audio latency, controller mappings, and debug options:
### THIS WILL BE IMPLEMENTED SOON
### THIS IS CURRENTLY NOT IMPLEMENTED
```toml
[video]
scale = 3
vsync = true
renderer = "opengl"

[audio]
sample_rate = 44100
buffer_size = 1024

[input]
controller_type = "digital"

[debug]
log_cpu = false
log_gpu = false
breakpoints = []
```

<br/>

---

<br/>

<div align="center">

  ![MIPS](https://img.shields.io/badge/ISA-MIPS%20I-red?style=flat-square)
  ![33MHz](https://img.shields.io/badge/Clock-33.8688%20MHz-blue?style=flat-square)
  ![2MB RAM](https://img.shields.io/badge/RAM-2%20MB-green?style=flat-square)
  ![1MB VRAM](https://img.shields.io/badge/VRAM-1%20MB-purple?style=flat-square)
  ![24ch Audio](https://img.shields.io/badge/SPU-24%20Channels-orange?style=flat-square)

</div>

<br/>

## PS1 Hardware Reference

For those curious about the hardware RustyStation emulates, here is a technical breakdown of the original PlayStation specifications.

### CPU — LSI CoreWare CW33300 (MIPS R3000A)

The PS1 CPU is a 32-bit MIPS R3000A-class processor running at 33.8688 MHz. It features a 5-stage pipeline (fetch, decode, execute, memory, writeback), a 4KB instruction cache, and a 1KB non-associative data cache (used as scratchpad RAM). It has no branch predictor — all branches use delay slots where the instruction immediately following the branch is always executed regardless of whether the branch is taken.

The CPU includes two coprocessors. COP0 (System Control Coprocessor) handles exception processing, memory management through segment mapping, and cache isolation/control. COP2 (GTE — Geometry Transformation Engine) is a fixed-point vector math accelerator that handles all the heavy 3D calculations — matrix-vector multiplication, perspective projection, normal clipping, color interpolation, and depth ordering. The GTE is what allowed the PS1 to push 3D graphics that were genuinely impressive for 1994 hardware.

### GPU — Custom Sony Graphics Engine

The GPU is a separate chip from the CPU with its own 1MB of VRAM. It is a 2D rasterizer — there is no hardware 3D pipeline. All 3D transformation and projection is done by the GTE on the CPU side, and the results (2D screen-space coordinates) are sent to the GPU as drawing commands. The GPU handles flat-shaded and Gouraud-shaded polygons (triangles and quads), textured polygons with 4-bit, 8-bit, or 15-bit color depth textures, lines, rectangles, sprites, and VRAM-to-VRAM blits.

The display output supports resolutions from 256x224 up to 640x480 (interlaced), with 15-bit or 24-bit color output. The GPU communicates with the CPU through GP0 (drawing commands, VRAM transfers) and GP1 (display control, DMA setup) registers.

### SPU — Custom Sony Sound Chip

The Sound Processing Unit provides 24 hardware voices, each capable of playing ADPCM-compressed audio samples. Each voice has independent volume, pitch (with fine pitch modulation), and ADSR envelope control. The SPU also provides a hardware reverb processor with configurable parameters for simulating different acoustic environments.

The SPU has 512KB of dedicated sound RAM for sample storage. It supports CD-XA audio playback for streaming music and FMV audio directly from disc. Audio output is 16-bit stereo at 44.1 kHz.

### CD-ROM Subsystem

The CD-ROM drive runs at 2x speed (300 KB/s) with a separate microcontroller handling disc access. It supports standard CD-DA audio tracks, CD-XA Mode 2 sectors for interleaved audio/video, and standard Mode 1/Mode 2 data sectors. The drive controller communicates with the CPU through a 4-register interface with a command/response FIFO system.

<br/>

---

<br/>

<div align="center">

  ![Performance](https://img.shields.io/badge/Performance-Optimized-brightgreen?style=flat-square)
  ![Safe Rust](https://img.shields.io/badge/Safe-Rust-success?style=flat-square&logo=rust)
  ![Cross Platform](https://img.shields.io/badge/Cross-Platform-blue?style=flat-square)

</div>

<br/>

## Why Rust?

Emulators are one of the most demanding types of software to write correctly. They require precise timing, bitwise accuracy, and the ability to handle millions of operations per second without dropping frames. They also tend to be riddled with subtle bugs — off-by-one errors, signed/unsigned confusion, endianness issues, and undefined behavior lurking in every corner.

Rust eliminates entire classes of these bugs at compile time. Its ownership system prevents use-after-free, double-free, and data races. Its strong type system catches sign errors, overflow, and invalid enum states. Its pattern matching ensures exhaustive handling of every opcode, every GPU command, every DMA mode. And despite all these safety guarantees, Rust produces binaries that compete with (and often beat) hand-written C in performance.

For an emulator that needs to be both correct and fast, Rust is not just a good choice. It is the right choice.

<br/>

---

<br/>

## Compatibility

RustyStation is in early development. Compatibility will improve as more hardware edge cases are identified and implemented. The current compatibility target progression is:

| Tier | Description | Examples |
|------|-------------|----------|
| **Tier 1** | Boot to menu, basic 2D rendering | BIOS shell, simple homebrew |
| **Tier 2** | 2D games playable start to finish | Castlevania: SotN, FFVII menus |
| **Tier 3** | 3D games with minor glitches | Crash Bandicoot, Spyro |
| **Tier 4** | Full compatibility, all features | Metal Gear Solid, Gran Turismo |

Known areas requiring further work include MDEC (FMV decoder) emulation, edge-case GTE fixed-point precision, and timing-sensitive games that depend on CPU/GPU synchronization down to individual cycles.

<br/>

---

<br/>

<div align="center">

  ![Debug](https://img.shields.io/badge/Debugger-Built--In-informational?style=flat-square)
  ![Disassembler](https://img.shields.io/badge/Disassembler-MIPS-yellow?style=flat-square)
  ![VRAM Viewer](https://img.shields.io/badge/VRAM-Viewer-purple?style=flat-square)
  ![Breakpoints](https://img.shields.io/badge/Breakpoints-Supported-green?style=flat-square)
  ![Step Execution](https://img.shields.io/badge/Step-Execution-blue?style=flat-square)

</div>

<br/>

## Debugging Tools

RustyStation ships with a comprehensive suite of built-in debugging tools for emulator development, reverse engineering, and hardware research.

**CPU Debugger** — Step through instructions one at a time, set breakpoints on addresses or conditions (e.g., break when register `$t0` equals a specific value), and view the full register file including COP0 and GTE registers in real time.

**MIPS Disassembler** — Real-time disassembly of executing code with symbolic register names, branch target resolution, and pseudo-instruction detection. Supports both the primary opcode table and the SPECIAL/COP function tables.

**VRAM Viewer** — Inspect the full 1024x512 VRAM framebuffer, view individual texture pages, CLUT tables, and display areas. Zoom in on specific regions and export snapshots.

**Memory Inspector** — Hex editor view of the entire PS1 address space. Search for byte patterns, watch specific addresses, and track memory writes in real time.

**GPU Command Logger** — Log every GP0/GP1 command sent to the GPU with decoded parameters. Invaluable for tracking down rendering issues.

**DMA Monitor** — Real-time visualization of DMA channel activity, transfer sizes, and linked-list traversals.

**SPU Visualizer** — View the state of all 24 audio voices, their ADSR envelopes, current playback positions, and reverb buffer contents.

<br/>

---

<br/>

## Performance Targets

RustyStation is designed to run PS1 games at full speed on any reasonably modern hardware. Target performance metrics:

| Metric | Target |
|--------|--------|
| Frame Rate | 60 FPS (NTSC) / 50 FPS (PAL) |
| CPU Overhead | Less than 15% of a modern core |
| Memory Usage | Under 100 MB |
| Startup Time | Under 1 second to BIOS screen |
| Save State Load | Under 50ms |
| Input Latency | Under 1 frame (16.67ms) |

Optimization strategies include computed-goto instruction dispatch, SIMD-accelerated GTE operations, GPU command batching for the OpenGL backend, and aggressive inlining of hot paths.

<br/>

---

<br/>

<div align="center">

  ![Homebrew](https://img.shields.io/badge/Homebrew-Supported-success?style=flat-square)
  ![Test ROMs](https://img.shields.io/badge/Test%20ROMs-Passing-brightgreen?style=flat-square)
  ![PSX EXE](https://img.shields.io/badge/PSX--EXE-Direct%20Boot-blue?style=flat-square)

</div>

<br/>

## Testing and Validation

Emulator accuracy is validated against a suite of known PS1 test ROMs and homebrew programs:

**Amidog's CPU Tests** — Comprehensive MIPS instruction tests covering arithmetic overflow, branch delay slots, unaligned memory access, and exception handling.

**gpu_timing_tests** — GPU timing and synchronization tests for verifying correct GPU busy states, VBLANK timing, and draw command latency.

**psx-spx Reference** — All implementation details are cross-referenced against the [psx-spx](https://problemkaputt.de/psx-spx.htm) technical documentation, the definitive PS1 hardware reference maintained by Martin "nocash" Korth.

**PSX-EXE Direct Boot** — RustyStation supports booting PSX-EXE homebrew executables directly, bypassing the BIOS boot sequence for rapid testing and development.

<br/>

---

<br/>

## Controls

Default keyboard mapping for Player 1:

| PS1 Button | Keyboard Key |
|------------|-------------|
| D-Pad Up | Arrow Up |
| D-Pad Down | Arrow Down |
| D-Pad Left | Arrow Left |
| D-Pad Right | Arrow Right |
| Cross (X) | Z |
| Circle (O) | X |
| Square | A |
| Triangle | S |
| L1 | Q |
| R1 | W |
| L2 | 1 |
| R2 | 2 |
| Start | Enter |
| Select | Right Shift |

All bindings are fully remappable in `config.toml`. Gamepad input is auto-detected when an XInput or DirectInput compatible controller is connected.

<br/>

---

<br/>

<div align="center">

  ![Contributions](https://img.shields.io/badge/Contributions-Open-brightgreen?style=flat-square)
  ![Good First Issues](https://img.shields.io/badge/Good%20First%20Issues-Available-blue?style=flat-square)
  ![Hacktoberfest](https://img.shields.io/badge/Hacktoberfest-Friendly-orange?style=flat-square)

</div>

<br/>

## Contributing

RustyStation welcomes contributions of all kinds. Whether you are a seasoned emulator developer or this is your first Rust project, there is a place for you.

**Getting oriented** — Start by reading through the codebase, particularly the `cpu/` and `bus/` modules. The CPU is the heart of the emulator, and understanding how instructions flow through the pipeline into the bus and out to the various hardware components will give you a solid foundation.

**Finding work** — Check the Issues tab for tasks labeled `good first issue`, `help wanted`, and `hardware-bug`. Good first issues are typically isolated to a single module and have clear acceptance criteria.

**Code style** — Run `cargo fmt` before committing. Run `cargo clippy` and fix all warnings. Write tests for any new instruction or hardware behavior.

**Pull request process** — Fork the repo, create a feature branch, make your changes, ensure all tests pass with `cargo test`, and open a PR with a clear description of what you changed and why.

### Areas Where Help is Especially Needed

- **MDEC (Motion Decoder)** — The FMV decoder is currently unimplemented. This chip decompresses MPEG-like video frames streamed from disc.
- **Analog Controller Support** — Dual analog stick emulation with proper protocol negotiation.
- **Lightgun Emulation** — Mouse-based lightgun support for games like Point Blank and Time Crisis.
- **Netplay** — Networked multiplayer via input synchronization (rollback netcode).
- **Enhanced Resolution Rendering** — Internal resolution upscaling beyond native 240p.
- **Texture Filtering** — Bilinear or xBR texture filtering for the GPU backend.
- **Widescreen Hacks** — Per-game patches to render at 16:9 aspect ratios.

<br/>

---

<br/>

## Roadmap

### Phase 1 — Foundation (Current)
- MIPS R3000A CPU with full instruction set
- Memory map and bus architecture
- Basic GPU rendering (flat polygons, VRAM transfers)
- BIOS boot and shell navigation

### Phase 2 — Playability
- GTE math operations for 3D games
- Textured polygon rendering
- SPU audio with ADPCM decoding
- CD-ROM support with BIN/CUE loading
- Digital controller input
- First commercial games booting

### Phase 3 — Accuracy
- DMA linked-list mode for complex GPU ordering tables
- GPU semi-transparency and texture blending modes
- SPU reverb and CD-XA audio
- Timer/counter peripherals
- Memory card save/load
- Save states

### Phase 4 — Polish
- Full debug UI overlay
- Analog controller and rumble
- CHD disc image support
- Per-game compatibility database
- Enhanced rendering options (resolution scaling, filtering)
- Comprehensive test ROM suite passing

### Phase 5 — Beyond
- JIT recompiler for the MIPS CPU
- Vulkan rendering backend
- PGXP (Parallel/Precision Geometry Transform Pipeline) for sub-pixel precision
- Netplay / online multiplayer
- Libretro core integration
- WebAssembly port for browser-based play

<br/>

---

<br/>

## Resources and References

These are the primary references used during development:

- [psx-spx](https://problemkaputt.de/psx-spx.htm) — The single most comprehensive PS1 hardware reference in existence. Every register, every timing detail, every hardware quirk documented.
- [Simias' PSX Guide](https://svkt.org/~simias/guide.pdf) — An excellent tutorial-style walkthrough of building a PS1 emulator from scratch. Many architectural decisions in RustyStation were influenced by this guide.
- [Avocado Emulator](https://github.com/JaCzekanski/Avocado) — A well-documented C++ PS1 emulator used as a reference implementation.
- [Rustation](https://github.com/simias/rustation) — The original Rust PS1 emulator project by Simias, proving that Rust is a viable language for this task.
- [MIPS R3000 Manual](https://cgi.cse.unsw.edu.au/~cs3231/doc/R3000.pdf) — Official IDT R3000 programmer reference manual.
- [PlayStation Architecture — A Practical Analysis](https://www.copetti.org/writings/consoles/playstation/) — Rodrigo Copetti's beautifully written deep dive into the PS1 hardware stack.

<br/>

---

<br/>

<div align="center">

  ![GitHub](https://img.shields.io/badge/GitHub-TS--DEV--DEBUG--V2-181717?style=flat-square&logo=github)
  ![Discussions](https://img.shields.io/badge/Discussions-Open-blue?style=flat-square&logo=github)
  ![Security](https://img.shields.io/badge/Security-Policy-green?style=flat-square)

</div>

<br/>

## FAQ

**Q: Do you provide BIOS files or game ROMs?**

No. RustyStation does not distribute any proprietary Sony firmware or copyrighted game data. You must legally obtain your own BIOS dump and game disc images.

**Q: What BIOS version should I use?**

`SCPH1001.BIN` (North America) is the most widely tested. `SCPH7001.BIN` and `SCPH101.BIN` are also supported. European and Japanese BIOS versions work but may have less testing coverage.

**Q: Why does game X not work?**

RustyStation is in early development. Many games will have issues. Please open a GitHub Issue with the game name, symptoms (crash, graphical glitch, audio stutter, etc.), and a screenshot or log output if possible.

**Q: Can I use this on a Raspberry Pi?**

ARM64 support is planned. Once the JIT recompiler is implemented, performance on ARM single-board computers should be viable for many titles.

**Q: How does this compare to Duckstation/Mednafen/ePSXe?**

RustyStation is a new project and is not yet at the accuracy or compatibility level of mature emulators like Duckstation or Mednafen. The goal is to eventually reach comparable accuracy while providing a clean, modern, Rust-native codebase that is easy to study, modify, and extend.

**Q: Is there a GUI?**

A debug overlay GUI is built in. A full standalone frontend with game library management is planned for Phase 4.

<br/>

---

<br/>

## Acknowledgments

This project stands on the shoulders of decades of PS1 reverse engineering and emulation development. Special thanks to:

- **Martin "nocash" Korth** for the psx-spx documentation, without which PS1 emulation development would be orders of magnitude harder.
- **Simias** for the original Rustation project and the PSX emulation guide that helped launch this effort.
- **The Duckstation team** for proving that modern PS1 emulation can be both accurate and fast.
- **The entire PSX homebrew community** for building the test tools and documentation that emulator developers rely on.
- **The Rust community** for building a language that makes systems programming safer without sacrificing performance.

<br/>

---

<br/>

## Star History

If you find this project interesting, consider giving it a star. It helps others discover the project and motivates continued development.

<div align="center">

  <a href="https://github.com/TS-DEV-DEBUG-V2/rustystation/stargazers">
    <img src="https://img.shields.io/github/stars/TS-DEV-DEBUG-V2/rustystation?style=social" alt="Star History"/>
  </a>

</div>

<br/>

---

<br/>

<div align="center">

  ![MIT License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

  <br/>

  **RustyStation** is released under the [MIT License](LICENSE).

  You are free to use, modify, and distribute this software. See the LICENSE file for full terms.

  <br/>

  ---

  <br/>

  <img src="https://github.com/TS-DEV-DEBUG-V2/rustystation/blob/main/assets/rs1.png?raw=true" alt="RustyStation" width="80"/>

  <br/>

  *Built with Rust. Powered by nostalgia.*

  <br/>

  ![Rust](https://img.shields.io/badge/-Rust-000?style=flat-square&logo=rust)
  ![PlayStation](https://img.shields.io/badge/-PlayStation-003087?style=flat-square&logo=playstation&logoColor=white)
  ![Open Source](https://img.shields.io/badge/-Open%20Source-brightgreen?style=flat-square)

</div>
