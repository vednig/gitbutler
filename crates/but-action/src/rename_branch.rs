use std::vec;

use async_openai::{Client, config::OpenAIConfig};
use but_workspace::StackId;
use gitbutler_command_context::CommandContext;

use crate::workflow::{self, Workflow};

pub struct RenameBranchParams {
    pub commit_id: gix::ObjectId,
    pub commit_message: String,
    pub stack_id: StackId,
    pub current_branch_name: String,
}

pub async fn rename_branch(
    ctx: &mut CommandContext,
    client: &Client<OpenAIConfig>,
    parameters: RenameBranchParams,
    trigger_id: uuid::Uuid,
) -> anyhow::Result<()> {
    let RenameBranchParams {
        commit_id,
        commit_message,
        stack_id,
        current_branch_name,
    } = parameters;

    let repo = &ctx.gix_repo_for_merging_non_persisting()?;
    let stacks = crate::stacks(ctx, repo)?;
    let existing_branch_names = stacks
        .iter()
        .flat_map(|s| s.heads.iter().map(|h| h.name.clone().to_string()))
        .collect::<Vec<_>>();
    let changes = but_core::diff::ui::commit_changes_by_worktree_dir(repo, commit_id)?;
    let diff = changes.try_as_unidiff_string(repo, ctx.app_settings().context_lines)?;
    let diffs = vec![diff];

    let commit_messages = vec![commit_message];
    let branch_name =
        crate::generate::branch_name(client, &commit_messages, &diffs, &existing_branch_names)
            .await?;
    let normalized_branch_name = gitbutler_reference::normalize_branch_name(&branch_name)?;

    let update = gitbutler_branch_actions::stack::update_branch_name(
        ctx,
        stack_id,
        current_branch_name.clone(),
        normalized_branch_name.clone(),
    )
    .map_err(|e| anyhow::anyhow!("Failed to rename branch: {}", e));

    let status = match &update {
        Ok(_) => workflow::Status::Completed,
        Err(e) => workflow::Status::Failed(e.to_string()),
    };

    Workflow::new(
        workflow::Kind::RenameBranch(workflow::RenameBranchOutcome {
            stack_id,
            old_branch_name: current_branch_name,
            new_branch_name: normalized_branch_name,
        }),
        workflow::Trigger::Snapshot(trigger_id),
        status,
        vec![],
        vec![],
        None,
    )
    .persist(ctx)
    .ok();

    Ok(())
}
