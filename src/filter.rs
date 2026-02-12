use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub struct LogFilter {
    regex_filters: Vec<Regex>,
    field_filters: HashMap<String, String>,
    level_filter: Option<String>,
}

impl LogFilter {
    pub fn new() -> Self {
        Self {
            regex_filters: Vec::new(),
            field_filters: HashMap::new(),
            level_filter: None,
        }
    }
    
    pub fn add_regex_filter(&mut self, pattern: &str) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.regex_filters.push(regex);
        Ok(())
    }
    
    pub fn add_field_filter(&mut self, field: String, value: String) {
        self.field_filters.insert(field, value);
    }
    
    pub fn set_level_filter(&mut self, level: String) {
        self.level_filter = Some(level);
    }
    
    pub fn should_process(&self, log_entry: &Value) -> bool {
        // Check level filter
        if let Some(ref level_filter) = self.level_filter {
            if let Some(level) = log_entry.get("level").and_then(|v| v.as_str()) {
                if level != level_filter {
                    return false;
                }
            }
        }
        
        // Check field filters
        for (field, expected_value) in &self.field_filters {
            if let Some(actual_value) = log_entry.get(field).and_then(|v| v.as_str()) {
                if actual_value != expected_value {
                    return false;
                }
            }
        }
        
        // Check regex filters
        if !self.regex_filters.is_empty() {
            if let Some(message) = log_entry.get("message").and_then(|v| v.as_str()) {
                for regex in &self.regex_filters {
                    if regex.is_match(message) {
                        return true;
                    }
                }
                return false;
            }
        }
        
        true
    }
    
    pub fn clear_filters(&mut self) {
        self.regex_filters.clear();
        self.field_filters.clear();
        self.level_filter = None;
    }
}