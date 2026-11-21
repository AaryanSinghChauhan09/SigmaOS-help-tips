// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-help-tips: mint help tips component model.
//!
//! Real implementation: tipss, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tips {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintHelpTipsError {
    EmptyTipsName,
    DuplicateTips(String),
    EmptyEventSubject,
}

impl fmt::Display for MintHelpTipsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTipsName => write!(f, "tips name must not be empty"),
            Self::DuplicateTips(id) => write!(f, "duplicate tips {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintHelpTipsError {}

pub struct MintHelpTips {
    pub name: String,
    tipss: BTreeMap<String, Tips>,
    events: BTreeMap<String, Event>,
}

impl MintHelpTips {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tipss: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_tips(&mut self, item: Tips) -> Result<(), MintHelpTipsError> {
        if item.name.trim().is_empty() {
            return Err(MintHelpTipsError::EmptyTipsName);
        }
        let id = item.id.clone();
        if self.tipss.contains_key(&id) {
            return Err(MintHelpTipsError::DuplicateTips(id.clone()));
        }
        self.tipss.insert(id, item);
        Ok(())
    }

    pub fn tips_count(&self) -> usize {
        self.tipss.len()
    }

    pub fn get_tips(&self, id: &str) -> Option<&Tips> {
        self.tipss.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintHelpTipsError> {
        if event.subject.trim().is_empty() {
            return Err(MintHelpTipsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tips(id: &str, name: &str) -> Tips {
        Tips {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintHelpTips::new("MintHelpTips");
        assert_eq!(m.name, "MintHelpTips");
        assert_eq!(m.tips_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_tips() {
        let mut m = MintHelpTips::new("MintHelpTips");
        m.add_tips(sample_tips("a1", "Alpha")).unwrap();
        assert_eq!(m.tips_count(), 1);
        assert!(m.get_tips("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintHelpTips::new("MintHelpTips");
        assert!(m.add_tips(sample_tips("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintHelpTips::new("MintHelpTips");
        m.add_tips(sample_tips("x", "One")).unwrap();
        assert!(m.add_tips(sample_tips("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintHelpTips::new("MintHelpTips");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
