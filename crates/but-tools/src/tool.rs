use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use crate::emit::{Emittable, Emitter, ToolCall};
use but_workspace::ui::StackEntryNoOpt;
use but_workspace::{StackId, ui::StackEntry};
use gitbutler_command_context::CommandContext;
use gix::ObjectId;
use serde_json::json;

pub struct WorkspaceToolset<'a> {
    ctx: &'a mut CommandContext,
    emitter: std::sync::Arc<crate::emit::Emitter>,
    message_id: Option<String>,
    tools: BTreeMap<String, Arc<dyn Tool>>,
    commit_mapping: HashMap<ObjectId, ObjectId>,
}

pub trait Toolset {
    fn register_tool<T: Tool>(&mut self, tool: T);
    fn get(&self, name: &str) -> Option<Arc<dyn Tool>>;
    fn list(&self) -> Vec<Arc<dyn Tool>>;
    fn call_tool(&mut self, name: &str, parameters: &str) -> serde_json::Value;
}

impl<'a> WorkspaceToolset<'a> {
    pub fn new(
        ctx: &'a mut CommandContext,
        emitter: std::sync::Arc<crate::emit::Emitter>,
        message_id: Option<String>,
    ) -> Self {
        WorkspaceToolset {
            ctx,
            emitter,
            message_id,
            tools: BTreeMap::new(),
            commit_mapping: HashMap::new(),
        }
    }

    fn call_tool_inner(
        &mut self,
        name: &str,
        parameters: &str,
    ) -> anyhow::Result<serde_json::Value> {
        let tool = self
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", name))?;
        let params: serde_json::Value = serde_json::from_str(parameters)
            .map_err(|e| anyhow::anyhow!("Failed to parse parameters: {}", e))?;
        tool.call(
            params,
            self.ctx,
            self.emitter.clone(),
            &mut self.commit_mapping,
        )
    }
}

impl Toolset for WorkspaceToolset<'_> {
    fn register_tool<T: Tool>(&mut self, tool: T) {
        self.tools.insert(tool.name(), Arc::new(tool));
    }

    fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    fn list(&self) -> Vec<Arc<dyn Tool>> {
        self.tools.values().cloned().collect()
    }

    fn call_tool(&mut self, name: &str, parameters: &str) -> serde_json::Value {
        let result = self.call_tool_inner(name, parameters).unwrap_or_else(|e| {
            serde_json::json!({
                "error": format!("Failed to call tool '{}': {}", name, e.to_string())
            })
        });

        // Emit the tool call event if a message ID is provided
        if let Some(message_id) = &self.message_id {
            let project_id = self.ctx.project().id;
            let tool_call = ToolCall {
                project_id,
                message_id: message_id.to_owned(),
                name: name.to_string(),
                parameters: parameters.to_string(),
                result: result.to_string(),
            };
            let (name, payload) = tool_call.emittable();
            (self.emitter)(&name, payload);
        }

        result
    }
}

pub trait Tool: 'static + Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    fn call(
        self: Arc<Self>,
        parameters: serde_json::Value,
        ctx: &mut CommandContext,
        emitter: Arc<Emitter>,
        commit_mapping: &mut HashMap<ObjectId, ObjectId>,
    ) -> anyhow::Result<serde_json::Value>;
}

pub fn error_to_json(error: &anyhow::Error, action_identifier: &str) -> serde_json::Value {
    serde_json::json!({
        "error": format!("Failed to {}: {}", action_identifier, error.to_string())
    })
}

pub fn string_result_to_json(
    result: &Result<String, &anyhow::Error>,
    action_identifier: &str,
) -> serde_json::Value {
    match result {
        Ok(value) => json!({ "result": value }),
        Err(e) => error_to_json(e, action_identifier),
    }
}

pub fn string_vec_result_to_json(
    result: &Result<Vec<String>, &anyhow::Error>,
    action_identifier: &str,
) -> serde_json::Value {
    match result {
        Ok(values) => json!({ "result": values }),
        Err(e) => error_to_json(e, action_identifier),
    }
}

pub fn result_to_json<T: serde::Serialize>(
    result: &Result<T, anyhow::Error>,
    action_identifier: &str,
    data_identifier: &str,
) -> serde_json::Value {
    match result {
        Ok(entry) => json!({ "result": serde_json::to_value(entry).unwrap_or_else(
            |e| json!({ "error": format!("Failed to serialize {}: {}", data_identifier, e.to_string())}),
        )}),
        Err(e) => error_to_json(e, action_identifier),
    }
}

pub trait ToolResult: 'static + Send + Sync {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value;
}

impl ToolResult for Result<StackEntry, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        result_to_json(self, action_identifier, "StackEntry")
    }
}

impl ToolResult for Result<StackEntryNoOpt, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        result_to_json(self, action_identifier, "StackEntry")
    }
}

impl ToolResult for Result<but_workspace::commit_engine::ui::CreateCommitOutcome, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        result_to_json(self, action_identifier, "CreateCommitOutcome")
    }
}

impl ToolResult for Result<StackId, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        result_to_json(self, action_identifier, "StackId")
    }
}

impl ToolResult for Result<gix::ObjectId, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        let result = self.as_ref().map(|id| id.to_string());
        string_result_to_json(&result, action_identifier)
    }
}
impl ToolResult for Result<Vec<gix::ObjectId>, anyhow::Error> {
    fn to_json(&self, action_identifier: &str) -> serde_json::Value {
        let result = self
            .as_ref()
            .map(|ids| ids.iter().map(|id| id.to_string()).collect::<Vec<String>>());
        string_vec_result_to_json(&result, action_identifier)
    }
}
