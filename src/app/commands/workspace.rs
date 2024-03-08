//! Workspace commands
//!
use colored::*;

use crate::app::commands;
use crate::app::error::AppError;
use crate::app::error::AppError::WorkspaceIdError;

/// List all workspaces
pub async fn workspace_list() -> Result<(), AppError> {
    println!("{}", "Listing all workspaces".green());
    let client = commands::anythingllm_client();
    let workspaces = client.workspace_list().await.unwrap();
    for ws in workspaces {
        println!("{:>2}: {} ({})", ws.id, ws.name, ws.slug);
    }

    Ok(())
}

/// Create a new workspace
pub async fn workspace_create(workspace_name: &str) -> Result<(), AppError> {
    let client = commands::anythingllm_client();
    client.workspace_create(workspace_name).await?;

    println!("Added workspace '{}'", workspace_name);

    Ok(())
}

/// Delete a workspace
pub async fn workspace_delete(workspace_id: u8) -> Result<(), AppError> {
    let client = commands::anythingllm_client();
    let workspace_slug = client
        .workspace_list()
        .await?
        .iter()
        .find(|ws| ws.id == workspace_id)
        .ok_or(WorkspaceIdError(workspace_id))?
        .slug
        .clone();

    client.workspace_delete(&workspace_slug).await?;

    println!(
        "{} '{}'",
        "Removed workspace".green(),
        workspace_id.to_string().bold()
    );

    Ok(())
}
