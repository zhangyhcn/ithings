use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "matches")]
    Matches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub property_identifier: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
}

impl RuleCondition {
    pub fn new(property_identifier: &str, operator: ConditionOperator, value: serde_json::Value) -> Self {
        Self {
            property_identifier: property_identifier.to_string(),
            operator,
            value,
        }
    }

    pub fn evaluate(&self, property_value: &serde_json::Value) -> bool {
        match &self.operator {
            ConditionOperator::Equal => property_value == &self.value,
            ConditionOperator::NotEqual => property_value != &self.value,
            ConditionOperator::GreaterThan => {
                compare_values(property_value, &self.value, |a, b| a > b)
            }
            ConditionOperator::GreaterThanOrEqual => {
                compare_values(property_value, &self.value, |a, b| a >= b)
            }
            ConditionOperator::LessThan => {
                compare_values(property_value, &self.value, |a, b| a < b)
            }
            ConditionOperator::LessThanOrEqual => {
                compare_values(property_value, &self.value, |a, b| a <= b)
            }
            ConditionOperator::Contains => {
                if let (serde_json::Value::String(s), serde_json::Value::String(pattern)) = (property_value, &self.value) {
                    s.contains(pattern)
                } else {
                    false
                }
            }
            ConditionOperator::Matches => {
                if let (serde_json::Value::String(s), serde_json::Value::String(pattern)) = (property_value, &self.value) {
                    regex::Regex::new(pattern)
                        .map(|re| re.is_match(s))
                        .unwrap_or(false)
                } else {
                    false
                }
            }
        }
    }
}

fn compare_values<F>(a: &serde_json::Value, b: &serde_json::Value, cmp: F) -> bool
where
    F: Fn(f64, f64) -> bool,
{
    match (a, b) {
        (serde_json::Value::Number(a_num), serde_json::Value::Number(b_num)) => {
            if let (Some(a_f), Some(b_f)) = (a_num.as_f64(), b_num.as_f64()) {
                cmp(a_f, b_f)
            } else {
                false
            }
        }
        _ => false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    #[serde(rename = "set_property")]
    SetProperty { identifier: String, value: serde_json::Value },
    #[serde(rename = "trigger_event")]
    TriggerEvent { event_identifier: String, data: HashMap<String, serde_json::Value> },
    #[serde(rename = "call_service")]
    CallService { service_identifier: String, params: HashMap<String, serde_json::Value> },
    #[serde(rename = "log")]
    Log { level: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub identifier: String,
    pub name: String,
    pub conditions: Vec<RuleCondition>,
    #[serde(rename = "condition_logic")]
    pub logic: ConditionLogic,
    pub actions: Vec<RuleAction>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConditionLogic {
    #[default]
    And,
    Or,
}

impl Rule {
    pub fn new(identifier: &str, name: &str) -> Self {
        Self {
            identifier: identifier.to_string(),
            name: name.to_string(),
            conditions: Vec::new(),
            logic: ConditionLogic::And,
            actions: Vec::new(),
            enabled: true,
        }
    }

    pub fn with_condition(mut self, condition: RuleCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_action(mut self, action: RuleAction) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_logic(mut self, logic: ConditionLogic) -> Self {
        self.logic = logic;
        self
    }

    pub fn evaluate(&self, property_values: &HashMap<String, serde_json::Value>) -> bool {
        if !self.enabled {
            return false;
        }

        let results: Vec<bool> = self.conditions.iter()
            .map(|cond| {
                property_values.get(&cond.property_identifier)
                    .map(|v| cond.evaluate(v))
                    .unwrap_or(false)
            })
            .collect();

        match self.logic {
            ConditionLogic::And => results.iter().all(|&r| r),
            ConditionLogic::Or => results.iter().any(|&r| r),
        }
    }
}
