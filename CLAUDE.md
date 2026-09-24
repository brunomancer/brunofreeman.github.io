# CLAUDE.md

Personal site: Rust + [Leptos](https://leptos.dev/) (CSR only — GitHub Pages can't run a server) bundled with [Trunk](https://trunkrs.dev/), deployed via GitHub Actions (`.github/workflows/deploy.yml`) to a custom domain (`CNAME`).

## Layout

Tiny, single-page app — the whole thing is a few files:

| File | Role |
| --- | --- |
| `src/main.rs` | Entry point: panic hook + `mount_to_body(App)`. Rarely needs touching. |
| `src/app.rs` | The entire UI: one `App` component. Currently a cyberpunk "Yes, And" improv scene generator (`OPENERS` / `YES_ANDS` string pools, `rand_pick` via `js_sys::Math::random`, a scene-energy meter capped at `MAX_BEATS`). |
| `style/main.css` | All styling, hand-written plain CSS. Theme colors are custom properties on `:root` (`--cyan`, `--magenta`, etc.); one mobile breakpoint at 480px. |
| `index.html` | Trunk entry: `<title>`, meta description, Google Fonts (Orbitron, Share Tech Mono), and `<link data-trunk rel="css">` pulling in `style/main.css`. `<body>` is empty; Leptos mounts into it. |
| `Trunk.toml` | Build → `dist/`, dev server port 8080, and a `post_build` hook that runs `redirects/redirects.sh`. |
| `redirects/redirects.txt` | Short links: one `<slug> <url>` per line. `www.dev.quest/<slug>` redirects to `<url>`. |
| `redirects/redirects.sh` | POSIX `sh` script (runs under dash in CI). Validates `redirects/redirects.txt` and writes a static `<slug>/index.html` (`location.replace` plus a `<meta refresh>` fallback) into Trunk's staging dir. Bad slugs, non-http(s) URLs, unsafe URL characters and duplicate or colliding slugs fail the build. |
| `rust-toolchain.toml` | Stable + `wasm32-unknown-unknown` target. |
| `CNAME` | Custom domain (`www.dev.quest`). The deploy workflow copies it into `dist/`. Don't delete it. |

Conventions: Leptos 0.8 (`leptos::prelude::*`, `signal()`, `view!`, `<For>`), Rust edition 2024. No router, no server functions, no JS toolchain (npm, bundlers). Everything runs in the browser, so reach browser APIs through `js-sys`/`web-sys`. If you use a new `web-sys` API, add its feature to `Cargo.toml` too.

Deploy: push to `main` → `trunk build --release` on ubuntu → GitHub Pages. No tests or lint in CI; `cargo check` locally is the quick sanity check.

## Commands

```sh
trunk serve             # dev server w/ hot reload, http://localhost:8080
trunk build --release   # production build -> dist/
cargo check              # fast type-check without building wasm
```

## Local toolchain (macOS)

Rust comes from Homebrew's keg-only `rustup`, the only Rust install on this machine (the Homebrew `rust` formula was removed because it conflicted with rustup). Its `cargo`/`rustc` wrappers live in `/opt/homebrew/opt/rustup/bin`, and `trunk` lives in `~/.cargo/bin`. `~/.zshrc` puts both on PATH:

```sh
export PATH="/opt/homebrew/opt/rustup/bin:$HOME/.cargo/bin:$PATH"
```

If `cargo` or `trunk` isn't found, the shell probably didn't load `~/.zshrc`. Run that export yourself.

No `DYLD_LIBRARY_PATH` workaround is needed. The old `libLLVM.dylib` errors came from mixing the two installs. Update Rust with `rustup update`.

`cargo` prints a future-incompatibility warning about `proc-macro-error2`, which comes in through Leptos. Ignore it.

## House rules

- Never `git commit` or `git push` without explicit go-ahead, even after scaffolding/build work is verified.
