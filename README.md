# 🐍 Terminal Snake Game in Rust

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)

A modular, robust, and responsive terminal-based Snake game built with **Rust** and **crossterm**. Engineered with deterministic input handling, thread-safe terminal RAII cleanup, dynamic timing adjustments, and full boundary collision safety.

---

## ✨ Features

- **Input Validation & Reversal Protection**: Reversal guard logic prevents self-collisions caused by rapid key strokes within a single frame tick.
- **Robust Terminal Safety (RAII)**: Uses safe, atomic single-cleanup guards to ensure the terminal state, cursor, and raw mode are correctly restored—even on panic or force-kill.
- **Dynamic Speed Progression**: Game loop scales tick speeds based on score thresholds while maintaining smooth key responsiveness via polling timeouts.
- **Safe Food Spawning & Win Condition**: Random food generation checks remaining open space to prevent infinite loops, triggering a clear Victory state when the board is completely filled.
- **Clean Architecture**: Strictly decoupled domain logic, input handling, model state, and frame rendering.

---

## 🕹️ Controls

| Key | Action |
| :--- | :--- |
| `W` / `A` / `S` / `D` or `↑` / `←` / `↓` / `→` | Change Snake Direction |
| `P` | Pause / Resume Game |
| `R` | Restart Game *(when Game Over or Won)* |
| `ESC` | Exit Application |
---

## 🛠️ Installation & Building

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (2024 edition or newer)

### Quick Start

1. **Clone the repository:**
  ```
    git clone https://github.com/joebertcerezo/snake_game

    cd snake_game
  ```

2. **Run the game:**
  ```
    cargo run
  ```


---

## 📂 Project Structure

```text
snake_game/
├── Cargo.lock
├── Cargo.toml
├── README.md
├── rustfmt.toml
└── src/
    ├── app.rs          # Application loop & time management
    ├── config.rs       # Configuration options & validation
    ├── game/
    │   ├── mod.rs
    │   ├── state.rs    # GameState definitions (Running, Paused, GameOver, Won)
    │   └── world.rs    # Central game logic, boundary physics, & state updates
    ├── input.rs        # Non-blocking input handling & key mapping
    ├── lib.rs          # Core library exports
    ├── main.rs         # Binary entry point & CLI setup
    ├── models/
    │   ├── direction.rs # Direction vectors & opposite checks
    │   ├── food.rs      # Food spawning logic
    │   ├── mod.rs
    │   ├── position.rs  # 2D coordinate calculations
    │   └── snake.rs     # Snake body state & advance mechanics
    ├── renderer/
    │   ├── board.rs     # Canvas border & entity rendering
    │   ├── hud.rs       # Scoreboard & dynamic HUD overlay
    │   └── mod.rs       # Frame queuing & flushing
    └── terminal.rs     # RAII guard for terminal raw mode & alternate buffer
```


---

## 🧪 Testing & Quality Assurance

Run the test suite to verify physics, input guards, and bounds safety:

```bash
cargo test
```

Check code formatting and run Clippy lints:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

