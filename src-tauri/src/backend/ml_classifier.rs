use std::path::PathBuf;
use crate::domain::file_meta::FileMeta;
use crate::domain::classification::{ClassificationResult, Classifier};

pub struct MLClassifier {
    pub model: String,
}

impl Classifier for MLClassifier {
    fn classify(&self, meta: &FileMeta) -> ClassificationResult {
        println!("{:?}", meta);
        return ClassificationResult { category: "".to_string(), confidence: 1.0, suggested_path: PathBuf::from(""), reasoning: Some("".to_string()) };
    }
}

