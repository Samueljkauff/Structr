use crate::backend::db;
use crate::backend::ml_classifier::MLClassifier;
use crate::domain::file_meta::FileMeta;
use crate::domain::classification::ClassificationResult;

pub struct ClassificationService {
    pub classifier: MLClassifier,
}

impl ClassificationService {
    pub async fn classify(
        &self,
        app: &tauri::AppHandle,
        meta: &FileMeta
    ) -> ClassificationResult {

        let descriptions = match db::get_all_descriptions(app) {
            Ok(d) => d,
            Err(_) => {
                return ClassificationResult {
                    category: "unknown".into(),
                    confidence: 0.0,
                    suggested_path: Default::default(),
                    reasoning: Some("DB error".into()),
                };
            }
        };

        let result = self
            .classifier
            .classify_with_context(meta, descriptions)
            .await;

        result
    }
}