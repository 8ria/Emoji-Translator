# 😀 Emoji Translator

A web application that uses WebAssembly (Wasm) and word embeddings to translate regular text into relevant emojis. The translator finds semantic connections between your words and emoji keywords using GloVe (Global Vectors for Word Representation) embeddings.

Try it live at: [https://8ria.github.io/Emoji-Translator](https://8ria.github.io/Emoji-Translator)

## 🧠 How It Works

This application uses natural language processing techniques to find semantic similarities between input text and emoji keywords:

1. **Word Embeddings**: Uses GloVe word embeddings to convert words into vectors in a high-dimensional space where similar words are positioned closer together
2. **Cosine Similarity**: Calculates the semantic similarity between input words and emoji keywords
3. **WebAssembly**: Leverages Rust compiled to WebAssembly for efficient text processing and vector calculations
4. **Web Frontend**: Provides a simple, clean interface for entering text and displaying emoji translations

## 🛠️ Technical Stack

- **Rust**: Core translation logic and vector similarity calculations
- **WebAssembly**: Compiles Rust code for browser execution
- **JavaScript**: Glues everything together and handles the UI interaction
- **HTML/CSS**: Simple and responsive user interface

## 📦 Project Structure

```
Emoji-Translator/
├── src/
│   └── lib.rs           # Rust code for the translator
├── static/
│   ├── emoji.json       # Emoji to keywords mapping (from emojilib)
│   └── glove.txt        # GloVe word embeddings
├── Cargo.toml           # Rust package configuration
├── build.sh            # Build script
├── emoji_site.js       # Generated JavaScript bindings
├── emoji_site_bg.wasm  # Compiled WebAssembly module
└── index.html          # Web UI
```

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Setup

1. Clone this repository:
   ```
   git clone https://github.com/8ria/Emoji-Translator.git
   cd Emoji-Translator
   ```

2. Download the required data files:

   You'll need:
   - `glove.txt`: A subset of GloVe word embeddings (we recommend the 50-dimensional version)
   - `emoji.json`: Get this from [emojilib](https://github.com/muan/emojilib) (`emoji.json` file)

   Place these files in the `static/` directory.

3. Build the project:
   ```
   ./build.sh
   ```

4. Serve the directory with a local web server:
   ```
   python -m http.server
   # or any other web server of your choice
   ```

5. Open your browser and navigate to `http://localhost:8000`

## 📋 Usage

1. Wait for the application to initialize (it will load the word embeddings and emoji data)
2. Type or paste your text in the input box
3. Click "Translate" (or press Ctrl+Enter)
4. View your emoji translation!

## 📝 Data Files

### emoji.json

This project uses [emojilib](https://github.com/muan/emojilib) for emoji data. It's a JSON file mapping emoji characters to arrays of related keywords:

```json
{
  "😗": [
    "kissing_face",
    "love",
    "like",
    "face",
    "kiss",
    "duck",
    "kissy",
    "whistling"
  ],
  "☺️": [
    "smiling_face",
    "face",
    "blush",
    "happiness",
    "happy",
    "smile",
    "smiley"
  ]
}
```

### glove.txt

A text file containing word embeddings, where each line has a word followed by its vector representation:

```
the 0.418 0.24968 -0.41242 0.1217 0.34527 ...
, 0.013441 0.23682 -0.16899 0.40951 0.63812 ...
. 0.15164 0.30177 -0.16763 0.17684 0.31719 ...
```

## 🔧 Customization

- **Adjust similarity threshold**: In `lib.rs`, modify the similarity threshold (currently 0.5) to make the translator more or less strict
- **Add more emojis**: Add entries to the `emoji.json` file
- **Use different embeddings**: Replace `glove.txt` with different word embeddings and update the loading logic if needed

## 🤝 Contributing

Contributions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [GloVe](https://nlp.stanford.edu/projects/glove/) for the word embeddings
- [emojilib](https://github.com/muan/emojilib) by [@muan](https://github.com/muan) for the emoji keyword data
- [Rust WebAssembly Working Group](https://github.com/rustwasm) for `wasm-bindgen` and related tools
- All emoji creators and standardization committees!
