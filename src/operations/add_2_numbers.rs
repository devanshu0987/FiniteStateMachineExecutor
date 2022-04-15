pub struct AddTwoNumOperationGlobalState {
    v1: u32,
    v2: u32,
    sum: u32,
}

/// args should be available to new so that they can be initialized properly
pub struct AddTwoNumOperationArgs {
    v1: u32,
    v2: u32,
}

pub enum States {
    start,
    end,
}

impl AddTwoNumOperationGlobalState {
    pub fn execute_transition_function(&mut self, current_state: States) -> Option<States> {
        match current_state {
            States::start => self.transition_fn_start(),
            States::end => self.transition_fn_dummy(),
        }
    }
    /// if it is first time, then we initialize the Global state, but post that we deserialize it from storage
    pub fn new(args: AddTwoNumOperationArgs) -> Self {
        AddTwoNumOperationGlobalState {
            v1: args.v1,
            v2: args.v2,
            sum: 0,
        }
    }

    /// Executed when we are in start state
    pub fn transition_fn_start(&mut self) -> Option<States> {
        self.sum = self.v1 + self.v2;
        Some(States::end)
    }

    pub fn transition_fn_dummy(&mut self) -> Option<States> {
        None
    }
}
