use std::collections::HashMap;
use crate::diagnostic::ErrorCategory;
use crate::japanese::rules::all_rules;
use crate::japanese::template::JapaneseDiagnostic;

pub struct ExplanationService;

impl ExplanationService {
    pub fn list_all() -> Vec<JapaneseDiagnostic> {
        all_rules().into_iter().map(|r| r.general_explanation()).collect()
    }

    pub fn search(query: &str) -> Vec<JapaneseDiagnostic> {
        let q = query.to_lowercase();
        all_rules()
            .into_iter()
            .map(|r| r.general_explanation())
            .filter(|jd| {
                jd.code.to_lowercase().contains(&q)
                    || jd.title.to_lowercase().contains(&q)
                    || jd.summary.to_lowercase().contains(&q)
                    || jd.reason.to_lowercase().contains(&q)
                    || jd.solution.to_lowercase().contains(&q)
                    || jd.category.name_ja().to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn stats() -> (usize, HashMap<ErrorCategory, usize>) {
        let rules = all_rules();
        let total = rules.len();
        let mut map = HashMap::new();
        for r in rules {
            *map.entry(r.category()).or_insert(0) += 1;
        }
        (total, map)
    }
}
