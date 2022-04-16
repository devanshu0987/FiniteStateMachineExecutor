# FiniteStateMachineExecutor
Implements a Finite state machine(FSM) executor with durable execution.

## General idea of FSM and code representation
- User writes Operations which logically represents a FSM.
- FSM structure
  - Each FSM is a Directed acyclic graph. Nodes in this DAG represents FSM states that are possible. When we want to go from one state to another, we execute a transition function.
  - We store some amount of state that remains available between transition function executions.
- Code structure
    - GlobalState struct : Stores all the operation specific data, state to persist between transitions.
    ```
    pub struct AddTwoNumOperationGlobalState {
        v1: u32,
        v2: u32,
        sum: u32,
    }
    ```
    - List of possible states
    ```
    pub enum States {
        start,
        end,
    }
    ```
    - Transition function definitions
    ```
    impl AddTwoNumOperationGlobalState {
        /// other things here ....

        /// Executed when we are in start state
        pub fn transition_fn_start(&mut self) -> Option<States> {
            self.sum = self.v1 + self.v2;
            Some(States::end)
        }

        /// Will refactor this out later. This essentially means we have reached the end of execution.
        pub fn transition_fn_dummy(&mut self) -> Option<States> {
            None
        }
    }
    ```
    - `execute_transition_function` defines the whole DAG.
    ```
    // define the DAG here
    pub fn execute_transition_function(&mut self, current_state: States) -> Option<States> {
        match current_state {
            States::start => self.transition_fn_start(),
            States::end => self.transition_fn_dummy(),
        }
    }
    ```
    - Initialization will need some args
    ```
    pub fn new(args: AddTwoNumOperationArgs) -> Self {
        AddTwoNumOperationGlobalState {
            v1: args.v1,
            v2: args.v2,
            sum: 0,
        }
    }
    ```
## General idea for executor
- Executor is the orchestrator of executions. It will store all the information necessary to perform execution.
    ```
    pub struct Executor {
        active_workflows: Vec<ExecutionContext>,
        /// other things here ...
    }
    ```
- We enqueue future executions into active workflows. This represents what things the executor needs to execute.
- It keeps popping elements out and performs the execution defined in the ExecutionContext.
- We define `ExecutionContext` as following
    ```
    pub enum ExecutionContext {
        Initialize {
            operation_name: String,
            serialized_operation_args: Option<String>,
        },
        Execute {
            operation_name: String,
            serialized_operation_args: Option<String>,
            serialized_operation_global_state: Option<String>,
            next_state_to_execute: Option<String>,
        },
    }
    ```
- `ExecutionContext` stores information in serialized format because later, we can persist these contexts in a database and make the executions durable between each transitions. For now, we will use in-memory data structures to house this data.
- Potential flow
  - `serialized_execution_context` is present with `Executor`. We can retrieve it from DB or any other source. It is just serialized string representing what we need to execute.
  -  We pass this to `ExecutionContext::new` to perform validation on the serialized data and generate `ExecutionContext`.
    - Validations
        - `operation_name` is valid or not. Does executor know about this FSM or not?
        - Once we know that a struct is present in the operations folder with the following name, we validate all the data wrt types and functions defined inside that file where it has been defined.
        - matching `serialized_operation_args` and creating a deserialized `operation_args` struct from the data.
        - validating `serialized_operation_global_state` with the operation said by `operation_name`.
        - validating if `next_state_to_execute` is present and has a matching transition function for it.
        - etc.
    - Once validation done, we run `ExecutionContext::run` which finds the real transition function that we defined in the context and executes it and returns us the resulting state.
    - We serialize the context again and store in the DB for future execution. if error occurs while execution, we store the same context again to DB. This is how we achieve durability between transitions.


## Things to figure out
- When we perform validation on `serialized_execution_context`, we need access to type information and associated functions wrt that type. How do we do that?
    - They are code and rust will ask me to make them available at compile time.
- We need to be able to generate proper type out from `serialized_operation_global_state` and get `AddTwoNumOperationGlobalState` from it. But serde need the type info to which to deserialize it to. For example: `let type: AddTwoNumOperationGlobalState = serde_json::from_str(&serialized_operation_global_state).unwrap();`
    - But to do this for **all** the operations we will define, the only answer I have is to write a bunch of if statements.
    - But that is not maintainable.
    - Essentially what I need is some kind of dynamic dispatch which does this matching for me.
- Maybe Executor exposes some function which says "hey register all these operations for me, this struct is for Global state, this enum is for state and this struct is for args"
    - Either it is able to do this on its own OR we provide it in some form. Not so maintainable but lets see.
    - Once that is done, it is able to deserialize things properly at runtime.