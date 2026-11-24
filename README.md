<div align="center">
<pre>
██████╗  ██████╗ ██╗   ██╗██╗     ████████╗██████╗ ██╗   ██╗
██╔══██╗██╔═══██╗██║   ██║██║     ╚══██╔══╝██╔══██╗╚██╗ ██╔╝
██████╔╝██║   ██║██║   ██║██║        ██║   ██████╔╝ ╚████╔╝&nbsp;
██╔═══╝ ██║   ██║██║   ██║██║        ██║   ██╔══██╗  ╚██╔╝&nbsp;&nbsp;
██║     ╚██████╔╝╚██████╔╝███████╗   ██║   ██║  ██║   ██║&nbsp;&nbsp;&nbsp;
╚═╝      ╚═════╝  ╚═════╝ ╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;██╗    ██╗ █████╗ ██████╗ ███████╗&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;██║    ██║██╔══██╗██╔══██╗██╔════╝&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;██║ █╗ ██║███████║██████╔╝███████╗&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;██║███╗██║██╔══██║██╔══██╗╚════██║&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;╚███╔███╔╝██║  ██║██║  ██║███████║&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
</pre>
</div>

<p align="center">A local two-player platform fighter written in Rust using Macroquad. Play in browser through WebAssembly; online accounts keep track of each player's rank.</p>

<p align="center">
  <a href="https://pwars.oosmond.dev"><img src="https://img.shields.io/badge/demo-live-brightgreen" alt="Live Demo"></a>
  <img src="https://img.shields.io/badge/Rust-WASM-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/engine-macroquad-yellow" alt="macroquad">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="License: MIT"></a>
</p>

---

<!-- TODO: add a gameplay GIF or screenshot here (drag the file into the GitHub editor to get a user-attachments URL) -->

## Please Note
Poultry Wars was built by a small team (C.O.M Studios) as an in-class project.
Accounts are for keeping score only. Do not reuse a real password, as passwords are not hashed (see [Future plans/add-ons](#known-limitations)).

## Overview

Poultry Wars is a fighting game in the style of platform brawlers, where two players share one keyboard. Each player logs in, picks a
fighter (Chicken or Turkey) and a map, and then fights until one player runs out of stocks. The winner gains rank,
which is saved to a Turso database through an Cloudflare worker API, so progress carries over between sessions and devices.
The game is written in Rust with the macroquad engine. 

## Features

- Local two-player combat on one keyboard
- Two fighters, each with its own move:
  - **Chicken:** ground slam
  - **Turkey:** dash
- 10 HP per life and 3 stocks per player, with animated health bars and stock counters
- Two maps with platform collision
- Account creation and login; each win gives +1 rank.
- Five rank tiers, each with its own trophy image
- Animated welcome cutscene, tutorial screen, and music
- Runs in browser, /game exists for local play but is discontinued. Play in-browser for best experience.

## Controls

| Action | Player 1 | Player 2 |
|--------|----------|----------|
| Move | `A` / `D` | `←` / `→` |
| Jump | `W` | `↑` |
| Attack | `Space` | `M` |
| Special (slam / dash) | `S` | `↓` |

## Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2024) |
| Engine | macroquad 0.4 (rendering, input, audio, game loop) |
| Web target | WebAssembly (`wasm32-unknown-unknown`), `wasm-bindgen`, `web-sys` |
| JS glue | Custom post-build patch (`patch_wasm.js`) and DB bridge (`db_bridge.js`) |
| Native networking | `ureq` |
| Serialization | `serde`, `serde_json` |
| Database | Turso (libSQL), reached through a worker API |
| Hosting | Cloudflare (static assets, deployed with `wrangler`) |

## Architecture
<img width="1880" height="2000" alt="pwars_diagram" src="https://github.com/user-attachments/assets/37716384-61d8-4bb3-8563-6b9c54ab30da" />


### main.rs (screen manager):
Preloads every texture once through the `TextureManager`, then runs the login screen for both players. After that it loops over
screens by name: each screen's `run()` is an async function that returns the name of the next screen to show, excluding the main
"game" block, where welcome passes "game" and then internal game screens pass back data.

```
login ─▶ welcome ──▶ guide
            └─▶ selection ─▶ game ─▶ win ─▶ (back to welcome)
```

### screens/:
One file per screen: `login`, `welcome` and `guide` (tutorial and landing), `selection` (of player type and map),
`game` (the actual game screen), and `win` (results). The win screen updates both players' rank and displays the rank of the winner, 
then saves them with `update_player_dbs`.

### classes/ (Chicken, Turkey):
Each fighter owns its own physics (acceleration, deceleration, top speed, gravity), input handling, attack state, health, and stocks.
Player 1 and Player 2 come from the same struct and differ only in their key bindings and sprite set.

### maps/ and utils/collision.rs:
Each map provides its floor and platform geometry. Fighters check their position against it every frame to land and to reset their jump.
`collision.rs` handles hit detection between fighters.

### ui/:
Small reusable widgets drawn with macroquad: `Label`, `TextButton`, `TextInput` (with clipboard support), `StillImage`, and a debug grid.

### utils/database.rs:
An async `DatabaseClient` that sends JSON requests (`fetch_table`, `insert_record`, `update_record_by_id`, and others) to the worker
at `WORKER_URL`. The client never talks to Turso directly, so no database token ships with the game.

## Web Build Pipeline
macroquad and `wasm-bindgen` both expect to load the WASM module themselves, so the output of a plain build does not run. `build.sh` fixes this:

1. Builds `gwb/game` (the web copy of the game) for `wasm32-unknown-unknown` in release mode.
2. Runs `wasm-bindgen --target web` on the output.
3. Runs `helpers/patch_wasm.js` on the generated JS. The script removes the bare `env` import, removes `wasm-bindgen`'s own loader blocks, and exports
   `set_wasm` so that macroquad's loader can hand over the WASM instance.
4. Copies `db_bridge.js` and the package into `web_release/` for deployment.

### Async database calls from WASM
On native builds, the client calls `ureq` directly. In the browser, Rust calls an `extern "C"` function, `mq_db_query`, which
`db_bridge.js` provides. The JS side runs `fetch` and writes the response into a buffer. Meanwhile the Rust side calls `next_frame().await`
in a loop and polls `mq_db_query_result_len()` until the response arrives, then copies it out with `mq_db_query_fill_result()`.
Because of this, the game keeps rendering while it waits for the network, with no extra async runtime.

## Known Limitations
- Passwords are stored in plain text.
- Login downloads the whole `poultrywars` table and checks credentials on the client, so any client can read every account.
- The worker has no per-user authentication.
- The web build lives in a separate copy of the source (`gwb/`), and changes are carried over by hand (`gwb/changes/changes.txt`).
- Rank values are stored as strings and parsed with `unwrap()`, so bad data panics the game.

## Getting Started
Play it live at https://pwars.oosmond.dev.

## Project Structure

```
game/          Desktop Rust crate (discontinued)
gwb/           Main Rust Build (Rust core that gets compiled to WASM)
  src/screens/   Login, welcome, guide, selection, game, win
  src/classes/   Chicken and Turkey fighters
  src/maps/      Map geometry
  src/ui/        Widgets
  src/utils/     Collision, DB client, texture preloading, ranks
  assets/        Sprites, maps, health bars, rank images
web/           Web build workspace (HTML, CSS, JS helpers, wasm-bindgen output)
web_release/   Files deployed to Cloudflare
build.sh       Web build pipeline
run.sh         Desktop run
```

## Challenges & What We Learned

### Getting macroquad and wasm-bindgen to cooperate
When initially attempting to get the game working on the web,
having macroquad attempt to work and draw nicely in WASM in a contained
HTML environment was a real challenge. The game did not scale nicely inside its
container, and many issues were being had with database.rs not being able to talk
to our Turso. These issues were solved by writing custom JS glue, and implimenting a
2d camera system inside the Rust core to deal with dynamic sizing. This taught us a lot
about integrating Rust WASM with web environments, and how specifically the Web Assembly
communicates with other JS through its glue.

### Database calls without blocking the game loop
One issue that was had initially was that database calls would freeze the entire game, and cause
the screen to flash black while running on the web. This was solved in win.rs by asyncronously 
calling the database such that the rendering loop was never forced to pause to wait. 

### Migrating the database
After the class ended, we lost direct access to the old database, so upon recent revision of this project,
Oliver used the old URL and key stored in the ENV of the old worker with a Rust script to migrate all 
previous data to a new Turso databse, from which the current live build of the game now runs through the same worker.

## Future plans/add-ons
- [ ] Hash passwords and check logins on the worker, not the client
- [ ] Per-user auth on the worker
- [ ] Update `/game` to match `/gwb`, potentially automate changes with a script
- [ ] Implement store feature (use for coins in database)
- [ ] Leaderboards
- [ ] Online multiplayer

## Credits
**Team**
- **Oliver Osmond** ([@o0smond](https://github.com/o0smond)): project lead, project structure, database, web deployment, player classes, screen manager, music
- **Mujibullah Muhebullah** ([@mjv-7](https://github.com/mjv-7)): cutscenes, cutscene art, in-game rank managment, map design
- **Cameron Sidlauskas** ([@ButterBae247](https://github.com/ButterBae247)): Majority of art, databasing, ui design

**Thanks**
- Some modules based on templates by Mathew Dusome

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
