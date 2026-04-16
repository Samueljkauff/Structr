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

You MUST return ONLY valid JSON.
DO NOT include any text before or after the JSON.
DO NOT use markdown.
DO NOT use ``` blocks.

The JSON MUST exactly match this schema:
{{
  "selected_path": "string",
  "confidence": number,
  "reasoning": "string"
}}

Rules:
- You MUST choose EXACTLY ONE path from the provided list
- You MUST NOT invent new paths
- "selected_path" MUST be one of the provided folders
- "confidence" MUST be a number between 0 and 1

Decision Rules (VERY IMPORTANT):
- Match files primarily using FILE NAME KEYWORDS
- If a file name contains keywords found in a folder description, that folder MUST be selected
- Course codes (e.g. CS-450, BUSA-103, COMM-319, KNGT-450) are STRONG signals and override file type
- Academic terms like "syllabus", "assignment", "project", "report" indicate course folders
- File type (PDF, JPG, etc.) is a WEAK signal compared to keywords

Fallback Rule:
- The Downloads folder should ONLY be selected if NO strong keyword match exists
- If multiple folders match, choose the MOST specific one

Example output:
{{
  "selected_path": "/Users/example/Pictures",
  "confidence": 0.82,
  "reasoning": "File is a JPG image, best fits Pictures folder"
}}

File Metadata:
- name: {}
- extension: {}
- mime: {}
- size_bytes: {}
- timestamp: {}

Available Folders:
{}

Now respond with ONLY the JSON.
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

        let mut cleaned = cleaned.to_string();

        if let (Some(start), Some(end)) = (cleaned.find('{'), cleaned.rfind('}')) {
            cleaned = cleaned[start..=end].to_string();
        }

        if !cleaned.trim_end().ends_with('}') {
            cleaned.push('}');
        }

        let parsed: serde_json::Value = match serde_json::from_str(&cleaned) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("JSON parse failed: {}", e);
                eprintln!("Cleaned response was:\n{}", cleaned);

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
        println!("RAW MODEL OUTPUT:\n{}", response);
        self.parse_response(&response)
    }
}