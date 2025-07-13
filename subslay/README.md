## 🧾 What is SubSlay?

**SubSlay** is a text-to-emoji transformer that turns plain text into expressive emoji sequences — powered by semantic similarity (via GloVe vectors), direct keyword mappings, and fuzzy Levenshtein fallback.

> ✨ `"Hello, world!"` → `"👋 🌍"`

This crate is built entirely in Rust and is compatible with **native binaries** and **WebAssembly**, with no external files or setup required.

---

## 🌐 Try It Live

🔗 [SubSlay.app](https://subslay.app) — Web demo powered by this crate

---

## 🛠 Features

- 🦀 **Pure Rust** (no WASM bindings inside)
- 📦 **No setup** — GloVe + emoji mappings are embedded
- 💡 **Keyword match**, **cosine similarity**, and **Levenshtein** fallback
- 🧪 Fully tested
- 🌍 Easily used in WASM apps (via your own wrapper)

---

## ✨ Usage

Add it to your project:

```toml
[dependencies]
subslay = "0.1.8"
````

Use it in Rust:

```rust
use subslay::EmojiStylist;

fn main() {
    let stylist = EmojiStylist::new().unwrap();
    let emojis = stylist.slay("Hello, world!");
    println!("{:?}", emojis); // ["👋", "🌍"]
}
```

---

## 🧠 How It Works

SubSlay internally loads:

* `emoji.json` — maps emojis to keyword lists
* `glove.txt` — GloVe-style word embeddings

Then it:

1. Matches input words to emoji keywords
2. Uses **cosine similarity** to suggest close matches
3. Falls back to **Levenshtein distance** if all else fails

All logic is pure Rust and runs without filesystem access — ideal for embedded and WASM environments.

---

## 📁 Project Layout

```bash
subslay/
├── src/
│   └── lib.rs            # Emoji transformer engine
├── static/
│   ├── emoji.json        # Emoji → keywords (embedded)
│   └── glove.txt         # GloVe embeddings (embedded)
├── Cargo.toml
└── README.md
```

---

## ✅ Test

Run tests with:

```bash
cargo test
```

All embeddings and mappings are embedded, so no setup is needed.

---

## 🤝 Contribute

Feel free to open issues or pull requests for:

* New emoji suggestions
* Better embedding support
* Fallback improvements
* WASM wrappers or examples

---

## 📄 License

MIT © [AndriaK](mailto:hey@andriak.com)<br>
GitHub: [github.com/8ria/subslay](https://github.com/8ria/subslay)

---

> *"Built to slay, not obey."* 💅🏻