use gitbutler_repo::hooks;

#[cfg(test)]
mod tests {

    use gitbutler_testsupport::{Case, Suite};
    use hooks::{ErrorData, HookResult, MessageData};

    use super::*;

    #[test]
    fn pre_commit_hook_success() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        let hook = b"
#!/bin/sh
# do nothing
";
        git2_hooks::create_hook(ctx.repo(), git2_hooks::HOOK_PRE_COMMIT, hook);
        assert_eq!(hooks::pre_commit(ctx)?, HookResult::Success);
        Ok(())
    }

    #[test]
    fn pre_commit_hook_not_found() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        assert_eq!(hooks::pre_commit(ctx)?, HookResult::NotFound);
        Ok(())
    }

    #[test]
    fn pre_commit_hook_rejection() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        let hook = b"
#!/bin/sh
echo 'rejected'
exit 1
";
        git2_hooks::create_hook(ctx.repo(), git2_hooks::HOOK_PRE_COMMIT, hook);

        assert_eq!(
            hooks::pre_commit(ctx)?,
            HookResult::Failure(ErrorData {
                error: "rejected\n".to_owned()
            })
        );
        Ok(())
    }

    #[test]
    fn post_commit_hook_rejection() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        let hook = b"
#!/bin/sh
echo 'rejected'
exit 1
";
        git2_hooks::create_hook(ctx.repo(), git2_hooks::HOOK_POST_COMMIT, hook);

        assert_eq!(
            hooks::post_commit(ctx)?,
            HookResult::Failure(ErrorData {
                error: "rejected\n".to_owned()
            })
        );
        Ok(())
    }

    #[test]
    fn message_hook_rejection() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        let hook = b"
#!/bin/sh
echo 'rejected'
exit 1
";
        git2_hooks::create_hook(ctx.repo(), git2_hooks::HOOK_COMMIT_MSG, hook);

        let message = "commit message".to_owned();
        assert_eq!(
            hooks::message(ctx, message)?,
            HookResult::Failure(ErrorData {
                error: "rejected\n".to_owned()
            })
        );
        Ok(())
    }

    #[test]
    fn rewrite_message() -> anyhow::Result<()> {
        let suite = Suite::default();
        let Case { ctx, .. } = &suite.new_case();

        let hook = b"
#!/bin/sh
echo 'rewritten message' > $1
";
        git2_hooks::create_hook(ctx.repo(), git2_hooks::HOOK_COMMIT_MSG, hook);

        let message = "commit message".to_owned();
        assert_eq!(
            hooks::message(ctx, message)?,
            HookResult::Message(MessageData {
                message: "rewritten message\n".to_owned()
            })
        );
        Ok(())
    }
}
