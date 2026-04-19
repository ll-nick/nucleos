use std::fs;
use std::path::Path;

use mlua::{LuaSerdeExt, Result, UserData, UserDataMethods, Value};
use serde::{Deserialize, Serialize};

// ---------- Core types ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub module_type: String,
    pub params: serde_json::Value,
    pub state: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyResult {
    pub changed: bool,
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Reversibility {
    Reversible,
    Imperfect { hint: String },
    Irreversible,
}

// ---------- Module trait ----------

pub trait Module {
    fn check_requirements(&self) -> Result<()> {
        Ok(())
    }
    /// Idempotent: checks current state against desired state, acts only if needed.
    /// Receives the previously stored snapshot if one exists.
    fn apply(&self, snapshot: Option<&Snapshot>) -> Result<ApplyResult>;
    fn undo(&self, snapshot: &Snapshot) -> Result<()>;
    fn reversibility(&self) -> Reversibility;
}

// ---------- LuaModule wrapper ----------

pub struct LuaModule(pub Box<dyn Module>);

impl UserData for LuaModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("check_requirements", |_, this, ()| {
            this.0.check_requirements()
        });

        methods.add_method("apply", |lua, this, snapshot_value: Value| {
            let snapshot: Option<Snapshot> = match snapshot_value {
                Value::Nil => None,
                v => Some(lua.from_value(v)?),
            };
            let result = this.0.apply(snapshot.as_ref())?;
            lua.to_value(&result)
        });

        methods.add_method("undo", |lua, this, snapshot_value: Value| {
            let snapshot: Snapshot = lua.from_value(snapshot_value)?;
            this.0.undo(&snapshot)
        });

        methods.add_method("reversibility", |lua, this, ()| {
            lua.to_value(&this.0.reversibility())
        });
    }
}

// ---------- Helpers ----------

fn make_snapshot(module_type: &str, params: serde_json::Value, state: serde_json::Value) -> Snapshot {
    Snapshot {
        module_type: module_type.to_string(),
        params,
        state,
    }
}

// ---------- Echo ----------

pub struct Echo {
    pub message: String,
}

impl Module for Echo {
    fn apply(&self, _snapshot: Option<&Snapshot>) -> Result<ApplyResult> {
        println!("{}", self.message);
        Ok(ApplyResult {
            changed: true,
            snapshot: None,
        })
    }

    fn undo(&self, _snapshot: &Snapshot) -> Result<()> {
        Ok(())
    }

    fn reversibility(&self) -> Reversibility {
        Reversibility::Irreversible
    }
}

// ---------- Copy ----------

/// Copies a file from `src` to `dest`, or writes inline `content` to `dest`.
/// Exactly one of `src` or `content` must be provided.
/// Refuses if `dest` already exists unless `force = true`, in which case prior
/// contents are captured in the snapshot for undo (Imperfect reversibility).
/// If Nucleos owns the path (snapshot present) and content has drifted, it re-applies.
pub struct Copy {
    pub src: Option<String>,
    pub content: Option<String>,
    pub dest: String,
    pub force: bool,
}

impl Copy {
    fn resolve_content(&self) -> Result<String> {
        match (&self.src, &self.content) {
            (Some(src), None) => fs::read_to_string(src).map_err(mlua::Error::external),
            (None, Some(c)) => Ok(c.clone()),
            (Some(_), Some(_)) => Err(mlua::Error::runtime(
                "copy: specify either src or content, not both",
            )),
            (None, None) => Err(mlua::Error::runtime(
                "copy: either src or content is required",
            )),
        }
    }

    fn snapshot(&self, state: serde_json::Value) -> Snapshot {
        make_snapshot(
            "copy",
            serde_json::json!({
                "src": self.src,
                "content": self.content,
                "dest": self.dest,
                "force": self.force,
            }),
            state,
        )
    }
}

impl Module for Copy {
    fn apply(&self, snapshot: Option<&Snapshot>) -> Result<ApplyResult> {
        let dest = Path::new(&self.dest);
        let desired = self.resolve_content()?;

        if let Some(existing) = snapshot {
            // We own this file — re-apply if content drifted or file was deleted.
            let already_correct = dest.exists()
                && fs::read_to_string(dest)
                    .map(|c| c == desired)
                    .unwrap_or(false);
            if already_correct {
                return Ok(ApplyResult {
                    changed: false,
                    snapshot: Some(self.snapshot(existing.state.clone())),
                });
            }
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).map_err(mlua::Error::external)?;
            }
            fs::write(dest, &desired).map_err(mlua::Error::external)?;
            return Ok(ApplyResult {
                changed: true,
                snapshot: Some(self.snapshot(existing.state.clone())),
            });
        }

        // First apply: refuse if dest exists unless force = true.
        if dest.exists() {
            if !self.force {
                return Err(mlua::Error::runtime(format!(
                    "copy: '{}' already exists; use force = true to overwrite",
                    self.dest
                )));
            }
            let prior = fs::read_to_string(dest).map_err(mlua::Error::external)?;
            fs::write(dest, &desired).map_err(mlua::Error::external)?;
            return Ok(ApplyResult {
                changed: true,
                snapshot: Some(self.snapshot(serde_json::json!({
                    "existed_before": true,
                    "prior_contents": prior,
                }))),
            });
        }

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).map_err(mlua::Error::external)?;
        }
        fs::write(dest, &desired).map_err(mlua::Error::external)?;
        Ok(ApplyResult {
            changed: true,
            snapshot: Some(self.snapshot(serde_json::json!({ "existed_before": false }))),
        })
    }

    fn undo(&self, snapshot: &Snapshot) -> Result<()> {
        let dest = Path::new(&self.dest);
        let existed_before = snapshot.state["existed_before"].as_bool().unwrap_or(false);
        if existed_before {
            let prior = snapshot.state["prior_contents"].as_str().unwrap_or("");
            fs::write(dest, prior).map_err(mlua::Error::external)?;
        } else if dest.exists() {
            fs::remove_file(dest).map_err(mlua::Error::external)?;
        }
        Ok(())
    }

    fn reversibility(&self) -> Reversibility {
        if self.force {
            Reversibility::Imperfect {
                hint: "file was overwritten; prior contents were captured but permissions \
                       and metadata may differ"
                    .to_string(),
            }
        } else {
            Reversibility::Reversible
        }
    }
}
