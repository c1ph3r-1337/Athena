use std::path::Path;
use crate::task::{Task, TaskStatus};

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn add_error(&mut self, err: String) {
        self.errors.push(err);
        self.valid = false;
    }
}

pub fn validate_plan(tasks: &[Task]) -> ValidationResult {
    let mut result = ValidationResult::new();
    let mut task_ids = std::collections::HashSet::new();

    for task in tasks {
        if task.title.trim().is_empty() {
            result.add_error(format!("Task {} has an empty title", task.id));
        }
        if task.description.trim().is_empty() {
            result.add_error(format!("Task {} has an empty description", task.id));
        }
        if task.assigned_agent.is_none() {
            result.add_error(format!("Task {} has no assigned agent", task.id));
        }
        if !task_ids.insert(&task.id) {
            result.add_error(format!("Duplicate task ID: {}", task.id));
        }
    }

    for task in tasks {
        for dep in &task.dependencies {
            if !task_ids.contains(dep) {
                result.add_error(format!("Task {} references non-existent dependency: {}", task.id, dep));
            }
        }
    }

    if crate::scheduler::has_circular_deps(tasks) {
        result.add_error("Circular dependencies detected in the task graph".to_string());
    }

    result
}

pub fn validate_completion(tasks: &[Task], _session_dir: &Path) -> ValidationResult {
    let mut result = ValidationResult::new();

    for task in tasks {
        if task.status != TaskStatus::Completed {
            result.add_error(format!("Task {} is not completed (status: {:?})", task.id, task.status));
        }
        
        if task.status == TaskStatus::Failed || task.status == TaskStatus::Blocked {
            result.add_error(format!("Task {} is in a failed or blocked state", task.id));
        }

        if let Some(output_file) = &task.output_file {
            let path = Path::new(output_file);
            if !path.exists() {
                result.add_error(format!("Output file for task {} does not exist: {}", task.id, output_file));
            }
        }
    }

    result
}

pub fn display_validation(result: &ValidationResult) {
    if result.valid {
        println!("  \x1b[32m✓\x1b[0m Validation passed");
    } else {
        println!("  \x1b[31m✗\x1b[0m Validation failed");
    }
    for err in &result.errors {
        println!("    \x1b[31m✗\x1b[0m {}", err);
    }
    for warn in &result.warnings {
        println!("    \x1b[33m▸\x1b[0m {}", warn);
    }
}
