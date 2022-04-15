use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct Executor {
    active_workflows: Vec<ExecutionContext>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionContext {
    operation_name: String,
    serialized_global_state: Option<String>,
    operation_args: Option<String>,
    current_state: Option<String>,
}

impl<'a> ExecutionContext {
    pub fn new(serialized_execution_context: String) -> Self {
        let exec_ctx: ExecutionContext =
            serde_json::from_str(&serialized_execution_context).unwrap();
        exec_ctx
    }

    /// If only operation_name is present, it means we need to initialize it, we need args.
    pub fn execute(&mut self) {
        // we need to define how does execution context look like and what to do when we see it
        // executor will deserialize these things and get the execution context, and then execute it.
        // here we need dynamic dispatch kind of system, not a bunch of if else's
    }
}
