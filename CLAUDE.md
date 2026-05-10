# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run

```bash
cargo build              # Debug build
cargo run                # Run the game
cargo run -- -c CODE     # Decode secret codes (CLI utility mode)
cargo build --release    # Release build
cargo bundle --release   # Create macOS .app bundle (requires cargo-bundle)
./create_dmg.sh          # Package .app into .dmg (after bundle)
```

No test suite or linter is configured.

## Architecture

This is "MerinoBreakout", a breakout/Arkanoid game built with **Bevy 0.18** (Rust edition 2024). It uses Bevy's ECS with a state machine driving scene transitions.

### Game States (defined in `main.rs`)

`Splash → Menu → Shop | Game → Transition → (next level or Menu)`

Each state has a plugin that registers `OnEnter`/`OnExit`/`Update` systems and uses a tag component (e.g., `GameTag`, `MenuTag`) for bulk despawning on exit.

### Module Responsibilities

- **`game.rs`** — The `Game` resource (central state: grid, lives, assets, audio, player progress). Level initialization. Registers all gameplay systems.
- **`consts.rs`** — All game constants: dimensions, physics, level data (as byte-string grids), secrets, barrel weights, layer z-ordering.
- **`collisions.rs`** — AABB/circle intersection, grid coordinate conversion (`game_xy_to_rc`, `game_rc_to_xy`), brick collision + destruction logic.
- **`paddle.rs`** — Paddle movement (arrow keys + shift for speed), ball catch/release (space), gun firing.
- **`balls_and_bullets.rs`** — Ball physics (wall/paddle/brick bouncing, speed scaling), bullet movement.
- **`barrels.rs`** — Power-up barrels: spawning (weighted random), falling, paddle collision, effect application (extend, gun, shrink, magnet, multiball, speed changes, portal, extra life, extra time).
- **`meanies.rs`** — Enemy spawning from top portals, movement AI, ball-killing collisions.
- **`shop.rs`** — Code entry UI, Vigenere cipher encode/decode, secret unlocking logic. Codes are `USERNAME_LEN + 3` characters.
- **`save.rs`** — Flat-file persistence (`~/.merino_breakout.txt`): saves/loads secret codes which reconstitute all progress.
- **`animation.rs`** — Generic frame-based sprite animation component (repeat, despawn, freeze modes).
- **`countdown_and_portal.rs`** — Level timer, portal open/close logic.
- **`menu.rs`** — Main menu (play/shop/reset/credits), music selection.
- **`splash.rs`** / **`transition.rs`** — Intro splash and inter-level transition screens.

### Key Design Patterns

- Levels are defined as `[&[u8; 15]; 22]` byte grids in `consts.rs`. Hex chars `0-e` map to brick variants.
- The `Game` resource holds all asset handles (images, audio) loaded at startup — systems reference them via `Res<Game>` / `ResMut<Game>`.
- Progress is stored as cipher-encoded codes. The save file is just a list of valid codes; loading replays them through `shop_code_add`.
- Barrel power-up availability is gated by unlocked secrets (e.g., Gun barrel only appears after the Gun secret is unlocked).
- Frame-rate independence uses `game.avg_delta` rather than Bevy's `Time` resource directly.
