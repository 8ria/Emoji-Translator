use std::collections::HashMap;

const EMOJI_TXT: &str = include_str!("../data/emoji.txt");
const GLOVE_TXT: &str = include_str!("../data/glove.txt");

pub struct EmojiStylist {
    word_embeddings: HashMap<String, Vec<f32>>,
    emoji_vectors: HashMap<String, Vec<f32>>,
}

impl EmojiStylist {
    pub fn new() -> Result<Self, String> {
        let mut word_embeddings = HashMap::new();
        for line in GLOVE_TXT.lines() {
            let mut parts = line.split_whitespace();
            if let Some(word) = parts.next() {
                let vec = parts.map(|x| x.parse::<f32>().unwrap_or(0.0)).collect();
                word_embeddings.insert(word.to_string(), vec);
            }
        }

        let mut emoji_vectors = HashMap::new();
        for line in EMOJI_TXT.lines() {
            let mut parts = line.split_whitespace();
            if let Some(emoji) = parts.next() {
                let vec = parts.map(|x| x.parse::<f32>().unwrap_or(0.0)).collect();
                emoji_vectors.insert(emoji.to_string(), vec);
            }
        }

        Ok(EmojiStylist {
            word_embeddings,
            emoji_vectors,
        })
    }

    pub fn slay(&self, input_text: &str) -> Vec<String> {
        let cleaned = input_text
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>();

        let words = cleaned
            .split_whitespace()
            .map(|s| s.to_lowercase());

        let mut output = vec![];

        for word in words {
            if let Some(word_vec) = self.word_embeddings.get(&word) {
                let best_emoji = self.emoji_vectors
                    .iter()
                    .map(|(emoji, vec)| (emoji, cosine_similarity(word_vec, vec)))
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                if let Some((emoji, _)) = best_emoji {
                    output.push(emoji.clone());
                } else {
                    output.push("@".to_string());
                }
            } else {
                output.push("@".to_string());
            }
        }

        output
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return -1.0;
    }
    dot / (norm_a * norm_b)
}

#[test]
fn test_vector_based_matching() {
    let s = EmojiStylist::new().unwrap();
    let result = s.slay("happy pizza");
    assert!(!result.is_empty());

    for emoji in result {
        assert_ne!(emoji, "@", "Expected valid emoji, got fallback '@'");
    }
}
