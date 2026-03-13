# dxa-wasm

> **GitHub About:** DEXA compiler as WebAssembly for the browser. Developed in private monorepo until 1.0.0.

DEXA compiler compiled to **WebAssembly** for in-browser use: [dxa.dev](https://dxa.dev) playground, AST/IR viewer, and any future browser-based tooling.

**Status:** This repo is a **placeholder**. The WASM crate is currently built inside [dxa-dev](https://github.com/dxiv/dxa-dev). When the monorepo is split, this repo will hold the WASM bindings and build so it can be versioned and consumed independently (e.g. by the website or other frontends).

---

## What it does

- **Depends on:** The DEXA compiler ([dxa-compiler](https://github.com/dxiv/dxa-compiler)) — same pipeline (lex → parse → typecheck → lower → validate → interpret).
- **Build:** Produces a WebAssembly module and a small JS glue layer via `wasm-pack`.
- **Export:** A single function `runDexa(source: string)` that compiles and runs DEXA in the browser and returns a JSON-serializable result.

No server required: parsing, typechecking, and execution run entirely in the browser.

---

## API (JavaScript)

```js
import init, { runDexa } from '/wasm/dexa_compiler_wasm.js';
await init(); // load WASM
const result = runDexa('fn main() -> int { return 42; }');
```

**`result` shape:**

| Field | When | Description |
|-------|------|-------------|
| `ok` | Always | `true` if the program ran successfully, `false` on compile or runtime error. |
| `prints` | On success | Array of strings from `print(...)` calls. |
| `result` | On success | String form of the program return value. |
| `gasSteps` | On success / require fail | Gas (steps) consumed. |
| `error` | On failure | Error message (lex/parse/typecheck/IR or `require failed: ...`). |
| `ast` | When available | Pretty-printed AST (debug; used by playground AST tab). |
| `ir` | When available | Pretty-printed IR (debug; used by playground IR tab). |

On `require` failure, `ok` is `false`, `error` is set, and `ast`/`ir` are still populated when the build includes debug output.

---

## Build (after split)

Requires [Rust](https://rustup.rs) and [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (`cargo install wasm-pack`).

This crate will depend on `dexa-compiler` (from [dxa-compiler](https://github.com/dxiv/dxa-compiler)) via path or published crate. Then:

```bash
wasm-pack build --target web --out-dir pkg
```

For the dxa.dev website (when it consumes this repo), the output is typically copied to the site’s static assets (e.g. `website/public/wasm/`), and the site loads the generated JS and WASM from there.

---

## Version and dependencies

- **Version:** 0.1.0 (aligned with dxa-dev).
- **Rust:** Edition 2021, `crate-type = ["cdylib"]`.
- **Dependencies:** `dexa-compiler`, `wasm-bindgen`, `serde`, `serde-wasm-bindgen`.

---

## Where it’s used

- **[dxa.dev/playground](https://dxa.dev/playground)** — Run DEXA in the browser; Output / AST / IR tabs use `runDexa` and the `ast`/`ir` fields.

When split, the website repo will depend on this package (or a published npm artifact) for the playground.

---

## License

Apache 2.0 (to match the WASM crate in dxa-dev).

---

## Links

- **Language & site:** [dxa.dev](https://dxa.dev)
- **Monorepo (current development):** [dxa-dev](https://github.com/dxiv/dxa-dev)
- **Compiler (Rust):** [dxa-compiler](https://github.com/dxiv/dxa-compiler)
