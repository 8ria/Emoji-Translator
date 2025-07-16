# 🤖 SubSlay Discord Bot

A stylish Discord bot that transforms your text into expressive emoji-enhanced messages using the power of the [`subslay`](https://github.com/yourusername/subslay) library — all running in pure Rust.

## ✨ What It Does

The SubSlay Discord bot listens for messages starting with `!slay` and responds with a transformed version that adds emojis for dramatic, funny, or expressive flair. It's built using the [Serenity](https://github.com/serenity-rs/serenity) library for Discord bots in Rust and powered by the custom `subslay` emoji styling engine.

---

## 🛠️ Features

* 🧠 Intelligent emoji insertion with [`subslay`](../subslay)
* ⚡ Fast and efficient (Rust + async + LTO + release optimized)
* 🛡️ Secure (uses rustls backend for TLS)
* 🧵 Multi-threaded runtime with Tokio
* 🔒 `.env`-based token management

---

## 🚀 Getting Started

### 1. Prerequisites

* Rust (latest stable recommended): [Install Rust](https://rustup.rs)
* A Discord Bot Token ([guide](https://discord.com/developers/applications))
* `subslay` crate built and available at `../subslay`

---

### 2. Clone and Setup

```bash
git clone https://github.com/yourusername/subslay.git
cd subslay/discord-bot
```

### 3. Configure Your Token

Copy the `.env.example` and set your bot token:

```bash
cp .env.example .env
```

Edit `.env`:

```env
DISCORD_TOKEN=your_token_here
```

---

### 4. Run the Bot

In release mode:

```bash
cargo run --release
```

Or in dev mode:

```bash
cargo run
```

---

## 🧪 Usage

In any Discord channel where the bot is present:

```
!slay Hello world
```

Bot reply:

```
👋 🌎
```

(Example only – real output depends on `subslay`'s emoji matching!)

---

## 📁 Project Structure

```
discord-bot/
├── src/
│   └── main.rs        # Entry point of the Discord bot
├── .env.example       # Environment variable template
├── Cargo.toml         # Bot dependencies and config
```

---

## 🤝 Dependencies

* [`serenity`](https://crates.io/crates/serenity) — Discord API library
* [`tokio`](https://crates.io/crates/tokio) — Async runtime
* [`dotenv`](https://crates.io/crates/dotenv) — Load `.env` files
* [`subslay`](../subslay) — Local crate for emoji transformation

---

## 🧩 Integration

The bot uses:

```rust
if msg.content.starts_with("!slay ") {
    let input = msg.content.trim_start_matches("!slay ");
    let output = self.stylist.slay(input).join(" ");
}
```

You can easily adapt this for other platforms or commands.

---

## 🛡️ Security

Never commit your `.env` file or token. The `.env.example` is safe to share.

---

## 📜 License

MIT — free to use, modify, and distribute. See [`LICENSE`](../LICENSE) in the root of the repo.

---

## 👤 Author

Created by [@8ria](https://github.com/8ria) as part of the [`subslay`](../subslay) project.
