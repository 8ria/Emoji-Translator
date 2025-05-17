use std::collections::HashMap;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EmojiTranslator {
    embeddings: HashMap<String, Vec<f32>>,
    emoji_keywords: HashMap<String, Vec<String>>,
}

#[wasm_bindgen]
impl EmojiTranslator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            embeddings: HashMap::new(),
            emoji_keywords: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, glove_data: &str, emoji_json: &str) -> Result<(), JsValue> {

        self.load_glove_embeddings(glove_data)
            .map_err(|e| JsValue::from_str(&e))?;

        self.load_emoji_keywords(emoji_json)
            .map_err(|e| JsValue::from_str(&e))?;

        Ok(())
    }

    fn load_glove_embeddings(&mut self, data: &str) -> Result<(), String> {
        let lines = data.lines();

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() > 1 {
                if let Some(word) = parts.first() {
                    let values = &parts[1..];
                    let vector: Vec<f32> = values
                        .iter()
                        .filter_map(|s| s.parse::<f32>().ok())
                        .collect();

                    if !vector.is_empty() {
                        self.embeddings.insert(word.to_string(), vector);
                    }
                }
            }
        }
        Ok(())
    }

    fn load_emoji_keywords(&mut self, json_str: &str) -> Result<(), String> {
        let json: Value = serde_json::from_str(json_str)
            .map_err(|e| format!("Error parsing JSON: {}", e))?;

        if let Value::Object(map) = json {
            for (emoji, keywords) in map {
                if let Value::Array(keyword_array) = keywords {
                    let keywords_vec: Vec<String> = keyword_array
                        .iter()
                        .filter_map(|k| k.as_str().map(String::from))
                        .collect();

                    self.emoji_keywords.insert(emoji, keywords_vec);
                }
            }
        }
        Ok(())
    }

    pub fn translate_text(&self, text: &str) -> String {
        let filtered_text = self.filter_text(text);
        self.process_text(&filtered_text)
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {

        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        match (norm_a, norm_b) {
            (0.0, _) | (_, 0.0) => 0.0,
            _ => dot_product / (norm_a * norm_b)
        }
    }

    fn process_text(&self, text: &str) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut result: Vec<String> = Vec::with_capacity(words.len());

        for word in words.iter() {
            let word_lc = word.to_lowercase();
            let mut best_match: Option<(String, f32)> = None;

            if let Some(word_vec) = self.embeddings.get(&word_lc) {
                for (emoji, keywords) in &self.emoji_keywords {
                    for kw in keywords {
                        if let Some(kw_vec) = self.embeddings.get(kw) {
                            let sim = self.cosine_similarity(word_vec, kw_vec);

                            if sim > 0.5 && (best_match.is_none() || sim > best_match.as_ref().unwrap().1) {
                                best_match = Some((emoji.clone(), sim));
                            }
                        }
                    }
                }

                if let Some((emoji, _)) = best_match {
                    result.push(emoji);
                }
            }
        }
        result.join(" ")
    }

    fn filter_text(&self, text: &str) -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace() || c == &'.')
            .collect()
    }
}

#[wasm_bindgen]
pub fn initialize() -> i32 {
    0
}
