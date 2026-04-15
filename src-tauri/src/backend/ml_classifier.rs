use std::path::PathBuf;

use crate::backend::ai;
use crate::domain::file_meta::FileMeta;
use crate::domain::classification::{ClassificationResult};

pub struct MLClassifier {
    pub model: String,
}

impl MLClassifier {
    fn build_prompt(
        &self,
        meta: &FileMeta,
        descriptions: &[(String, String)],
    ) -> String {
        let category_block = descriptions
            .iter()
            .map(|(path, desc)| {
                format!(
                    "- path: {}\n  description: {}",
                    path, desc
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
r#"
You are a file organization assistant.

Assign the file to the BEST matching folder.

You MUST choose ONE of the provided folders.
Do NOT invent new folders.

Return ONLY valid JSON:
{{
  "selected_path": "...",
  "confidence": 0.0,
  "reasoning": "..."
}}

File Metadata:
- name: {}
- extension: {}
- mime: {}
- size_bytes: {}
- timestamp: {}

Available Folders:
{}

Rules:
- Use file name and extension as primary signals
- Use mime and size as supporting signals
- Choose the MOST specific match
- Never return a path not listed
"#,
            meta.file_name,
            meta.extension.as_deref().unwrap_or("unknown"),
            meta.mime.as_deref().unwrap_or("unknown"),
            meta.size,
            meta.timestamp,
            category_block
        )
    }

    fn parse_response(&self, response: &str) -> ClassificationResult {
        let cleaned = response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let parsed: serde_json::Value = match serde_json::from_str(cleaned) {
            Ok(v) => v,
            Err(_) => {
                return ClassificationResult {
                    category: "unknown".into(),
                    confidence: 0.0,
                    suggested_path: PathBuf::new(),
                    reasoning: Some("Failed to parse AI response".into()),
                };
            }
        };

        let path = parsed["selected_path"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let confidence = parsed["confidence"]
            .as_f64()
            .unwrap_or(0.0) as f32;

        let reasoning = parsed["reasoning"]
            .as_str()
            .map(|s| s.to_string());

        ClassificationResult {
            category: path.clone(),
            confidence,
            suggested_path: PathBuf::from(path),
            reasoning,
        }
    }

    pub async fn classify_with_context(
        &self,
        meta: &FileMeta,
        descriptions: Vec<(String, String)>,
    ) -> ClassificationResult {
        let prompt = self.build_prompt(meta, &descriptions);

        let response = match ai::generate(&self.model, &prompt).await {
            Ok(r) => r,
            Err(e) => {
                return ClassificationResult {
                    category: "unknown".into(),
                    confidence: 0.0,
                    suggested_path: PathBuf::new(),
                    reasoning: Some(format!("AI error: {}", e)),
                };
            }
        };

        self.parse_response(&response)
    }
}