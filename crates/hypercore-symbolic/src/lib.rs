use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymbolicError {
    #[error("parsing error: {0}")]
    Parse(String),
    #[error("logic error: {0}")]
    Logic(String),
}

pub type SymbolicResult<T> = Result<T, SymbolicError>;

/// A Fact represents a piece of knowledge in the system.
/// It follows a simple Entity-Attribute-Value (EAV) or Triple model,
/// but can store structured JSON in 'value'.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Fact {
    pub entity: String,
    pub attribute: String,
    pub value: String, // standardized string or json
    pub confidence: u8, // 0-100
}

impl Fact {
    pub fn new(entity: &str, attribute: &str, value: &str) -> Self {
        Self {
            entity: entity.to_string(),
            attribute: attribute.to_string(),
            value: value.to_string(),
            confidence: 100,
        }
    }
}

/// A Rule defines a logical implication: IF all conditions are met, THEN assert the head.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub conditions: Vec<Condition>,
    pub head: Fact, // The fact to assert if conditions match
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub attribute: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Equals,
    Contains,
    // Add more as needed
}

/// The Symbolic Engine manages the working memory of facts and executes rules.
pub struct SymbolicEngine {
    facts: HashSet<Fact>,
    rules: Vec<Rule>,
}

impl SymbolicEngine {
    pub fn new() -> Self {
        Self {
            facts: HashSet::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.insert(fact);
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    /// Run forward chaining deduction to derive all possible facts.
    /// Returns the number of new facts derived.
    pub fn deduce(&mut self) -> usize {
        let mut new_facts_count = 0;
        
        // Simple fixed-point iteration
        loop {
            let mut derived_this_round = Vec::new();

            for rule in &self.rules {
                if self.check_conditions(&rule.conditions) {
                    // Check if head already exists
                    if !self.facts.contains(&rule.head) {
                        derived_this_round.push(rule.head.clone());
                    }
                }
            }

            if derived_this_round.is_empty() {
                break;
            }

            new_facts_count += derived_this_round.len();
            for f in derived_this_round {
                self.facts.insert(f);
            }
        }

        new_facts_count
    }

    fn check_conditions(&self, conditions: &[Condition]) -> bool {
        for cond in conditions {
            let match_found = self.facts.iter().any(|f| {
                f.attribute == cond.attribute && self.evaluate_op(&f.value, &cond.operator, &cond.value)
            });
            if !match_found {
                return false;
            }
        }
        true
    }

    fn evaluate_op(&self, fact_val: &str, op: &Operator, target_val: &str) -> bool {
        match op {
            Operator::Equals => fact_val == target_val,
            Operator::Contains => fact_val.contains(target_val),
        }
    }

    pub fn get_facts(&self) -> Vec<&Fact> {
        self.facts.iter().collect()
    }
}
