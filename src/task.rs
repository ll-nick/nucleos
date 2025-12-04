use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskState {
    Applied,
    OutOfDate,
    NotApplied,
    Stateless,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoSafety {
    /// Fully revertible without side effects
    Safe,

    /// Reversible but may have side effects
    Risky,

    /// Cannot be undone automatically
    Impossible,
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
    pub name: String,
    pub module: mlua::AnyUserData,
    pub opts: TaskOpts,
}
