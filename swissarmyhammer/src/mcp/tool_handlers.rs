//! Tool handlers for MCP operations

use super::types::{ISSUE_BRANCH_PREFIX, ISSUE_NUMBER_WIDTH};
use super::responses::{
    create_all_complete_response, create_error_response, create_issue_response,
    create_mark_complete_response, create_success_response,
};
use super::types::*;
use super::utils::validate_issue_name;
use crate::git::GitOperations;
use crate::issues::IssueStorage;
use crate::Result;
use rmcp::model::*;
use rmcp::Error as McpError;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Tool handlers for MCP server operations
pub struct ToolHandlers {
    issue_storage: Arc<RwLock<Box<dyn IssueStorage>>>,
    git_ops: Arc<Mutex<Option<GitOperations>>>,
}

impl ToolHandlers {
    /// Create a new tool handlers instance with the given issue storage and git operations
    pub fn new(
        issue_storage: Arc<RwLock<Box<dyn IssueStorage>>>,
        git_ops: Arc<Mutex<Option<GitOperations>>>,
    ) -> Self {
        Self {
            issue_storage,
            git_ops,
        }
    }

    /// Handle the issue_create tool operation.
    ///
    /// Creates a new issue with auto-assigned number and stores it in the
    /// issues directory as a markdown file.
    ///
    /// # Arguments
    ///
    /// * `request` - The create issue request containing name and content
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result
    pub async fn handle_issue_create(
        &self,
        request: CreateIssueRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        tracing::debug!("Creating issue: {}", request.name);

        // Validate issue name using shared validation logic
        let validated_name = validate_issue_name(request.name.as_ref())?;

        let issue_storage = self.issue_storage.write().await;
        match issue_storage
            .create_issue(validated_name, request.content)
            .await
        {
            Ok(issue) => {
                tracing::info!("Created issue {} with number {}", issue.name, issue.number);
                Ok(create_issue_response(&issue))
            }
            Err(crate::SwissArmyHammerError::IssueAlreadyExists(num)) => {
                tracing::warn!("Issue #{:06} already exists", num);
                Err(McpError::invalid_params(
                    format!("Issue #{:06} already exists", num),
                    None,
                ))
            }
            Err(e) => {
                tracing::error!("Failed to create issue: {}", e);
                Err(McpError::internal_error(
                    format!("Failed to create issue: {}", e),
                    None,
                ))
            }
        }
    }

    /// Handle the issue_mark_complete tool operation.
    ///
    /// Marks an issue as complete by moving it to the completed issues directory.
    ///
    /// # Arguments
    ///
    /// * `request` - The mark complete request containing the issue number
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result
    pub async fn handle_issue_mark_complete(
        &self,
        request: MarkCompleteRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        let issue_storage = self.issue_storage.write().await;
        match issue_storage.mark_complete(request.number.into()).await {
            Ok(issue) => Ok(create_mark_complete_response(&issue)),
            Err(crate::SwissArmyHammerError::IssueNotFound(num)) => Err(McpError::invalid_params(
                format!("Issue #{:06} not found", num),
                None,
            )),
            Err(e) => Err(McpError::internal_error(
                format!("Failed to mark issue complete: {}", e),
                None,
            )),
        }
    }

    /// Handle the issue_all_complete tool operation.
    ///
    /// Checks if all issues are completed by listing pending issues.
    ///
    /// # Arguments
    ///
    /// * `_request` - The all complete request (no parameters needed)
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result with completion status
    pub async fn handle_issue_all_complete(
        &self,
        _request: AllCompleteRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        let issue_storage = self.issue_storage.read().await;
        match issue_storage.list_issues().await {
            Ok(issues) => {
                let pending_count = issues.iter().filter(|i| !i.completed).count();
                Ok(create_all_complete_response(issues.len(), pending_count))
            }
            Err(e) => Err(McpError::internal_error(
                format!("Failed to check issues: {}", e),
                None,
            )),
        }
    }

    /// Handle the issue_update tool operation.
    ///
    /// Updates the content of an existing issue with new markdown content.
    ///
    /// # Arguments
    ///
    /// * `request` - The update request containing issue number and new content
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result
    pub async fn handle_issue_update(
        &self,
        request: UpdateIssueRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        let issue_storage = self.issue_storage.write().await;
        match issue_storage
            .update_issue(request.number.into(), request.content)
            .await
        {
            Ok(issue) => Ok(create_success_response(format!(
                "Updated issue {} ({})",
                issue.number, issue.name
            ))),
            Err(e) => Ok(create_error_response(format!(
                "Failed to update issue: {}",
                e
            ))),
        }
    }

    /// Handle the issue_current tool operation.
    ///
    /// Determines the current issue being worked on by checking the git branch name.
    /// If on an issue branch (starts with 'issue/'), returns the issue name.
    ///
    /// # Arguments
    ///
    /// * `_request` - The current issue request (no parameters needed)
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result with current issue info
    pub async fn handle_issue_current(
        &self,
        _request: CurrentIssueRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        let git_ops = self.git_ops.lock().await;
        match git_ops.as_ref() {
            Some(ops) => match ops.current_branch() {
                Ok(branch) => {
                    if let Some(issue_name) = branch.strip_prefix(ISSUE_BRANCH_PREFIX) {
                        Ok(create_success_response(format!(
                            "Currently working on issue: {}",
                            issue_name
                        )))
                    } else {
                        Ok(create_success_response(format!(
                            "Not on an issue branch. Current branch: {}",
                            branch
                        )))
                    }
                }
                Err(e) => Ok(create_error_response(format!(
                    "Failed to get current branch: {}",
                    e
                ))),
            },
            None => Ok(create_error_response(
                "Git operations not available".to_string(),
            )),
        }
    }

    /// Handle the issue_work tool operation.
    ///
    /// Switches to a work branch for the specified issue. Creates a new branch
    /// with the format 'issue/{issue_number}_{issue_name}' if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `request` - The work request containing the issue number
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result
    pub async fn handle_issue_work(
        &self,
        request: WorkIssueRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        // First get the issue to determine its name
        let issue_storage = self.issue_storage.read().await;
        let issue = match issue_storage.get_issue(request.number.into()).await {
            Ok(issue) => issue,
            Err(e) => {
                return Ok(create_error_response(format!(
                    "Failed to get issue {}: {}",
                    request.number, e
                )))
            }
        };
        drop(issue_storage);

        // Create work branch
        let mut git_ops = self.git_ops.lock().await;
        let issue_name = format!(
            "{:0width$}_{}",
            issue.number,
            issue.name,
            width = ISSUE_NUMBER_WIDTH
        );

        match git_ops.as_mut() {
            Some(ops) => match ops.create_work_branch(&issue_name) {
                Ok(branch_name) => Ok(create_success_response(format!(
                    "Switched to work branch: {}",
                    branch_name
                ))),
                Err(e) => Ok(create_error_response(format!(
                    "Failed to create work branch: {}",
                    e
                ))),
            },
            None => Ok(create_error_response(
                "Git operations not available".to_string(),
            )),
        }
    }

    /// Handle the issue_merge tool operation.
    ///
    /// Merges the work branch for an issue back to the main branch.
    /// The branch name is determined from the issue number and name.
    ///
    /// # Arguments
    ///
    /// * `request` - The merge request containing the issue number
    ///
    /// # Returns
    ///
    /// * `Result<CallToolResult, McpError>` - The tool call result
    pub async fn handle_issue_merge(
        &self,
        request: MergeIssueRequest,
    ) -> std::result::Result<CallToolResult, McpError> {
        // First get the issue to determine its name
        let issue_storage = self.issue_storage.read().await;
        let issue = match issue_storage.get_issue(request.number.into()).await {
            Ok(issue) => issue,
            Err(e) => {
                return Ok(create_error_response(format!(
                    "Failed to get issue {}: {}",
                    request.number, e
                )))
            }
        };
        drop(issue_storage);

        // Validate that the issue is completed before allowing merge
        if !issue.completed {
            return Ok(create_error_response(format!(
                "Issue {} is not completed. Only completed issues can be merged.",
                request.number
            )));
        }

        // Check working directory is clean before merge
        let git_ops_guard = self.git_ops.lock().await;
        if let Some(git_ops) = git_ops_guard.as_ref() {
            if let Err(e) = self.check_working_directory_clean(git_ops).await {
                return Ok(create_error_response(format!(
                    "Working directory is not clean. Please commit or stash changes before merging: {}",
                    e
                )));
            }
        }
        drop(git_ops_guard);

        // Merge branch
        let mut git_ops = self.git_ops.lock().await;
        let issue_name = format!(
            "{:0width$}_{}",
            issue.number,
            issue.name,
            width = ISSUE_NUMBER_WIDTH
        );

        match git_ops.as_mut() {
            Some(ops) => {
                // First merge the branch
                match ops.merge_issue_branch(&issue_name) {
                    Ok(_) => {
                        let mut success_message =
                            format!("Merged work branch for issue {} to main", issue_name);

                        // Get commit information after successful merge
                        let commit_info = match ops.get_last_commit_info() {
                            Ok(info) => {
                                let parts: Vec<&str> = info.split('|').collect();
                                if parts.len() >= 4 {
                                    format!(
                                        "\n\nMerge commit: {}\nMessage: {}\nAuthor: {}\nDate: {}",
                                        &parts[0][..8], // First 8 chars of hash
                                        parts[1],
                                        parts[2],
                                        parts[3]
                                    )
                                } else {
                                    format!("\n\nMerge commit: {}", info)
                                }
                            }
                            Err(_) => String::new(),
                        };

                        // If delete_branch is true, delete the branch after successful merge
                        if request.delete_branch {
                            let branch_name = format!("issue/{}", issue_name);
                            match ops.delete_branch(&branch_name) {
                                Ok(_) => {
                                    success_message
                                        .push_str(&format!(" and deleted branch {}", branch_name));
                                }
                                Err(e) => {
                                    success_message
                                        .push_str(&format!(" but failed to delete branch: {}", e));
                                }
                            }
                        }

                        success_message.push_str(&commit_info);
                        Ok(create_success_response(success_message))
                    }
                    Err(e) => Ok(create_error_response(format!(
                        "Failed to merge branch: {}",
                        e
                    ))),
                }
            }
            None => Ok(create_error_response(
                "Git operations not available".to_string(),
            )),
        }
    }

    /// Check if working directory is clean
    async fn check_working_directory_clean(&self, _git_ops: &GitOperations) -> Result<()> {
        use std::process::Command;

        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .map_err(|e| {
                crate::SwissArmyHammerError::git_operation_failed("git status check", &e.to_string())
            })?;

        let status = String::from_utf8_lossy(&output.stdout);

        if !status.trim().is_empty() {
            return Err(crate::SwissArmyHammerError::Other(
                "Working directory is not clean - there are uncommitted changes".to_string(),
            ));
        }

        Ok(())
    }
}
