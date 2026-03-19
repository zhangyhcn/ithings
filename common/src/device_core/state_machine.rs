use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct State {
    pub identifier: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub is_initial: bool,
    #[serde(default)]
    pub is_final: bool,
    #[serde(default)]
    pub on_enter: Vec<String>,
    #[serde(default)]
    pub on_exit: Vec<String>,
}

impl State {
    pub fn new(identifier: &str, name: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            description: None,
            is_initial: false,
            is_final: false,
            on_enter: Vec::new(),
            on_exit: Vec::new(),
        }
    }

    pub fn initial(mut self) -> Self {
        self.is_initial = true;
        self
    }

    pub fn final_state(mut self) -> Self {
        self.is_final = true;
        self
    }

    pub fn on_enter(mut self, action: &str) -> Self {
        self.on_enter.push(action.to_string());
        self
    }

    pub fn on_exit(mut self, action: &str) -> Self {
        self.on_exit.push(action.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    pub from_state: String,
    pub to_state: String,
    pub trigger: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default)]
    pub actions: Vec<String>,
}

impl Transition {
    pub fn new(from: &str, to: &str, trigger: &str) -> Self {
        Self {
            from_state: from.to_string(),
            to_state: to.to_string(),
            trigger: trigger.to_string(),
            condition: None,
            actions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }

    pub fn with_action(mut self, action: &str) -> Self {
        self.actions.push(action.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachine {
    pub identifier: String,
    pub name: String,
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    #[serde(default)]
    pub variables: HashMap<String, serde_json::Value>,
}

impl StateMachine {
    pub fn new(identifier: &str, name: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            states: Vec::new(),
            transitions: Vec::new(),
            variables: HashMap::new(),
        }
    }

    pub fn add_state(mut self, state: State) -> Self {
        self.states.push(state);
        self
    }

    pub fn add_transition(mut self, transition: Transition) -> Self {
        self.transitions.push(transition);
        self
    }

    pub fn get_initial_state(&self) -> Option<&State> {
        self.states.iter().find(|s| s.is_initial)
    }

    pub fn get_state(&self, identifier: &str) -> Option<&State> {
        self.states.iter().find(|s| s.identifier == identifier)
    }

    pub fn get_transitions_from(&self, state_identifier: &str) -> Vec<&Transition> {
        self.transitions.iter()
            .filter(|t| t.from_state == state_identifier)
            .collect()
    }

    pub fn find_transition(&self, from_state: &str, trigger: &str) -> Option<&Transition> {
        self.transitions.iter()
            .find(|t| t.from_state == from_state && t.trigger == trigger)
    }
}

#[derive(Debug, Clone)]
pub struct StateMachineInstance {
    pub state_machine: StateMachine,
    pub current_state: String,
    pub context: HashMap<String, serde_json::Value>,
}

impl StateMachineInstance {
    pub fn new(state_machine: StateMachine) -> Option<Self> {
        let initial_state = state_machine.get_initial_state()?;
        Some(Self {
            current_state: initial_state.identifier.clone(),
            state_machine,
            context: HashMap::new(),
        })
    }

    pub fn current_state(&self) -> Option<&State> {
        self.state_machine.get_state(&self.current_state)
    }

    pub fn can_transition(&self, trigger: &str) -> bool {
        self.state_machine.find_transition(&self.current_state, trigger).is_some()
    }

    pub fn transition(&mut self, trigger: &str) -> Result<Vec<String>, String> {
        let transition = self.state_machine.find_transition(&self.current_state, trigger)
            .ok_or_else(|| format!("No transition found for trigger '{}' from state '{}'", trigger, self.current_state))?;

        let from_state = self.state_machine.get_state(&self.current_state)
            .ok_or_else(|| format!("State '{}' not found", self.current_state))?;

        if from_state.is_final {
            return Err(format!("Cannot transition from final state '{}'", self.current_state));
        }

        let to_state = self.state_machine.get_state(&transition.to_state)
            .ok_or_else(|| format!("Target state '{}' not found", transition.to_state))?;

        let mut actions = Vec::new();
        actions.extend(from_state.on_exit.clone());
        actions.extend(transition.actions.clone());
        actions.extend(to_state.on_enter.clone());

        self.current_state = transition.to_state.clone();

        Ok(actions)
    }
}

#[derive(Debug, Clone)]
pub struct StateMachineContext {
    pub instance: StateMachineInstance,
    pub last_transition: Option<Transition>,
    pub last_actions: Vec<String>,
}

impl StateMachineContext {
    pub fn new(state_machine: StateMachine, initial_state: String) -> Self {
        let instance = StateMachineInstance {
            state_machine,
            current_state: initial_state,
            context: HashMap::new(),
        };
        Self {
            instance,
            last_transition: None,
            last_actions: Vec::new(),
        }
    }

    pub fn current_state(&self) -> Option<&State> {
        self.instance.current_state()
    }

    pub fn current_state_identifier(&self) -> &str {
        &self.instance.current_state
    }

    pub fn trigger(&mut self, trigger: &str) -> Result<Vec<String>, String> {
        let actions = self.instance.transition(trigger)?;
        self.last_actions = actions.clone();
        self.last_transition = self.instance.state_machine
            .find_transition(&self.instance.current_state, trigger)
            .cloned();
        Ok(actions)
    }

    pub fn set_variable(&mut self, key: &str, value: serde_json::Value) {
        self.instance.context.insert(key.to_string(), value);
    }

    pub fn get_variable(&self, key: &str) -> Option<&serde_json::Value> {
        self.instance.context.get(key)
    }
}
