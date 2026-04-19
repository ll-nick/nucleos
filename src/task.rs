use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Applied,
    NotApplied,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskOpts {
    pub enabled: bool,
}

impl Default for TaskOpts {
    fn default() -> Self {
        TaskOpts { enabled: true }
    }
}

pub struct Task {
    pub id: String,
    pub module: mlua::AnyUserData,
    pub opts: TaskOpts,
}
