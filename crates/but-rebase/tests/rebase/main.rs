use crate::utils::{assure_stable_env, four_commits_writable};
use anyhow::Result;
use but_rebase::{RebaseBuilder, RebaseStep};

mod error_handling {
    use crate::utils::{fixture, four_commits};
    use but_rebase::{RebaseBuilder, RebaseStep};
    use gix::ObjectId;
    use std::str::FromStr;

    fn non_existing_commit() -> gix::ObjectId {
        ObjectId::from_str("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap()
    }

    #[test]
    fn base_non_existing() -> anyhow::Result<()> {
        let result = RebaseBuilder::new(fixture("four-commits")?, non_existing_commit());
        assert_eq!(
            result.unwrap_err().to_string(),
            "An object with id eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee could not be found"
        );
        Ok(())
    }

    #[test]
    fn non_existing_commit_in_pick_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Pick {
            commit_id: non_existing_commit(),
            new_message: None,
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "An object with id eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee could not be found"
        );
        Ok(())
    }

    #[test]
    fn non_existing_commit_in_merge_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Merge {
            commit_id: non_existing_commit(),
            new_message: "merge commit".into(),
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "An object with id eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee could not be found"
        );
        Ok(())
    }

    #[test]
    fn non_existing_commit_in_fixup_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Fixup {
            commit_id: non_existing_commit(),
            new_message: None,
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "An object with id eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee could not be found"
        );
        Ok(())
    }

    #[test]
    fn using_base_in_pick_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Fixup {
            commit_id: commits.base,
            new_message: None,
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Fixup commit cannot be the base commit"
        );
        Ok(())
    }

    #[test]
    fn using_base_in_merge_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Merge {
            commit_id: commits.base,
            new_message: "merge commit".into(),
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Merge commit cannot be the base commit"
        );
        Ok(())
    }

    #[test]
    fn using_base_in_fixup_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Fixup {
            commit_id: commits.base,
            new_message: None,
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Fixup commit cannot be the base commit"
        );
        Ok(())
    }

    #[test]
    fn using_picked_commit_in_a_pick_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_merged_commit_in_a_pick_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Merge {
                commit_id: commits.a,
                new_message: "merge commit".into(),
            })?
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_fixup_commit_in_a_pick_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.b,
                new_message: None,
            })?
            .step(RebaseStep::Pick {
                commit_id: commits.b,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_picked_commit_in_a_merge_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Merge {
                commit_id: commits.a,
                new_message: "merge commit".into(),
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_merged_commit_in_a_merge_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Merge {
                commit_id: commits.a,
                new_message: "merge commit".into(),
            })?
            .step(RebaseStep::Merge {
                commit_id: commits.a,
                new_message: "merge commit".into(),
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_fixup_commit_in_a_merge_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.b,
                new_message: None,
            })?
            .step(RebaseStep::Merge {
                commit_id: commits.b,
                new_message: "merge commit".into(),
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_picked_commit_in_a_fixup_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.a,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_merged_commit_in_a_fixup_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Merge {
                commit_id: commits.a,
                new_message: "merge commit".into(),
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.a,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn using_fixup_commit_in_a_fixup_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Pick {
                commit_id: commits.a,
                new_message: None,
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.b,
                new_message: None,
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.b,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Picked commit already exists in a previous step"
        );
        Ok(())
    }

    #[test]
    fn fixup_is_first_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Fixup {
            commit_id: commits.a,
            new_message: None,
        });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Fixup must have a commit to work on"
        );
        Ok(())
    }

    #[test]
    fn fixup_is_only_preceeded_by_a_reference_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder
            .step(RebaseStep::Reference {
                name: "foo/bar".into(),
            })?
            .step(RebaseStep::Fixup {
                commit_id: commits.a,
                new_message: None,
            });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Fixup commit must not come after a reference step"
        );
        Ok(())
    }

    #[test]
    fn empty_reference_step() -> anyhow::Result<()> {
        let (repo, commits) = four_commits()?;
        let mut builder = RebaseBuilder::new(repo, commits.base)?;
        let result = builder.step(RebaseStep::Reference { name: "".into() });
        assert_eq!(
            result.unwrap_err().to_string(),
            "Reference step must have a non-empty name"
        );
        Ok(())
    }
}

#[test]
fn single_stack_journey() -> Result<()> {
    // TODO: make implement cherry-rebase in `gitoxide` to be able to use in-memory repos.
    assure_stable_env();
    let (repo, commits, _tmp) = four_commits_writable()?;
    let mut builder = RebaseBuilder::new(repo, commits.base)?;
    let out = builder
        .step(RebaseStep::Pick {
            commit_id: commits.a,
            new_message: Some("first step: pick a".into()),
        })?
        .step(RebaseStep::Fixup {
            commit_id: commits.b,
            new_message: Some("second step: squash b into a".into()),
        })?
        .step(RebaseStep::Reference {
            name: "anchor".into(),
        })?
        .step(RebaseStep::Merge {
            commit_id: commits.c,
            new_message: "third step: merge C into b".into(),
        })?
        .rebase()?;
    // TODO: get git-log output with just the right amount of info into insta (could use stable hashes)
    // M─┐ third step: merge C into b
    // │ o [main] c
    // │ o b
    // │ o a
    // o │ second step: squash b into a
    // I─┘ base
    dbg!(_tmp.into_path());
    insta::assert_debug_snapshot!(out, @r#"
    RebaseOutput {
        top_commit: Sha1(2e89cda20aa24cf27d947ade0858df7aab48cdf6),
        references: [
            ReferenceSpec {
                refname: "anchor",
                commit_id: Sha1(caf2eb225788ceb3f3ad8fd9866af40719a88dac),
                previous_commit_id: Sha1(a96434e2505c2ea0896cf4f58fec0778e074d3da),
            },
        ],
    }
    "#);
    // TODO: make more assertions on each commit
    Ok(())
}

pub mod utils {
    use anyhow::Result;
    use gix::ObjectId;

    /// Returns a fixture that may not be written to, objects will never touch disk either.
    pub fn fixture(fixture_name: &str) -> Result<gix::Repository> {
        let root = gix_testtools::scripted_fixture_read_only("rebase.sh")
            .map_err(anyhow::Error::from_boxed)?;
        let worktree_root = root.join(fixture_name);
        let repo =
            gix::open_opts(worktree_root, gix::open::Options::isolated())?.with_object_memory();
        Ok(repo)
    }

    #[derive(Debug)]
    pub struct Commits {
        pub base: ObjectId,
        pub a: ObjectId,
        pub b: ObjectId,
        pub c: ObjectId,
    }

    /// The commits in the fixture repo, starting from the oldest
    pub fn four_commits() -> Result<(gix::Repository, Commits)> {
        let repo = fixture("four-commits")?;
        let commits: Vec<_> = repo
            .head_id()?
            .ancestors()
            .all()?
            .map(Result::unwrap)
            .map(|info| info.id)
            .collect();
        assert_eq!(commits.len(), 4, "expecting a particular graph");
        Ok((
            repo,
            Commits {
                base: commits[3],
                a: commits[2],
                b: commits[1],
                c: commits[0],
            },
        ))
    }

    /// TODO: remove the need for this, impl everything in `gitoxide`.
    pub fn four_commits_writable() -> Result<(gix::Repository, Commits, tempfile::TempDir)> {
        let tmp = gix_testtools::scripted_fixture_writable("rebase.sh")
            .map_err(anyhow::Error::from_boxed)?;
        let worktree_root = tmp.path().join("four-commits");
        let repo = gix::open_opts(worktree_root, gix::open::Options::isolated())?;
        let commits: Vec<_> = repo
            .head_id()?
            .ancestors()
            .all()?
            .map(Result::unwrap)
            .map(|info| info.id)
            .collect();
        assert_eq!(commits.len(), 4, "expecting a particular graph");
        Ok((
            repo,
            Commits {
                base: commits[3],
                a: commits[2],
                b: commits[1],
                c: commits[0],
            },
            tmp,
        ))
    }

    /// Sets and environment that assures commits are reproducible.
    /// This needs the `testing` feature enabled in `but-core` as well to work.
    /// This changes the process environment, be aware.
    pub fn assure_stable_env() {
        let env = gix_testtools::Env::new()
            .set("GIT_AUTHOR_DATE", "2000-01-01 00:00:00 +0000")
            .set("GIT_AUTHOR_EMAIL", "author@example.com")
            .set("GIT_AUTHOR_NAME", "author")
            .set("GIT_COMMITTER_DATE", "2000-01-02 00:00:00 +0000")
            .set("GIT_COMMITTER_EMAIL", "committer@example.com")
            .set("GIT_COMMITTER_NAME", "committer")
            .set("CHANGE_ID", "committer");
        // assure it doesn't get racy.
        std::mem::forget(env);
    }
}
