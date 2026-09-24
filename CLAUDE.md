# CLAUDE.md

Personal site: Rust + [Leptos](https://leptos.dev/) (CSR only — GitHub Pages can't run a server) bundled with [Trunk](https://trunkrs.dev/), deployed via GitHub Actions (`.github/workflows/deploy.yml`) to the custom domain `www.dev.quest`. The domain is configured in the repo's Pages settings, not in a `CNAME` file; GitHub ignores `CNAME` files in workflow deployments.

## Layout

Tiny, single-page app — the whole thing is a few files:

| File | Role |
| --- | --- |
| `src/main.rs` | Entry point: panic hook + `mount_to_body(App)`. Rarely needs touching. |
| `src/app.rs` | The entire UI: one `App` component. Currently a cyberpunk "Yes, And" improv scene generator (`OPENERS` / `YES_ANDS` string pools, `rand_pick` via `js_sys::Math::random`, a scene-energy meter capped at `MAX_BEATS`). |
| `style/main.css` | All styling, hand-written plain CSS. Theme colors are custom properties on `:root` (`--cyan`, `--magenta`, etc.); one mobile breakpoint at 480px. |
| `index.html` | Trunk entry: `<title>`, meta description, Google Fonts (Orbitron, Share Tech Mono), and `<link data-trunk rel="css">` pulling in `style/main.css`. `<body>` is empty; Leptos mounts into it. |
| `Trunk.toml` | Build → `dist/`, dev server port 8080, and a `post_build` hook that runs `cargo run --package redirects`. |
| `redirects/redirects.sexp` | Short links: one `(slug "url")` form each, with `;` comments. `www.dev.quest/<slug>` redirects to `<url>`. |
| `redirects/` (crate) | A program that runs on the build machine (not in the browser), as a member of the Cargo workspace rooted at `Cargo.toml`. `src/main.rs` parses the `.sexp` file with the [`lexpr`](https://docs.rs/lexpr) crate (via `datum_iter`, so each form keeps its line number), validates it, then renders a static `<slug>/index.html` with Leptos's `view!` macro (the `ssr` feature, `.to_html()`; Leptos escapes attribute values) containing `location.replace` plus a `<meta refresh>` fallback, and writes it into Trunk's staging dir (`TRUNK_STAGING_DIR`). Bad slugs, non-http(s) URLs, unsafe URL characters and duplicate or colliding slugs fail the build, with a `file:line` error. |
| `rust-toolchain.toml` | Stable + `wasm32-unknown-unknown` target. |

Conventions: Leptos 0.8 (`leptos::prelude::*`, `signal()`, `view!`, `<For>`), Rust edition 2024. No router, no server functions, no JS toolchain (npm, bundlers). Everything runs in the browser, so reach browser APIs through `js-sys`/`web-sys`. If you use a new `web-sys` API, add its feature to `Cargo.toml` too.

Deploy: push to `main` → `trunk build --release` on ubuntu → GitHub Pages. No tests or lint in CI; run the checks below locally.

## Commands

```sh
trunk serve                                  # dev server w/ hot reload, http://localhost:8080
trunk build --release                        # production build -> dist/
cargo check --workspace                      # fast type-check of the site and redirects crate
cargo test -p redirects                      # redirect parser/validation tests
cargo clippy --workspace --all-targets       # lint
cargo fmt --all                              # format
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
