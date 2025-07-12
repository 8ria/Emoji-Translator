use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use strsim::levenshtein;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
pub struct EmojiTransformer {
    keyword_to_emoji: HashMap<String, String>,
    embeddings: HashMap<String, Vec<f32>>,
}

#[wasm_bindgen]
impl EmojiTransformer {
    #[wasm_bindgen(constructor)] 
    pub fn new(emoji_json: &str, glove_txt: &str) -> Result<EmojiTransformer, JsValue> {
        let emoji_to_keywords: HashMap<String, Vec<String>> =
            serde_json::from_str(emoji_json).map_err(|e| JsValue::from_str(&format!("Emoji JSON error: {}", e)))?; 

        let mut keyword_to_emoji = HashMap::new();
        for (emoji, keywords) in emoji_to_keywords {
            for keyword in keywords {
                keyword_to_emoji.insert(keyword, emoji.clone());
            }
        }

        let mut embeddings = HashMap::new();
        for line in glove_txt.lines() {
            let mut parts = line.split_whitespace();
            if let Some(word) = parts.next() {
                let vec = parts.map(|s| s.parse::<f32>().unwrap_or(0.0)).collect::<Vec<f32>>();
                embeddings.insert(word.to_string(), vec);
            }
        }

        Ok(EmojiTransformer { 
            keyword_to_emoji, embeddings,
        })
    }

    #[wasm_bindgen]
    pub fn transform(&self, input_text: &str) -> JsValue {
        let words: Vec<String> = input_text
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        let mut result = vec![];

        for word in words {
            if let Some(emoji) = self.keyword_to_emoji.get(&word) {
                result.push(emoji.clone());
                continue;
            }

            if let Some(word_embedding) = self.embeddings.get(&word) {
                let mut best_match: Option<(String, f32)> = None; 

                for(keyword, emoji) in &self.keyword_to_emoji {
                    if let Some(keyword_embedding) = self.embeddings.get(keyword) {
                        let sim = cosine_similarity(word_embedding, keyword_embedding);
                        if best_match.is_none() || sim > best_match.as_ref().unwrap().1 { 
                            best_match = Some((emoji.clone(), sim));
                        }
                    }
                }
                
                if let Some((best_emoji, _)) = best_match {
                    result.push(best_emoji);
                    continue;
                }
            }

            let mut best_match: Option<(String, usize)> = None;
            for (keyword, emoji) in &self.keyword_to_emoji {
                let dist = levenshtein(&word, keyword);
                if best_match.is_none() || dist < best_match.as_ref().unwrap().1 {
                    best_match = Some((emoji.clone(), dist));
                }
            }

            if let Some((emoji, _)) = best_match {
                result.push(emoji);
            } else {
                result.push("@".to_string());
            }
        }
        to_value(&result).unwrap()
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
