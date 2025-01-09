use anyhow::Result;
use git2_hooks;
use git2_hooks::HookResult as H;
use gitbutler_command_context::CommandContext;
use serde::Serialize;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct MessageData {
    pub message: String,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct ErrorData {
    pub error: String,
}

/// Hook result indicating either success or failure.
#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum HookResult {
    Success,
    NotFound,
    Message(MessageData),
    Failure(ErrorData),
}

pub fn message(ctx: &CommandContext, mut message: String) -> Result<HookResult> {
    match git2_hooks::hooks_commit_msg(ctx.repo(), Some(&["../.husky"]), &mut message)? {
        H::Ok { hook: _ } => Ok(HookResult::Message(MessageData { message })),
        H::NoHookFound => Ok(HookResult::NotFound),
        H::RunNotSuccessful { stdout, stderr, .. } => {
            let error = join_output(stdout, stderr);
            Ok(HookResult::Failure(ErrorData { error }))
        }
    }
}

pub fn pre_commit(ctx: &CommandContext) -> Result<HookResult> {
    match git2_hooks::hooks_pre_commit(ctx.repo(), Some(&["../.husky"]))? {
        H::Ok { hook: _ } => Ok(HookResult::Success),
        H::NoHookFound => Ok(HookResult::NotFound),
        H::RunNotSuccessful { stdout, stderr, .. } => {
            let error = join_output(stdout, stderr);
            Ok(HookResult::Failure(ErrorData { error }))
        }
    }
}

pub fn post_commit(ctx: &CommandContext) -> Result<HookResult> {
    match git2_hooks::hooks_post_commit(ctx.repo(), Some(&["../.husky"]))? {
        H::Ok { hook: _ } => Ok(HookResult::Success),
        H::NoHookFound => Ok(HookResult::NotFound),
        H::RunNotSuccessful { stdout, stderr, .. } => {
            let error = join_output(stdout, stderr);
            Ok(HookResult::Failure(ErrorData { error }))
        }
    }
}

fn join_output(stdout: String, stderr: String) -> String {
    if stdout.is_empty() && stderr.is_ascii() {
        return "hook produced no output".to_owned();
    } else if stdout.is_empty() {
        return stderr;
    } else if stderr.is_empty() {
        return stdout;
    }
    format!("stdout:\n{}\n\n{}", stdout, stderr)
}
