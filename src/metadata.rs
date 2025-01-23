use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::models::{EnrichedNotepad, NotepadData, NotepadMetadata, NotepadStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataStore {
    notepads: HashMap<String, NotepadMetadata>,
    categories: Vec<String>,
    templates: HashMap<String, String>,
}

impl MetadataStore {
    pub fn new() -> Self {
        Self {
            notepads: HashMap::new(),
            categories: vec![
                "tasks".to_string(),
                "technical".to_string(),
                "tests".to_string(),
                "docs".to_string(),
            ],
            templates: HashMap::new(),
        }
    }

    pub fn load(path: PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn enrich_notepad(&self, id: &str, data: NotepadData) -> EnrichedNotepad {
        if let Some(metadata) = self.notepads.get(id) {
            EnrichedNotepad {
                original_data: data,
                metadata: metadata.clone(),
            }
        } else {
            // Default to "general" category if no metadata exists
            EnrichedNotepad::new(data, "general".to_string())
        }
    }

    pub fn update_metadata(&mut self, id: String, metadata: NotepadMetadata) {
        self.notepads.insert(id, metadata);
    }

    pub fn get_by_category(&self, category: &str) -> Vec<(&String, &NotepadMetadata)> {
        self.notepads
            .iter()
            .filter(|(_, meta)| meta.category == category)
            .collect()
    }

    pub fn get_by_tag(&self, tag: &str) -> Vec<(&String, &NotepadMetadata)> {
        self.notepads
            .iter()
            .filter(|(_, meta)| meta.tags.contains(&tag.to_string()))
            .collect()
    }

    pub fn get_by_status(&self, target_status: NotepadStatus) -> Vec<(&String, &NotepadMetadata)> {
        self.notepads
            .iter()
            .filter(|(_, meta)| std::mem::discriminant(&meta.status) == std::mem::discriminant(&target_status))
            .collect()
    }

    pub fn add_category(&mut self, category: String) {
        if !self.categories.contains(&category) {
            self.categories.push(category);
        }
    }

    pub fn add_template(&mut self, name: String, content: String) {
        self.templates.insert(name, content);
    }

    pub fn get_template(&self, name: &str) -> Option<&String> {
        self.templates.get(name)
    }

    pub fn list_categories(&self) -> &[String] {
        &self.categories
    }

    pub fn list_templates(&self) -> Vec<&String> {
        self.templates.keys().collect()
    }
} 