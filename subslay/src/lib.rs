use std::collections::HashMap;
use strsim::levenshtein;
use serde::Deserialize;
use half::f16;

const EMBEDDINGS_BIN: &[u8] = include_bytes!("../data/embeddings.bin");

#[derive(Deserialize)]
struct EmbeddingData {
    word_embeddings: HashMap<String, Vec<f16>>,
    emoji_vectors: HashMap<String, Vec<f16>>,
}

pub struct EmojiStylist {
    word_embeddings: HashMap<String, Vec<f16>>,
    emoji_vectors: HashMap<String, Vec<f16>>,
}

impl EmojiStylist {
    pub fn new() -> Result<Self, String> {
        println!("embedding size = {}", EMBEDDINGS_BIN.len());

        let data: EmbeddingData = bincode::deserialize(EMBEDDINGS_BIN)
            .map_err(|e| format!("Deserialization failed: {}", e))?;

        Ok(EmojiStylist {
            word_embeddings: data.word_embeddings,
            emoji_vectors: data.emoji_vectors,
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
            let mut word_vec = self.word_embeddings.get(&word);

            let mut emoji = word_vec.and_then(|vec| {
                self.emoji_vectors
                    .iter()
                    .map(|(emoji, em_vec)| (emoji, cosine_similarity(vec, em_vec)))
                    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                    .map(|(emoji, _)| emoji.clone())
            });

            if emoji.is_none() {
                if let Some((closest_word, _)) = self.word_embeddings
                    .keys()
                    .map(|k| (k, levenshtein(&word, k)))
                    .min_by_key(|(_, dist)| *dist)
                {
                    word_vec = self.word_embeddings.get(closest_word);

                    emoji = word_vec.and_then(|vec| {
                        self.emoji_vectors
                            .iter()
                            .map(|(emoji, em_vec)| (emoji, cosine_similarity(vec, em_vec)))
                            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                            .map(|(emoji, _)| emoji.clone())
                    });
                }
            }
            output.push(emoji.unwrap_or_else(|| "@".to_string()));
        }
        output
    }
}

fn cosine_similarity(a: &[f16], b: &[f16]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x.to_f32() * y.to_f32()).sum::<f32>();
    let norm_a = a.iter().map(|x| x.to_f32().powi(2)).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x.to_f32().powi(2)).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return -1.0;
    }
    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_matching_single_word() {
        let stylist = EmojiStylist::new().expect("Failed to load embeddings");
        let result = stylist.slay("pizza");
        assert_eq!(result.len(), 1);
        assert_ne!(result[0], "@", "Should return a valid emoji for 'pizza'");
    }

    #[test]
    fn test_vector_matching_multiple_words() {
        let stylist = EmojiStylist::new().expect("Failed to load embeddings");
        let result = stylist.slay("happy pizza love");
        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|e| e != "@"), "All words should return valid emojis");
    }

    #[test]
    fn test_empty_input() {
        let stylist = EmojiStylist::new().expect("Failed to load embeddings");
        let result = stylist.slay("");
        assert!(result.is_empty());
    }
}
