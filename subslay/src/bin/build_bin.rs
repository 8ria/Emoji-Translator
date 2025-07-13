use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use serde::Serialize;
use half::f16;

#[derive(Serialize)]
struct EmbeddingData {
    word_embeddings: HashMap<String, Vec<f16>>,
    emoji_vectors: HashMap<String, Vec<f16>>,
}

fn main() {
    println!("Current directory: {:?}", std::env::current_dir().unwrap());

    let glove_path = PathBuf::from("tools/glove.txt");
    let emoji_path = PathBuf::from("tools/emoji.txt");

    let glove_txt = fs::read_to_string(&glove_path).expect("Missing glove.txt");
    let emoji_txt = fs::read_to_string(&emoji_path).expect("Missing emoji.txt");

    let mut word_embeddings = HashMap::new();
    for line in glove_txt.lines() {
        let mut parts = line.split_whitespace();
        if let Some(word) = parts.next() {
            let vec: Vec<f16> = parts
                .map(|s| {
                    let val = s.parse::<f32>().unwrap_or(0.0);
                    f16::from_f32(val)
                })
                .collect();
            word_embeddings.insert(word.to_string(), vec);
        }
    }

    let mut emoji_vectors = HashMap::new();
    for line in emoji_txt.lines() {
        let mut parts = line.split_whitespace();
        if let Some(emoji) = parts.next() {
            let vec: Vec<f16> = parts
                .map(|s| {
                    let val = s.parse::<f32>().unwrap_or(0.0);
                    f16::from_f32(val)
                })
                .collect();
            emoji_vectors.insert(emoji.to_string(), vec);
        }
    }

    let data = EmbeddingData {
        word_embeddings,
        emoji_vectors,
    };

    let bin = bincode::serialize(&data).expect("bincode serialization failed");
    let mut fout = File::create("data/embeddings.bin").expect("cannot write output");
    fout.write_all(&bin).expect("write failed");

    println!("✅ Generated quantized embeddings.bin (f16)");
}
