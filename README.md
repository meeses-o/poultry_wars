# Poultry Wars

<!-- TODO: one sentence pitch, e.g. "A 2D chicken-vs-turkey game in Rust that runs natively and in the browser via WebAssembly." -->

**Play:** https://pwars.oosmond.dev

<!-- TODO: gameplay GIF or screenshot here. Interviewers look at this first. -->

## Highlights

<!-- 3-5 bullets an interviewer can skim in 10 seconds. Lead with the hardest problems. -->
- One Rust codebase builds for desktop and for the browser (`wasm32-unknown-unknown`).
- Custom JS bridge connects macroquad's WASM loader to `wasm-bindgen` (see `web/helpers/patch_wasm.js`).
- Online accounts and player ranks stored in Turso (libSQL). The client talks to a worker proxy, so no DB token ships to the browser.
- Scripted build that produces the web release in one command (`build.sh`).
- Deployed as a static site on Cloudflare (`wrangler.jsonc`).

## Tech Stack

| Area | Tools |
|------|-------|
| Language | Rust (edition 2024) |
| Engine | macroquad 0.4 |
| Web | WebAssembly, wasm-bindgen, web-sys, JavaScript |
| Data | Turso / libSQL, serde, ureq (native) |
| Hosting | Cloudflare Workers static assets |

## Architecture

<!-- TODO: small diagram (game loop, screens, DB path). game/filemap.png may fit here. -->

```
game/src/
  screens/   welcome, login, selection, guide, game, win
  classes/   Chicken, Turkey (player characters)
  maps/      Map1, Map2
  ui/        Label, TextButton, TextInput, StillImage, grid
  utils/     collision, database client, texture preloading, rank calculation
```

- Screen flow: <!-- TODO: welcome -> login -> selection -> game -> win -->
- Database path: native uses `ureq`; web calls `mq_db_query` in `web/helpers/db_bridge.js`, which runs `fetch` against the worker.
- `gwb/` holds the web-build copy of the game, with web-only changes listed in `gwb/changes/changes.txt`.

## Technical Challenges

<!-- The most important section for interviews. 2-4 entries. Use: Problem, Approach, Result. -->

### 1. Running macroquad and wasm-bindgen together
- **Problem:** <!-- TODO: the generated JS imported `env` and its own loader, which clashed with miniquad -->
- **Approach:** <!-- TODO: post-build patch script, `set_wasm` export -->
- **Result:** <!-- TODO -->

### 2. Async database calls from a sync game loop
- **Problem:** <!-- TODO -->
- **Approach:** <!-- TODO: JS fetch writes to a result buffer; Rust polls length, then fills -->
- **Result:** <!-- TODO -->

### 3. Database migration
- **Problem:** <!-- TODO: lost access to the original database -->
- **Approach:** <!-- TODO -->
- **Result:** <!-- TODO -->

### 4. Cutscenes
- <!-- TODO: the image-sequence bug from the cutscene work, and how you fixed it -->

## My Role

<!-- TODO: team project, so say exactly what you built. Name teammates' parts too. Interviewers will ask. -->
- <!-- e.g. Built the web build pipeline and deployment -->
- <!-- e.g. Wrote the database bridge and migrated the DB -->

## Run Locally

Prerequisites: Rust, `wasm32-unknown-unknown` target, `wasm-bindgen-cli` (match version `0.2.106`), Node.js.

```bash
# Desktop
./run.sh

# Web build (output in web_release/)
./build.sh

# Deploy
npx wrangler deploy
```

## What I Would Improve

<!-- Shows judgment. 3 honest bullets. -->
- <!-- e.g. Remove the duplicate gwb/ tree with cfg flags -->
- <!-- e.g. Add tests for collision and rank calculation -->
- <!-- e.g. Replace unwrap() on DB data with error handling -->

## What I Learned

<!-- TODO: 2-3 bullets. Short and specific. -->

## Credits

<!-- TODO: teammates, the database module template author, art and music sources -->
