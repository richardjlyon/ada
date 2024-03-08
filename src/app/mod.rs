pub mod commands;
pub mod error;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "zot2llm",
    about = "A tool for managing workspaces and documents in zot2llm."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage workspaces
    Workspace {
        #[clap(subcommand)]
        command: WorkspaceCommands,
    },
    /// Manage documents
    Document {
        #[clap(subcommand)]
        command: DocumentCommands,
    },
    /// Manage Zotero collections
    Zotero {
        #[clap(subcommand)]
        command: ZoteroCommands,
    },
}

#[derive(Subcommand)]
pub enum WorkspaceCommands {
    /// Create a new workspace
    Create {
        /// Name of the workspace to create
        workspace_name: String,
    },
    /// Delete an existing workspace
    Delete {
        /// id of the workspace to delete (use 'list' to get the id)
        workspace_id: u8,
    },
    /// List all workspaces
    List,
}

#[derive(Subcommand)]
pub enum DocumentCommands {
    /// Add a new document
    Add {
        /// File path of the document
        document_filepath: String,
        /// ID of the workspace (use: 'workspace list')
        workspace_id: u8,
    },
    /// List all documents
    List,
}

#[derive(Subcommand)]
pub enum ZoteroCommands {
    /// Add a new collection
    Add {
        /// ID of the collection to add (list to get the id)
        collection_id: u8,
    },
    /// List all collections
    List,
}
