# brunofreeman.github.io

Personal site, built with [Rust](https://www.rust-lang.org/) + [Leptos](https://leptos.dev/) (client-side rendering) and bundled with [Trunk](https://trunkrs.dev/). Deployed to GitHub Pages via GitHub Actions — see [.github/workflows/deploy.yml](.github/workflows/deploy.yml).

## Local development

Requires the `wasm32-unknown-unknown` target and `trunk`:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Then, from the repo root:

```sh
trunk serve        # dev server with hot reload at http://localhost:8080
trunk build --release   # production build, output in dist/
```

## Deployment

Pushes to `main` trigger the `Deploy to GitHub Pages` workflow, which builds the site with Trunk and publishes `dist/` via GitHub Pages. The repo's Pages source must be set to "GitHub Actions" (Settings → Pages).
