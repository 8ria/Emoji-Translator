# SubSlay 💅🏻

🧠✨ **Text to Emoji Transformation powered by Rust + WebAssembly**  

---

## 🧾 Overview

**SubSlay** is a blazing-fast, web-based emoji transformer built in Rust and compiled to WebAssembly.  
It intelligently converts your text into an emoji-fied expression using semantic similarity and fabulous flair.

The Rust core powers the logic. The frontend is statically hosted via GitHub Pages.

🔗 [Try the Demo](https://andriak.com/subslay/)  
🔬 Source code lives right here in the `main` branch.

---

## 💡 Features

- 🦀 Rust-powered emoji transformation logic
- ⚡ WebAssembly compilation for performance
- 🧬 Uses GloVe word embeddings + Levenshtein fallback
- 🌐 Deployed with GitHub Pages
- 🎨 Sleek animated UI with a gradient-glow glass aesthetic
- 🔥 Instant transformation with debounced input

> ✨ **Example**: `"Hello, World!"` -> `"👋 🌏"`

---

## 🛠️ How It Works

The core Rust logic uses:
- A mapping of keywords → emojis (`emoji.json`)
- GloVe embeddings for vector similarity
- Levenshtein distance for unknown word fallback
- WebAssembly interface via `wasm-bindgen`

The frontend:
- Loads the WASM module (`subslay_bg.wasm`)
- Loads embeddings and emoji map
- Lets users type and instantly see the emoji-fied output

---

## 🚀 Get Started (Locally)

### 1. Build the WASM package

```bash
wasm-pack build --target web
````

This will create a `pkg/` folder with `subslay.js` and `subslay_bg.wasm`.

### 2. Serve the frontend

Make sure `index.html`, `pkg/`, and `static/` are in the root. Then serve using:

```bash
python3 -m http.server
# or use any local dev server of your choice
```

---

## 🌍 Deployment

* The static site is hosted from the [`gh-pages`](https://github.com/8ria/subslay/tree/gh-pages) branch.

---

## 📁 Project Structure

```bash
subslay/
├── src/
│   └── lib.rs               # Rust logic: text → emoji transformer
├── static/
│   ├── emoji.json           # Keyword to emoji map
│   └── glove.txt            # GloVe vectors
├── pkg/                     # Auto-generated WebAssembly files
├── index.html               # UI and JS logic
├── Cargo.toml               # Rust project config
├── LICENSE                  # MIT License
└── README.md                # You’re here 💅🏻
```

---

## 🧑‍💻 Contributing

Pull requests are welcome! Feel free to:

* Add new emoji mappings
* Improve GloVe or fallback logic
* Optimize the Rust or WASM build
* Beautify the frontend more ✨

---

## 📄 License

MIT © AndriaK
Feel free to slay it your way 💅🏻

---

> *"Built to slay, not obey."* 💅
