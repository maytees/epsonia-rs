use std::path::Path;

use epsonia_util::get_users;

#[derive(PartialEq, Clone)]
pub struct Check {
    pub points: i32,
    pub message: String,
    pub penalty_message: String,
    pub completed: bool,
    pub kind: CheckKind,
}

#[derive(PartialEq, Clone)]
pub enum CheckKind {
    FileExists {
        file_path: String,
        should_exist: bool,
    },
    FileLineContains {
        file_path: String,
        line: i32,
        line_content: String,
        should_contain: bool,
    },
    FileContainsContent {
        file_path: String,
        content: String,
        whitespace_matters: bool,
        should_contain: bool,
    },
    ServiceUp {
        service_name: String,
        should_be_up: bool,
    },
    BinaryExists {
        binary_name: String,
        should_exist: bool,
    },
    UserInGroup {
        user: String,
        group: String,
        should_be: bool,
    },
    UserIsAdminstrator {
        user: String,
        should_be: bool,
    },
    User {
        user: String,
        should_exist: bool,
        does_exist: bool,
    },
}

impl Check {
    pub fn new(
        points: i32,
        message: String,
        penalty_message: String,
        completed: bool,
        kind: CheckKind,
    ) -> Self {
        Check {
            points,
            message,
            penalty_message,
            completed,
            kind,
        }
    }
    pub fn run_check(&mut self) -> Self {
        self.completed = match &self.kind {
            CheckKind::FileExists {
                file_path,
                should_exist,
            } => Path::new(file_path).exists() == *should_exist,
            CheckKind::FileLineContains {
                file_path,
                line,
                line_content,
                should_contain,
            } => {
                // Don't have error handling yet.
                let file = std::fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
                let lines: Vec<&str> = file.split('\n').collect();
                let line = lines[(*line - 1) as usize];
                line.contains(line_content) == *should_contain
            }
            CheckKind::FileContainsContent {
                file_path,
                content,
                whitespace_matters,
                should_contain,
            } => {
                let file = std::fs::read_to_string(file_path).unwrap_or_else(|_| String::new());
                if *whitespace_matters {
                    file.contains(content) == *should_contain
                } else {
                    file.replace(' ', "").contains(content) == *should_contain
                }
            }
            CheckKind::ServiceUp {
                service_name,
                should_be_up,
            } => {
                let output = std::process::Command::new("systemctl")
                    .arg("is-active")
                    .arg(service_name)
                    .output()
                    .expect("Failed to execute command");
                let output = String::from_utf8_lossy(&output.stdout);
                output.contains("inactive") != *should_be_up
            }
            CheckKind::BinaryExists {
                binary_name,
                should_exist,
            } => {
                let output = std::process::Command::new("which")
                    .arg(binary_name)
                    .output()
                    .expect("Failed to execute command");
                let output = String::from_utf8_lossy(&output.stdout);
                output.contains(binary_name) == *should_exist
            }
            CheckKind::UserInGroup {
                user,
                group,
                should_be,
            } => {
                let output = std::process::Command::new("id")
                    .arg(user)
                    .output()
                    .expect("Failed to execute command");
                let output = String::from_utf8_lossy(&output.stdout);
                output.contains(group) == *should_be
            }
            CheckKind::UserIsAdminstrator { user, should_be } => {
                let output = std::process::Command::new("id")
                    .arg(user)
                    .output()
                    .expect("Failed to execute command");
                let output = String::from_utf8_lossy(&output.stdout);
                output.contains("sudo") == *should_be
            }
            CheckKind::User {
                user,
                should_exist,
                does_exist, // Initial exist
            } => {
                if *should_exist && *does_exist && get_users().iter().any(|u| u.name == *user) {
                    true
                } else if !should_exist
                    && !does_exist
                    && !get_users().iter().any(|u| u.name == *user)
                {
                    true
                } else {
                    false
                }
            }
        };

        self.clone()
    }
}
