//! # `SwissArmyHammer`
//!
//! A flexible prompt management library for AI assistants.
//!
//! ## Features
//!
//! - **Prompt Management**: Load, store, and organize prompts from various sources
//! - **Template Engine**: Powerful Liquid-based template processing
//! - **Search**: Full-text search capabilities for finding prompts
//! - **MCP Support**: Model Context Protocol server integration
//! - **Async/Sync APIs**: Choose between async and sync interfaces
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use swissarmyhammer::PromptLibrary;
//! use std::collections::HashMap;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a new prompt library
//! let mut library = PromptLibrary::new();
//!
//! // Add prompts from a directory
//! if std::path::Path::new("./.swissarmyhammer/prompts").exists() {
//!     library.add_directory("./.swissarmyhammer/prompts")?;
//! }
//!
//! // Get a prompt and render it
//! let prompt = library.get("code-review")?;
//! let mut args = HashMap::new();
//! args.insert("language".to_string(), "rust".to_string());
//! args.insert("file".to_string(), "main.rs".to_string());
//! let rendered = prompt.render(&args)?;
//!
//! println!("{}", rendered);
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]

/// Prompt management and storage
pub mod prompts;

/// Prompt filtering functionality
pub mod prompt_filter;

/// Prompt loading and resolution
pub mod prompt_resolver;

/// Template engine and rendering
pub mod template;

/// Model Context Protocol (MCP) server support
pub mod mcp;

/// Storage abstractions and implementations
pub mod storage;

/// Search functionality
pub mod search;

/// Advanced search functionality
pub mod search_advanced;

/// Plugin system for extensibility
pub mod plugins;

/// Workflow system for state-based execution
pub mod workflow;

/// Issue tracking and management
pub mod issues;

/// Memoranda management and storage system
pub mod memoranda;

/// Git operations for issue management
pub mod git;

/// Security utilities for path validation and resource limits
pub mod security;

/// File watching functionality for prompt directories
pub mod file_watcher;

/// Virtual file system for unified file loading
pub mod file_loader;

/// Directory traversal utilities
pub mod directory_utils;

/// Unified file system utilities for better error handling and testing
pub mod fs_utils;

/// Validation framework for checking content integrity
pub mod validation;

// Re-export core types

/// File source for loading prompts from various sources
pub use file_loader::FileSource;

/// File system utilities and abstractions
pub use fs_utils::{FileSystem, FileSystemUtils};

/// Plugin system types for extending functionality
pub use plugins::{CustomLiquidFilter, PluginRegistry, SwissArmyHammerPlugin};

/// Prompt filtering and search functionality
pub use prompt_filter::PromptFilter;

/// Advanced prompt loading and resolution
pub use prompt_resolver::PromptResolver;

/// Backward compatibility alias for FileSource
pub use file_loader::FileSource as PromptSource;

/// Core prompt management types and functionality
pub use prompts::{ArgumentSpec, Prompt, PromptLibrary, PromptLoader};

/// Storage backends and abstractions
pub use storage::{PromptStorage, StorageBackend};

/// Template engine and rendering functionality
pub use template::{Template, TemplateEngine};

/// Workflow system for state-based execution
pub use workflow::{
    State, StateId, Transition, Workflow, WorkflowName, WorkflowRun, WorkflowRunId,
    WorkflowRunStatus,
};

/// Memoranda (memo/note) management types
pub use memoranda::{
    CreateMemoRequest, DeleteMemoRequest, GetMemoRequest, ListMemosResponse, Memo, MemoId,
    SearchMemosRequest, SearchMemosResponse, UpdateMemoRequest,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Error types used throughout the library
pub mod error;

/// Configuration management
pub mod config;

pub use config::Config;
pub use error::{ErrorChainExt, ErrorContext, Result, SwissArmyHammerError};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        CustomLiquidFilter, FileSystem, FileSystemUtils, PluginRegistry, Prompt, PromptLibrary,
        PromptLoader, PromptStorage, Result, StorageBackend, SwissArmyHammerError,
        SwissArmyHammerPlugin, Template, TemplateEngine,
    };

    pub use crate::mcp::McpServer;
    pub use crate::search::{SearchEngine, SearchResult};
    pub use crate::search_advanced::{
        generate_excerpt, AdvancedSearchEngine, AdvancedSearchOptions, AdvancedSearchResult,
    };
    pub use crate::workflow::{
        State, StateId, Transition, Workflow, WorkflowName, WorkflowRun, WorkflowRunId,
        WorkflowRunStatus,
    };

    // Memoranda types for convenient access
    pub use crate::memoranda::{
        CreateMemoRequest, DeleteMemoRequest, GetMemoRequest, ListMemosResponse, Memo, MemoId,
        SearchMemosRequest, SearchMemosResponse, UpdateMemoRequest,
    };

    // Common utilities for easy access
    pub use crate::common::{
        env_loader::EnvLoader,
        error_context::IoResultExt,
        file_types::{is_prompt_file, ExtensionMatcher},
        mcp_errors::{McpResultExt, ToSwissArmyHammerError},
        validation_builders::{quick, ValidationChain, ValidationErrorBuilder},
    };
}

/// Test utilities module for testing support
pub mod test_utils;

/// Common utilities module for code reuse
pub mod common;
