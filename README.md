# dxa-wasm

Future home of the DEXA compiler compiled to WebAssembly for the [dxa.dev](https://dxa.dev) playground.

Currently built from the compiler in [dxa-dev](https://github.com/dxiv/dxa-dev). This repo is prepared for a possible split so the WASM package can be published and versioned separately (e.g. for the playground or other run-in-browser tooling).

- **Build:** `wasm-pack build` (depends on the compiler crate)
- **Output:** `pkg/` (JS + WASM) consumed by the website
