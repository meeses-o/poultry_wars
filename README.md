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
</p>

---

<!-- TODO: add a gameplay GIF or screenshot here (drag the file into the GitHub editor to get a user-attachments URL) -->

## Please Note
Poultry Wars was built by a small team (C.O.M Studios) as an in-class project.
Accounts are for keeping score only. Do not reuse a real password, as passwords are not hashed (see [Known Limitations](#known-limitations)).

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
<!-- TODO: diagram of the screen flow and the DB request path. game/filemap.png may work here. -->

### main.rs (screen manager):
Preloads every texture once through the `TextureManager`, then runs the login screen for both players. After that it loops over
screens by name: each screen's `run()` is an async function that returns the name of the next screen to show.

```
login ─▶ welcome ─┬─▶ guide
                  └─▶ selection ─▶ game ─▶ win ─▶ (back to welcome)
```

### screens/:
One file per screen: `login`, `welcome` (cutscene and main menu), `guide` (tutorial), `selection` (fighter and map),
`game` (the match), and `win` (results). The win screen updates both players' rank and coins, then saves them with `update_player_dbs`.

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
- Rank and coin values are stored as strings and parsed with `unwrap()`, so bad data panics the game.

## Getting Started
Play it at https://pwars.oosmond.dev, or build it yourself.

### Prerequisites
Rust (stable), the `wasm32-unknown-unknown` target, `wasm-bindgen-cli` **0.2.106** (it must match the crate version), and Node.js (for the patch script).

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.106
```

### Build & run
```bash
git clone https://github.com/meeses-o/poultry_wars.git
cd poultry_wars

# Desktop
./run.sh

# Web (outputs to web_release/)
./build.sh
python -m http.server -d web_release
```

### Deploy
```bash
npx wrangler deploy
```

### Using your own database
Create a Turso table:
```sql
CREATE TABLE poultrywars (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  username TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  rank TEXT NOT NULL,
  money TEXT NOT NULL
);
```
Next, deploy a worker that forwards the client's JSON requests to Turso as SQL, and keep the Turso URL and token as worker secrets.
Then set `WORKER_URL` in `game/src/utils/database.rs` (and in `gwb/game/src/utils/database.rs`) to your worker's URL, and rebuild.
<!-- TODO: add the worker source to the repo (e.g. worker/) and link it here -->

## Project Structure

```
game/          Main Rust crate (desktop build)
  src/screens/   Login, welcome, guide, selection, game, win
  src/classes/   Chicken and Turkey fighters
  src/maps/      Map geometry
  src/ui/        Widgets
  src/utils/     Collision, DB client, texture preloading, ranks
  assets/        Sprites, maps, health bars, rank images
gwb/           Web copy of the crate, plus notes on web-only changes
web/           Web build workspace (HTML, CSS, JS helpers, wasm-bindgen output)
web_release/   Files deployed to Cloudflare
build.sh       Web build pipeline
run.sh         Desktop run
```

## Challenges & What I Learned
<!-- TODO: rewrite these in your own voice. They are drafted from the commit history. -->

### Getting macroquad and wasm-bindgen to cooperate
<!-- What broke at first, how you found the cause, and why a post-build patch was the fix. -->

### Database calls without blocking the game loop
<!-- The polling bridge: why you couldn't simply await a JS promise from macroquad, and what you learned about the WASM/JS boundary. -->

### Migrating the database
<!-- The team lost access to the original database. Explain how you moved to a database you control and put it behind your own worker and domain. -->

### Cutscenes
<!-- The welcome cutscene first stopped at a hard-coded frame index (img_scene == 9), which failed when switching to the angry-chicken images. Explain how you fixed it. -->

## Future plans/add-ons
- [ ] Hash passwords and check logins on the worker, not the client
- [ ] Per-user auth on the worker
- [ ] Merge `gwb/` back into `game/` using `cfg(target_arch = "wasm32")`
- [ ] Store rank and coins as integers
- [ ] Spend coins on cosmetics
- [ ] Online multiplayer

## Credits
<!-- TODO: list each team member and what they built, plus art and music sources -->
- Database module based on a template by Mathew Dusome

## License
<!-- TODO: no LICENSE file yet. Add one (e.g. MIT), then add a badge as in the header. -->
