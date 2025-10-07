use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TelemetrySettings {
    /// Whether the anonymous metrics are enabled.
    pub app_metrics_enabled: bool,
    /// Whether anonymous error reporting is enabled.
    pub app_error_reporting_enabled: bool,
    /// Whether non-anonymous metrics are enabled.
    pub app_non_anon_metrics_enabled: bool,
    /// Distinct ID, if reporting is enabled.
    pub app_distinct_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitHubOAuthAppSettings {
    /// Client ID for the GitHub OAuth application. Set this to use custom (non-GitButler) OAuth application.
    pub oauth_client_id: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeatureFlags {
    /// Enable the usage of V3 workspace APIs.
    pub ws3: bool,
    /// Turn on the set a v3 version of checkout
    pub cv3: bool,
    /// Use the V3 version of apply and unapply.
    pub apply3: bool,
    /// Enable undo/redo support.
    ///
    /// ### Progression for implementation
    ///
    /// * use snapshot system in undo/redo queue
    ///     - consider not referring to these objects by reference to `git gc` will catch them,
    ///       or even purge them on shutdown. Alternatively, keep them in-memory with in-memory objects.
    /// * add user-control to snapshot system to purge now, or purge after time X. That way data isn't stored forever.
    /// * Finally, consider implementing undo/redo with invasive primitives that are undoable/redoable themselves for
    ///   the most efficient solution, inherently in memory, i.e.
    ///     - CRUD reference
    ///     - CRUD metadata
    ///     - CRUD workspace
    ///     - CRUD files
    pub undo: bool,
    /// Enable the usage of GitButler Acitions.
    pub actions: bool,
    /// Enable the usage of the butbot chat.
    pub butbot: bool,
    /// Enable processing of workspace rules.
    pub rules: bool,
    /// Enable single branch mode.
    pub single_branch: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExtraCsp {
    /// Additional hosts that the application can connect to.
    pub hosts: Vec<String>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Fetch {
    /// The frequency at which the app will automatically fetch. A negative value (e.g. -1) disables auto fetching.
    pub auto_fetch_interval_minutes: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Claude {
    /// Path to the Claude Code executable. Defaults to "claude" if not set.
    pub executable: String,
    /// Whether to show notifications when Claude Code finishes.
    pub notify_on_completion: bool,
    /// Whether to show notifications when Claude Code needs permission.
    pub notify_on_permission_request: bool,
    /// Whether to dangerously allow all permissions without prompting.
    pub dangerously_allow_all_permissions: bool,
    /// Whether to automatically commit changes and rename branches after completion.
    pub auto_commit_after_completion: bool,
    /// Whether to use the configured model in .claude/settings.json instead of passing --model.
    pub use_configured_model: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Reviews {
    /// Whether to auto-fill PR title and description from the first commit when a branch has only one commit.
    pub auto_fill_pr_description_from_commit: bool,
}
