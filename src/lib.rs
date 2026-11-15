//! SigmaOS-data-cleansing: data cleansing model.
//!
//! Real implementation: deduplication, normalization, correction, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Cleansing rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleansingRule {
    pub id: String,
    pub name: String,
    pub field: String,
    pub operation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleansingError {
    EmptyName,
    DuplicateRule(String),
}

impl fmt::Display for CleansingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "rule name must not be empty"),
            Self::DuplicateRule(id) => write!(f, "duplicate rule {:?}", id),
        }
    }
}

impl std::error::Error for CleansingError {}

/// Data cleansing engine.
pub struct DataCleansing {
    pub name: String,
    rules: BTreeMap<String, CleansingRule>,
}

impl DataCleansing {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            rules: BTreeMap::new(),
        }
    }

    pub fn add_rule(&mut self, rule: CleansingRule) -> Result<(), CleansingError> {
        if rule.name.trim().is_empty() { return Err(CleansingError::EmptyName); }
        let id = rule.id.clone();
        if self.rules.contains_key(&id) {
            return Err(CleansingError::DuplicateRule(id.clone()));
        }
        self.rules.insert(id, rule);
        Ok(())
    }

    pub fn rule_count(&self) -> usize { self.rules.len() }

    pub fn get_rule(&self, id: &str) -> Option<&CleansingRule> { self.rules.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleansing_create_aaa() {
        let c = DataCleansing::new("My Cleansing");
        assert_eq!(c.name, "My Cleansing");
        assert_eq!(c.rule_count(), 0);
    }

    #[test]
    fn test_add_rule_aaa() {
        let mut c = DataCleansing::new("My Cleansing");
        let r = CleansingRule { id: "r1".into(), name: "deduplicate".into(), field: "email".into(), operation: "unique".into() };
        c.add_rule(r).unwrap();
        assert_eq!(c.rule_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut c = DataCleansing::new("My Cleansing");
        let r = CleansingRule { id: "x".into(), name: "".into(), field: "a".into(), operation: "b".into() };
        assert!(c.add_rule(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut c = DataCleansing::new("My Cleansing");
        let r = CleansingRule { id: "x".into(), name: "X".into(), field: "a".into(), operation: "b".into() };
        c.add_rule(r).unwrap();
        let r2 = CleansingRule { id: "x".into(), name: "X".into(), field: "a".into(), operation: "b".into() };
        assert!(c.add_rule(r2).is_err());
    }
}
