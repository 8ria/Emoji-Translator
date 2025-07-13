use std::collections::HashMap;
use strsim::levenshtein;

const EMOJI_JSON: &str = include_str!("../static/emoji.json");
const GLOVE_TXT: &str = include_str!("../static/glove.txt");

pub struct EmojiStylist {
    keyword_to_emoji: HashMap<String, String>,
    word_embeddings: HashMap<String, Vec<f32>>,
}

impl EmojiStylist {
    pub fn new() -> Result<Self, String> {
        let emoji_keyword_map: HashMap<String, Vec<String>> =
            serde_json::from_str(EMOJI_JSON).map_err(|e| format!("Emoji JSON error: {}", e))?;
        
        let mut keyword_to_emoji = HashMap::new();
        for (emoji, keywords) in emoji_keyword_map {
            for keyword in keywords {
                keyword_to_emoji.insert(keyword, emoji.clone());
            }
        }

        let mut word_embeddings = HashMap::new();
        for line in GLOVE_TXT.lines() {
            let mut parts = line.split_whitespace();
            if let Some(word) = parts.next() {
                let embedding_vector = parts
                    .map(|s| s.parse::<f32>().unwrap_or(0.0))
                    .collect::<Vec<f32>>();
                word_embeddings.insert(word.to_string(), embedding_vector);
            }
        }

        Ok(EmojiStylist {
            keyword_to_emoji,
            word_embeddings,
        })
    }

    pub fn slay(&self, input_text: &str) -> Vec<String> {
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

                for (keyword, emoji) in &self.keyword_to_emoji {
                    if let Some(keyword_embedding) = self.word_embeddings.get(keyword) {
                        let sim = cosine_similarity(input_embedding, keyword_embedding);
                        if closest_keyword_match.is_none()
                            || sim > closest_keyword_match.as_ref().unwrap().1
                        {
                            closest_keyword_match = Some((emoji.clone(), sim));
                        }
                    }
                }
                if let Some((best_emoji, _)) = closest_keyword_match {
                    slayified_output.push(best_emoji);
                    continue;
                }
            }

            let mut closest_keyword_match: Option<(String, usize)> = None;
            for (keyword, emoji) in &self.keyword_to_emoji {
                let levenshtein_score = levenshtein(&word, keyword);
                if closest_keyword_match.is_none()
                    || levenshtein_score < closest_keyword_match.as_ref().unwrap().1
                {
                    closest_keyword_match = Some((emoji.clone(), levenshtein_score));
                }
            }

            if let Some((best_fallback_emoji, _)) = closest_keyword_match {
                slayified_output.push(best_fallback_emoji);
            } else {
                slayified_output.push("@".to_string());
            }
        }

        slayified_output
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_match() {
        let s = EmojiStylist::new().unwrap();
        assert!(s.slay("hello").contains(&"👋".to_string()));
    }

    #[test]
    fn test_levenshtein_fallback() {
        let s = EmojiStylist::new().unwrap();
        let result = s.slay("helllo");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_empty_input() {
        let s = EmojiStylist::new().unwrap();
        let result = s.slay("");
        assert!(result.is_empty() || result == vec!["@"]);
    }
}
