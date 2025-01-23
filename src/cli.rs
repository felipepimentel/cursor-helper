use clap::{Parser, Subcommand};
use colored::*;
use serde_json::json;

use crate::db::CursorDB;
use crate::error::Result;
use crate::models::Workspace;
use crate::workspace::list_workspaces;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage workspaces")]
    Workspace {
        #[command(subcommand)]
        command: WorkspaceCommands,
    },
    #[command(about = "Manage notepad")]
    Notepad {
        #[command(subcommand)]
        command: NotepadCommands,
    },
}

#[derive(Subcommand)]
enum WorkspaceCommands {
    #[command(about = "List all workspaces")]
    List,
}

#[derive(Subcommand)]
enum NotepadCommands {
    #[command(about = "List all notes")]
    List {
        #[arg(long, help = "Project name or workspace ID")]
        workspace: String,
    },
    #[command(about = "Add a new note")]
    Add {
        #[arg(long, help = "Project name or workspace ID")]
        workspace: String,
        #[arg(help = "Note content")]
        content: String,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Workspace { command } => handle_workspace_command(command),
        Commands::Notepad { command } => handle_notepad_command(command),
    }
}

fn handle_workspace_command(command: WorkspaceCommands) -> Result<()> {
    match command {
        WorkspaceCommands::List => {
            let workspaces = list_workspaces()?;
            println!("Available Workspaces:");
            for workspace in workspaces {
                let db = CursorDB::new(workspace.clone());
                let has_notes = db.get_notepad_data()?.is_some();
                let status = if has_notes {
                    "✓".green()
                } else {
                    "✗".red()
                };
                
                let display_name = workspace.project_name
                    .as_ref()
                    .map(|name| format!("{} ({})", name, workspace.id))
                    .unwrap_or_else(|| workspace.id.clone());
                
                println!("{} {}", status, display_name);
            }
            Ok(())
        }
    }
}

fn find_workspace<'a>(workspaces: &'a [Workspace], identifier: &str) -> Option<&'a Workspace> {
    workspaces.iter().find(|w| {
        w.id == identifier || 
        w.project_name.as_ref().map(|name| name == identifier).unwrap_or(false)
    })
}

fn handle_notepad_command(command: NotepadCommands) -> Result<()> {
    match command {
        NotepadCommands::List { workspace } => {
            let workspaces = list_workspaces()?;
            let workspace = find_workspace(&workspaces, &workspace)
                .ok_or_else(|| format!("Workspace '{}' not found", workspace))?;

            let db = CursorDB::new(workspace.clone());
            let data = db.get_notepad_data()?;

            match data {
                Some(json_str) => {
                    let value: serde_json::Value = serde_json::from_str(&json_str)?;
                    println!("{}", serde_json::to_string_pretty(&value)?);
                }
                None => println!("No notepad data available"),
            }
            Ok(())
        }
        NotepadCommands::Add { workspace, content } => {
            let workspaces = list_workspaces()?;
            let workspace = find_workspace(&workspaces, &workspace)
                .ok_or_else(|| format!("Workspace '{}' not found", workspace))?;

            let db = CursorDB::new(workspace.clone());
            let data = db.get_notepad_data()?;

            let mut value = match data {
                Some(json_str) => serde_json::from_str::<serde_json::Value>(&json_str)?,
                None => json!({
                    "notepads": {}
                }),
            };

            if !value.get("notepads").is_some() {
                value["notepads"] = json!({});
            }

            let notepads = value.get_mut("notepads").unwrap();
            if !notepads.get("tasks").is_some() {
                notepads["tasks"] = json!({
                    "list": []
                });
            }

            let tasks = notepads.get_mut("tasks").unwrap();
            let list = tasks.get_mut("list").unwrap();
            list.as_array_mut().unwrap().push(json!({
                "content": content,
                "created_at": chrono::Utc::now().to_rfc3339(),
            }));

            db.set_notepad_data(&serde_json::to_string(&value)?)?;
            println!("Note added successfully");
            Ok(())
        }
    }
} 