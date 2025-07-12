use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use strsim::levenshtein;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub struct EmojiTransformer {
    keyword_to_emoji: HashMap<String, String>,
    word_embeddings: HashMap<String, Vec<f32>>,
}

#[wasm_bindgen]
impl EmojiTransformer {
    #[wasm_bindgen(constructor)] 
    pub fn new(emoji_json: &str, glove_txt: &str) -> Result<EmojiTransformer, JsValue> {
        let emoji_keyword_map: HashMap<String, Vec<String>> =
            serde_json::from_str(emoji_json).map_err(|e| JsValue::from_str(&format!("Emoji JSON error: {}", e)))?; 

        let mut keyword_to_emoji = HashMap::new();
        for (emoji, keywords) in emoji_keyword_map {
            for keyword in keywords {
                keyword_to_emoji.insert(keyword, emoji.clone());
            }
        }

        let mut word_embeddings = HashMap::new();
        for line in glove_txt.lines() {
            let mut parts = line.split_whitespace();
            if let Some(word) = parts.next() {
                let embedding_vector = parts.map(|s| s.parse::<f32>().unwrap_or(0.0)).collect::<Vec<f32>>();
                word_embeddings.insert(word.to_string(), embedding_vector);
            }
        }

        Ok(EmojiTransformer { 
            keyword_to_emoji, word_embeddings,
        })
    }

    #[wasm_bindgen]
    pub fn slay(&self, input_text: &str) -> JsValue {
        let cleaned_words: Vec<String> = input_text
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        let mut slayified_output = vec![];

        for word in cleaned_words {
            if let Some(emoji) = self.keyword_to_emoji.get(&word) {
                slayified_output.push(emoji.clone());
                continue;
            }

            if let Some(input_embedding) = self.word_embeddings.get(&word) {
                let mut closest_keyword_match: Option<(String, f32)> = None; 

                for(keyword, emoji) in &self.keyword_to_emoji {
                    if let Some(keyword_embedding) = self.word_embeddings.get(keyword) {
                        let sim = cosine_similarity(input_embedding, keyword_embedding);
                        if closest_keyword_match.is_none() || sim > closest_keyword_match .as_ref().unwrap().1 { 
                            closest_keyword_match = Some((emoji.clone(), sim));
                        }
                    }
                }
                if let Some((best_emoji, _)) = closest_keyword_match  {
                    slayified_output.push(best_emoji);
                    continue;
                }
            }

            let mut closest_keyword_match: Option<(String, usize)> = None;
            for (keyword, emoji) in &self.keyword_to_emoji {
                let levenshtein_score = levenshtein(&word, keyword);
                if closest_keyword_match.is_none() || levenshtein_score < closest_keyword_match.as_ref().unwrap().1 {
                    closest_keyword_match = Some((emoji.clone(), levenshtein_score));
                }
            }

            if let Some((best_fallback_emoji, _)) = closest_keyword_match {
                slayified_output.push(best_fallback_emoji);
            } else {
                slayified_output.push("@".to_string());
            }

        }
        to_value(&slayified_output).unwrap()
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
