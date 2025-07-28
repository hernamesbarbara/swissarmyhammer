//! Model Context Protocol (MCP) server support
//!
//! This module provides MCP server functionality for serving prompts, workflows,
//! and various tools through the Model Context Protocol.

// Module declarations
pub mod error_handling;
pub mod file_watcher;
pub mod memo_types;
pub mod responses;
pub mod search_types;
pub mod server;
pub mod shared_utils;
pub mod tool_descriptions;
pub mod tool_handlers;
pub mod tool_registry;
pub mod tools;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export commonly used items from submodules
pub use server::McpServer;
pub use tool_handlers::ToolHandlers;
pub use tool_registry::{
    register_issue_tools, register_memo_tools, register_search_tools, ToolContext, ToolRegistry,
};
pub use types::{GetPromptRequest, ListPromptsRequest};

#[cfg(test)]
pub use types::{
    AllCompleteRequest, CreateIssueRequest, CurrentIssueRequest, IssueName, MarkCompleteRequest,
    MergeIssueRequest, UpdateIssueRequest, WorkIssueRequest,
};
#[cfg(test)]
pub use utils::validate_issue_name;