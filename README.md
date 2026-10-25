# Spins

A tavern table game of five wheels, after the Wheels minigame in Sea of Stars. You and a computer opponent each spin five carved wheels up to three times a turn, locking the ones you want to keep. At the end of the turn the symbols showing fill your two Legends' energy bars and build your Barricade, and every full Legend acts. A Knight strikes the Barricade first, a Wizard's second bolt flies over it, and a Bowman's arrow passes a Barricade lower than three. First to bring the other's health from ten to none wins.

Play it at https://sgilson7.github.io/spins/ in one of three views:

- **2D** (`/spins/2d/`), in the flat look of Lowballer, drawn as page elements with no canvas;
- **3D** (`/spins/3d/`), carved figures on a candlelit table, in the look of Protection of the Spheres, in WebGL 2;
- **Pixel** (`/spins/pixel/`), sprites on a sixteen-pixel grid and wheels that click round in steps.

All three load the same engine file. Only the drawing differs.

## How it is built

Spins is a [Builder](https://github.com/sgilson7/builder-setting) project. Every rule is in the Rust core (`crates/core`): the wheels, the 3oak+ count, each proficiency's attack, the turn, the computer opponent and the description of each screen. The shim (`crates/shim`) moves values into WebAssembly and decides nothing. The pages in `web/` draw what the core describes. The 3D view's instances are built by a second Rust crate (`crates/view3d`), held to the same rules as the core. The opponent's wheels stay inside the engine until the reveal, so a covered wheel cannot be read from the page.

```bash
cargo test --workspace          # the rules, the opponent, the scene, the boundary scans
./scripts/build.sh              # release WebAssembly build into dist/
python3 scripts/serve.py        # http://127.0.0.1:8000/
./scripts/check.sh              # everything, three browsers, before publishing
python3 scripts/break_tests.py  # each row of tests/breaks.json must make its test fail
python3 tools/pixel/sprites.py  # rewrite the pixel view's sprites from their grids
```

The design journal, with the mockups and the decisions behind each rule, is kept beside this repository and not published here.
