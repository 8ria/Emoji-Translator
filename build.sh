#!/bin/bash
# Create directories if they don't exist
mkdir -p static

# Check for wasm-pack
command -v wasm-pack &>/dev/null || cargo install wasm-pack

# Build and copy files
wasm-pack build --target web
cp pkg/*.js .
cp pkg/*.wasm .

echo "Build complete. Don't forget to add your data files to the static folder:"
echo "  - static/emoji.json"
echo "  - static/glove.6B.50d.txt"