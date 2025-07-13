**SubSlay** converts plain text into emoji sequences using semantic similarity with pre-embedded GloVe vectors and emoji keyword mappings — all bundled inside a pure Rust crate.

```rust
use subslay::EmojiStylist;

fn main() {
    let stylist = EmojiStylist::new().unwrap();
    let emojis = stylist.slay("Hello, world!");
    println!("{:?}", emojis); // ["👋", "🌍"]
}
```

---

## 🧾 What is SubSlay?

**SubSlay** is a Rust library that transforms ordinary text into emoji sequences that capture the **semantic meaning** of each word. It uses **pre-trained GloVe word embeddings** and maps them to emojis using embedded semantic vectors — all packed into the crate itself.

This enables fast, expressive emoji translation **fully offline** with zero API calls or external resources.

It’s ideal for:

* ✨ Chat apps
* 🤖 Discord bots
* 📱 Social media tools
* 💻 Terminal utilities

---

## 🚀 Features

* 🦀 **Pure Rust** — no runtime dependencies
* 📦 **Embedded embeddings** — GloVe + emoji vectors bundled as a single `embeddings.bin` file
* 🧠 **Cosine similarity** with emoji semantics for contextual matching
* 🆘 **Fallback handling** for unknown or out-of-vocab words
* 🌐 **WASM-compatible** — works in browser/Edge without filesystem or network access
* 💨 **Blazing fast & offline** — powered by `bincode` and quantized vector loading
* 🪶 **Tiny footprint** — uses `f16` (half-precision) vectors for compact size

---

## 📦 Installation

Add SubSlay to your `Cargo.toml`:

```toml
[dependencies]
subslay = "0.1.9"
```

---

## 🛠️ Usage

```rust
use subslay::EmojiStylist;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stylist = EmojiStylist::new()?;
    let emojis = stylist.slay("happy pizza");
    println!("{:?}", emojis); // e.g. ["😊", "🍕"]
    Ok(())
}
```

### Example:

```rust
let stylist = EmojiStylist::new()?;
let sentence = "I love pizza and cats";
let emojis = stylist.slay(sentence);
println!("Emojis: {:?}", emojis);
// Possible output: ["❤️", "🍕", "😺"]
```

---

## ⚙️ How It Works (Under the Hood)

SubSlay embeds a pre-serialized binary file (`embeddings.bin`) using `include_bytes!`, which includes:

* 🔤 A `HashMap<String, Vec<f16>>` of words mapped to **quantized 50D GloVe embeddings**
* 😎 A `HashMap<String, Vec<f16>>` of emojis mapped to semantic emoji vectors

When `.slay(&str)` is called:

1. The input is cleaned (only letters and spaces).
2. Words are lowercased and split.
3. Each word:

   * Tries to fetch a GloVe embedding
   * Computes cosine similarity with all emoji vectors
   * Selects the closest emoji
   * Falls back to `"@"` if no match is found

Cosine similarity is calculated like this:

```rust
fn cosine_similarity(a: &[f16], b: &[f16]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x.to_f32() * y.to_f32()).sum::<f32>();
    let norm_a = a.iter().map(|x| x.to_f32().powi(2)).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x.to_f32().powi(2)).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return -1.0;
    }
    dot / (norm_a * norm_b)
}
```

---

## 🎯 API Overview

| Method                | Description                                                                    |
| --------------------- | ------------------------------------------------------------------------------ |
| `EmojiStylist::new()` | Initializes the struct by deserializing the embedded binary file               |
| `stylist.slay(&str)`  | Translates each word in the input into a matching emoji, returns `Vec<String>` |

---

## 🧪 Testing

SubSlay includes unit tests for:

* ✅ Correct output for known words like `"pizza"`
* ✅ Full sentence translation
* ✅ Unknown/fallback behavior
* ✅ Empty input

Run tests:

```bash
cargo test
```

---

## 📁 Package Contents

* `src/lib.rs` — Main logic for translation and similarity
* `data/embeddings.bin` — Serialized, quantized GloVe and emoji vectors
* `Cargo.toml` — Dependency config
* `README.md` — You’re reading it

---

## 🧑‍💻 Developer Notes

* Embeddings are stored using **quantized half-precision floats (`f16`)** to reduce file size
* All data is serialized using `bincode`
* Pre-trained **GloVe 50d** embeddings are used for English vocabulary
* The emoji vectors are handcrafted or generated using centroid-like heuristics

---

## 💡 Future Plans

* Weighted phrase-to-emoji aggregation
* Fuzzy matching for slang and typos
* Custom user dictionaries or theme packs
* Parallel or batch `.slay()` calls
* Embedding compression + streaming load

---

## 📜 License

MIT © [AndriaK](mailto:hey@andriak.com)<br>
[GitHub Repository](https://github.com/8ria/subslay)<br>
[Live Demo](https://subslay.app/)

---

> *"Built to slay, not obey."* 💅🏻
