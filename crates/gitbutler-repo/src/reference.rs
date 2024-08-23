use std::collections::HashMap;
use std::str::FromStr;

use crate::credentials::Helper;
use crate::{LogUntil, RepoActionsExt};
use anyhow::Context;
use anyhow::{anyhow, Result};
use gitbutler_branch::BranchReference;
use gitbutler_branch::VirtualBranchesHandle;
use gitbutler_branch::{Branch, BranchId};
use gitbutler_command_context::CommandContext;
use gitbutler_commit::commit_ext::CommitExt;
use gitbutler_reference::ReferenceName;
use itertools::Itertools;

/// Given a branch id, returns the the GitButler references associated with the branch.
/// References within the same branch effectively represent a stack of sub-branches.
pub fn list_branch_references(
    ctx: &CommandContext,
    branch_id: BranchId,
) -> Result<Vec<BranchReference>> {
    let handle = VirtualBranchesHandle::new(ctx.project().gb_dir());
    let vbranch = handle.get_branch(branch_id)?;
    Ok(vbranch.references)
}

/// Given a list of commits ids, returns a map of commit ids to the references that point to them or None
pub fn list_commit_references(
    ctx: &CommandContext,
    commits: Vec<git2::Oid>,
) -> Result<HashMap<git2::Oid, Option<BranchReference>>> {
    let handle = VirtualBranchesHandle::new(ctx.project().gb_dir());
    let all_references = handle
        .list_all_branches()?
        .into_iter()
        .flat_map(|branch| branch.references)
        .collect_vec();
    Ok(commits
        .into_iter()
        .map(|commit_id| {
            let reference = all_references
                .iter()
                .find(|r| r.commit_id == commit_id)
                .cloned();
            (commit_id, reference)
        })
        .collect())
}

/// Creates a new virtual branch reference and associates it with the branch.
/// However this will return an error if:
///   - a reference for the same commit already exists, an error is returned.
///   - the reference name already exists, an error is returned.
pub fn create_branch_reference(
    ctx: &CommandContext,
    branch_id: BranchId,
    upstream: ReferenceName,
    commit_id: git2::Oid,
) -> Result<BranchReference> {
    // The reference must be parseable as a remote reference
    gitbutler_reference::RemoteRefname::from_str(&upstream)
        .context("Failed to parse the provided reference")?;
    let handle = VirtualBranchesHandle::new(ctx.project().gb_dir());

    // The branch must exist
    let mut vbranch = handle.get_branch(branch_id)?;

    // Enusre that the commit acutally exists
    let commit = ctx
        .repository()
        .find_commit(commit_id)
        .context(anyhow!("Commit {} does not exist", commit_id))?;

    let branch_reference = BranchReference {
        upstream,
        branch_id,
        commit_id,
        change_id: commit.change_id(),
    };
    let all_references = handle
        .list_all_branches()?
        .into_iter()
        .flat_map(|branch| branch.references)
        .collect_vec();
    // Ensure the reference name does not already exist
    if all_references
        .iter()
        .any(|r| r.upstream == branch_reference.upstream)
    {
        return Err(anyhow!(
            "A reference {} already exists",
            branch_reference.upstream
        ));
    }
    // Ensure the commit is not already referenced
    if all_references.iter().any(|r| r.commit_id == commit_id) {
        return Err(anyhow!(
            "A reference for commit {} already exists",
            commit_id
        ));
    }
    validate_commit(&vbranch, commit_id, ctx, &handle)?;
    vbranch.references.push(branch_reference.clone());
    handle.set_branch(vbranch)?;
    Ok(branch_reference)
}

/// Updates an existing branch reference to point to a different commit.
/// Only the commit and change_id can be updated.
/// The reference is identified by the branch id and the reference name.
/// This function will return an error if:
/// - this reference does not exist
/// - the reference exists, but the commit id is not in the branch
/// - the reference exists, but the commit id is already associated with another reference
///
/// If the commit ID is the same as the current commit ID, the function is a no-op.
pub fn update_branch_reference(
    ctx: &CommandContext,
    branch_id: BranchId,
    upstream: ReferenceName,
    new_commit_id: git2::Oid,
) -> Result<BranchReference> {
    // The reference must be parseable as a remote reference
    gitbutler_reference::RemoteRefname::from_str(&upstream)
        .context("Failed to parse the provided reference")?;
    let handle = VirtualBranchesHandle::new(ctx.project().gb_dir());
    // The branch must exist
    let mut vbranch = handle.get_branch(branch_id)?;

    // Enusre that the commit acutally exists
    let new_commit = ctx
        .repository()
        .find_commit(new_commit_id)
        .context(anyhow!("Commit {} does not exist", new_commit_id))?;

    // Fail early if the commit is not valid
    validate_commit(&vbranch, new_commit_id, ctx, &handle)?;

    let reference = vbranch
        .references
        .iter_mut()
        .find(|r| r.upstream == upstream)
        .ok_or(anyhow!(
            "Reference {} not found for branch {}",
            upstream,
            branch_id
        ))?;
    reference.commit_id = new_commit_id;
    reference.change_id = new_commit.change_id();
    let new_reference = reference.clone();
    handle.set_branch(vbranch)?;
    Ok(new_reference)
}

/// Pushes a gitbutler branch reference to the remote repository.
pub fn push_branch_reference(
    ctx: &CommandContext,
    branch_id: BranchId,
    upstream: ReferenceName,
    with_force: bool,
    credentials: &Helper,
) -> Result<()> {
    let handle = VirtualBranchesHandle::new(ctx.project().gb_dir());
    let vbranch = handle.get_branch(branch_id)?;
    let reference = vbranch
        .references
        .iter()
        .find(|r| r.upstream == upstream)
        .ok_or_else(|| anyhow!("Reference {} not found", upstream))?;
    let upstream_refname = gitbutler_reference::RemoteRefname::from_str(&reference.upstream)
        .context("Failed to parse the provided reference")?;
    ctx.push(
        &reference.commit_id,
        &upstream_refname,
        with_force,
        credentials,
        None,
        Some(Some(branch_id)),
    )
}

/// Validates a commit in the following ways:
/// - The reference does not already exists for any other branch
/// - There is no other reference already pointing to the commit
/// - The commit is between the branch base and the branch head
fn validate_commit(
    vbranch: &Branch,
    commit_id: git2::Oid,
    ctx: &CommandContext,
    handle: &VirtualBranchesHandle,
) -> Result<()> {
    let target = handle.get_default_target()?;
    let branch_commits = ctx
        .log(vbranch.head, LogUntil::Commit(target.sha))?
        .iter()
        .map(|c| c.id())
        .collect_vec();

    // Assert that the commit is between the branch base and the branch head
    if !branch_commits.contains(&commit_id) {
        return Err(anyhow!(
            "The commit {} is not between the branch base and the branch head",
            commit_id
        ));
    }
    Ok(())
}
