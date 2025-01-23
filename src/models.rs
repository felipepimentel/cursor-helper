use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub folder: String,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub project_name: Option<String>,
    pub path: PathBuf,
}

impl Workspace {
    pub fn display_name(&self) -> String {
        self.project_name.clone().unwrap_or_else(|| self.id.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotepadData {
    List(Vec<String>),
    Raw(serde_json::Value),
}

impl Default for NotepadData {
    fn default() -> Self {
        NotepadData::List(Vec::new())
    }
}

impl NotepadData {
    pub fn ensure_list(&mut self) {
        if let NotepadData::Raw(_) = self {
            *self = NotepadData::List(Vec::new());
        }
    }

    pub fn as_list(&self) -> Option<&Vec<String>> {
        match self {
            NotepadData::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<String>> {
        match self {
            NotepadData::List(list) => Some(list),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotepadMetadata {
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub references: Vec<String>,
    pub status: NotepadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotepadStatus {
    Active,
    Archived,
    Completed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedNotepad {
    pub original_data: NotepadData,
    pub metadata: NotepadMetadata,
}

impl EnrichedNotepad {
    pub fn new(data: NotepadData, category: String) -> Self {
        Self {
            original_data: data,
            metadata: NotepadMetadata {
                category,
                tags: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                references: Vec::new(),
                status: NotepadStatus::Active,
            },
        }
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.metadata.tags.contains(&tag) {
            self.metadata.tags.push(tag);
            self.metadata.updated_at = Utc::now();
        }
    }

    pub fn add_reference(&mut self, reference: String) {
        if !self.metadata.references.contains(&reference) {
            self.metadata.references.push(reference);
            self.metadata.updated_at = Utc::now();
        }
    }

    pub fn set_status(&mut self, status: NotepadStatus) {
        self.metadata.status = status;
        self.metadata.updated_at = Utc::now();
    }
} 