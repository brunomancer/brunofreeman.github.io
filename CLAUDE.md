# CLAUDE.md

Personal site: Rust + [Leptos](https://leptos.dev/) (CSR only — GitHub Pages can't run a server) bundled with [Trunk](https://trunkrs.dev/), deployed via GitHub Actions (`.github/workflows/deploy.yml`) to a custom domain (`CNAME`).

## Commands

```sh
trunk serve             # dev server w/ hot reload, http://localhost:8080
trunk build --release   # production build -> dist/
cargo check              # fast type-check without building wasm
```

## Local toolchain gotcha (macOS, Homebrew rustup)

If `trunk build`/`cargo build --target wasm32-unknown-unknown` fails with:

```
dyld[...]: Library not loaded: @rpath/libLLVM.dylib
```

`rust-lld` can't find its LLVM dylib. Fix by exporting, before any cargo/trunk invocation:

```sh
export DYLD_LIBRARY_PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/lib"
```

This only affects local Homebrew-installed `rustup` (keg-only, no shims in `~/.cargo/bin`) — GitHub Actions runners (Linux) are unaffected. Also note: `rustup` here isn't on PATH by default; invoke it via `$(brew --prefix rustup)/bin/rustup`, or use `~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/{cargo,rustc}` directly.

## House rules

- Never `git commit` or `git push` without explicit go-ahead, even after scaffolding/build work is verified.
