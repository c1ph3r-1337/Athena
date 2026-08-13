use crate::task::{Task, TaskStatus};
use std::collections::{HashMap, VecDeque};

/// Returns IDs of tasks that are Pending and have all dependencies Completed
pub fn get_ready_tasks(tasks: &[Task]) -> Vec<String> {
    let task_map: HashMap<&String, &Task> = tasks.iter().map(|t| (&t.id, t)).collect();
    let mut ready = Vec::new();

    for task in tasks {
        if task.status == TaskStatus::Pending {
            let all_deps_completed = task.dependencies.iter().all(|dep_id| {
                task_map.get(dep_id).map(|t| t.status == TaskStatus::Completed).unwrap_or(false)
            });

            if all_deps_completed {
                ready.push(task.id.clone());
            }
        }
    }
    ready
}

/// Computes execution groups where tasks in the same group can run in parallel
pub fn get_parallel_groups(tasks: &[Task]) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    let mut in_degree: HashMap<&String, usize> = HashMap::new();
    let mut adj: HashMap<&String, Vec<&String>> = HashMap::new();

    for task in tasks {
        in_degree.insert(&task.id, task.dependencies.len());
        for dep in &task.dependencies {
            adj.entry(dep).or_default().push(&task.id);
        }
    }

    let mut current_group: Vec<&String> = in_degree.iter()
        .filter(|(_, deg)| **deg == 0)
        .map(|(id, _)| *id)
        .collect();

    while !current_group.is_empty() {
        groups.push(current_group.iter().map(|s| (*s).clone()).collect());

        let mut next_group = Vec::new();
        for node in &current_group {
            if let Some(neighbors) = adj.get(*node) {
                for neighbor in neighbors {
                    if let Some(deg) = in_degree.get_mut(*neighbor) {
                        *deg -= 1;
                        if *deg == 0 {
                            next_group.push(*neighbor);
                        }
                    }
                }
            }
        }
        current_group = next_group;
    }

    groups
}

/// Returns true if the task graph has cycles
pub fn has_circular_deps(tasks: &[Task]) -> bool {
    let mut in_degree: HashMap<&String, usize> = HashMap::new();
    let mut adj: HashMap<&String, Vec<&String>> = HashMap::new();

    for task in tasks {
        in_degree.insert(&task.id, task.dependencies.len());
        for dep in &task.dependencies {
            adj.entry(dep).or_default().push(&task.id);
        }
    }

    let mut queue = VecDeque::new();
    for (id, deg) in &in_degree {
        if *deg == 0 {
            queue.push_back(*id);
        }
    }

    let mut visited_count = 0;
    while let Some(node) = queue.pop_front() {
        visited_count += 1;
        if let Some(neighbors) = adj.get(node) {
            for neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(*neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
    }

    visited_count != tasks.len()
}

/// Returns true if every task is Completed
pub fn all_complete(tasks: &[Task]) -> bool {
    tasks.iter().all(|t| t.status == TaskStatus::Completed)
}

/// Returns true if any task is Failed
pub fn any_failed(tasks: &[Task]) -> bool {
    tasks.iter().any(|t| t.status == TaskStatus::Failed)
}

/// Print a text visualization of the task dependency graph
pub fn display_dag(tasks: &[Task]) {
    println!("Task Dependency Graph:");
    for task in tasks {
        let status_marker = match task.status {
            TaskStatus::Completed => "\x1b[32m✓\x1b[0m",
            TaskStatus::Running => "\x1b[33m⟳\x1b[0m",
            TaskStatus::Failed => "\x1b[31m✗\x1b[0m",
            _ => "○",
        };
        if task.dependencies.is_empty() {
            println!("  {} {} - {}", status_marker, task.id, task.title);
        } else {
            println!("  {} {} - {} (after: {})", status_marker, task.id, task.title, task.dependencies.join(", "));
        }
    }
}
