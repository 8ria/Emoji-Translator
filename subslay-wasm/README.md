# 🧠 subslay-wasm

**SubSlay**: *Text → Emoji 💅🏻* powered by **Rust + WebAssembly**

This crate provides a WebAssembly (WASM) wrapper around the [`subslay`](https://github.com/8ria/subslay) core emoji-matching logic, enabling fast, offline, client-side translation of text into expressive emoji sequences—right from the browser or any JS runtime.

---

## ✨ Features

* 💬 **Text-to-Emoji**: Convert natural language into matching emoji strings
* ⚡ **WASM-Powered**: Ultra-fast performance, even on mobile
* 📦 **Lightweight**: Optimized release build with LTO
* 🔒 **Offline**: Works without internet connection once loaded
* 🌐 **JS + TS Ready**: Easily callable from JavaScript or TypeScript

---

## 🚀 Quickstart (JS/TS)

```sh
# Build the WASM package
wasm-pack build --release --target web
```

Then in your frontend project:

```js
import init, { WasmStylist } from './pkg/subslay.js';

(async () => {
  await init();

  const stylist = new WasmStylist();
  const result = stylist.slay("i love pizza and cats");
  console.log(result); // ["🍕", "🐱", ...]
})();
```

---

## 🛠️ Development

### Prerequisites

* 🦀 [Rust](https://rust-lang.org)
* 📦 [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Build

```bash
wasm-pack build --release --target web
```

### Folder Structure

```
subslay/
  ├── subslay/          # Core Rust crate (shared by both CLI and WASM)
  └── subslay-wasm/     # WASM wrapper using wasm-bindgen
```

---

## 📄 API

### `WasmStylist::new() → Result<WasmStylist, JsValue>`

Creates a new instance of the emoji stylist.

### `slay(input: &str) → JsValue`

Takes a string of text and returns a `JsValue` (usually a JSON array of emojis).

---

## 📦 Publishing to NPM (optional)

```bash
wasm-pack build --release --target bundler
cd pkg
npm publish
```

---

## 📜 License

MIT © [AndriaK](mailto:hey@andriaK.com)

---

## 🌍 Links

* 🧠 Core Engine: [`subslay`](https://github.com/8ria/subslay)
* 🧪 Demo (coming soon)
