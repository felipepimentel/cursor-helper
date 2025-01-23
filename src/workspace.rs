use std::fs;
use crate::error::Result;
use crate::models::Workspace;
use serde_json::Value;
use std::path::PathBuf;
use rusqlite;

fn extract_project_name(path: &PathBuf) -> Option<String> {
    // Try to read workspace.json first
    let workspace_json = path.join("workspace.json");
    if workspace_json.exists() {
        if let Ok(content) = fs::read_to_string(&workspace_json) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                // Try to get the project name from the folder path
                if let Some(folder) = json.get("folder").and_then(|v| v.as_str()) {
                    let path = PathBuf::from(folder);
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        return Some(name.to_string());
                    }
                }
            }
        }
    }

    // If workspace.json doesn't exist or doesn't contain valid data,
    // try to read workspace.name from state.vscdb
    let state_db = path.join("state.vscdb");
    if state_db.exists() {
        if let Ok(conn) = rusqlite::Connection::open(&state_db) {
            let mut stmt = conn.prepare("SELECT value FROM ItemTable WHERE key = 'workspace.name'").ok()?;
            let name: Option<String> = stmt.query_row([], |row| row.get(0)).ok();
            if name.is_some() {
                return name;
            }
        }
    }

    None
}

pub fn list_workspaces() -> Result<Vec<Workspace>> {
    let home = dirs::home_dir().ok_or("Could not find home directory")?;
    let cursor_dir = home.join(".config/Cursor/User/workspaceStorage");

    if !cursor_dir.exists() {
        return Err("Cursor directory not found".into());
    }

    let mut workspaces = Vec::new();
    for entry in fs::read_dir(&cursor_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let id = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            // Skip special directories
            if id == "ext-dev" || id == "images" {
                continue;
            }

            // Skip numeric-only directories (temporary workspaces)
            if id.chars().all(|c| c.is_numeric()) {
                continue;
            }

            let project_name = extract_project_name(&path);

            workspaces.push(Workspace {
                id: id.clone(),
                name: id,
                project_name,
                path,
            });
        }
    }

    // Sort workspaces by project name or id
    workspaces.sort_by(|a, b| {
        let a_name = a.project_name.as_ref().unwrap_or(&a.id);
        let b_name = b.project_name.as_ref().unwrap_or(&b.id);
        a_name.cmp(b_name)
    });

    Ok(workspaces)
} 