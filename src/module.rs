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
