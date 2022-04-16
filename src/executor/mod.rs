use serde::{Deserialize, Serialize};
use serde_json::Result;

pub struct Executor {
    active_workflows: Vec<ExecutionContext>,
    /// TODO: Not use string here. Use proper types to validate things here. Maybe a base type
    registered_operations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ExecutionContext {
    Initialize {
        operation_name: String,
        serialized_operation_args: Option<String>,
    },
    Execute {
        operation_name: String, // I wonder if we can validate this state with the struct we have in operations.
        serialized_operation_args: Option<String>,
        serialized_operation_global_state: Option<String>,
        next_state_to_execute: Option<String>, // I wonder if we can validate this state with the enum we have in operations.
    },
}

impl ExecutionContext {
    pub fn new(serialized_execution_context: String, executor: &Executor) -> Self {
        let exec_ctx: ExecutionContext =
            serde_json::from_str(&serialized_execution_context).unwrap();

        // We should do validation here. But all validation data is operation specific which we don't have access to here.
        // Plus, we will be writing a bunch of operations, it doesn't makes sense to keep adding if else branches here to do validation.
        // We should have a kind of searcher in the codebase which does all this setup for us.
        // All of this data should become available to executor.
        // Same goes for execute. we should be able to generate proper struct types out of Global state and execute the required transition function directly.
        let result = match &exec_ctx {
            ExecutionContext::Initialize {
                operation_name,
                serialized_operation_args,
            } => {
                assert!(
                    !operation_name.is_empty(),
                    "Operation name supplied was empty"
                );
                assert!(
                    executor.registered_operations.contains(&operation_name),
                    "Operation not registered"
                );
                // ensure that serialized_operation_args is able to be de-serialized to proper struct
                // thing is, we don't have access to those structs here. All of them are inside the particular operation.
            }
            ExecutionContext::Execute {
                operation_name,
                serialized_operation_args,
                serialized_operation_global_state,
                next_state_to_execute,
            } => {
                todo!()
            }
        };
        exec_ctx
    }

    /// If only operation_name is present, it means we need to initialize it, we need args.
    pub fn execute_once(&mut self) {
        // we need to define how does execution context look like and what to do when we see it
        // executor will deserialize these things and get the execution context, and then execute it.
        // here we need dynamic dispatch kind of system, not a bunch of if else's. Hopefully I will be able to find them.

        // just do what we have to do. all validation is complete before we created it.
        match self {
            ExecutionContext::Initialize {
                operation_name,
                serialized_operation_args,
            } => {}
            ExecutionContext::Execute {
                operation_name,
                serialized_operation_args,
                serialized_operation_global_state,
                next_state_to_execute,
            } => {
                todo!()
            }
        }
    }
}
