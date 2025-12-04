use std::fs;
use std::path::Path;

use mlua::{LuaSerdeExt, Result, UserData, UserDataMethods};

use crate::task::{TaskState, UndoSafety};

pub trait Module {
    fn apply(&self) -> Result<()>;
    fn undo(&self) -> Result<()>;
    fn undo_safety(&self) -> UndoSafety;
    fn state(&self) -> Result<TaskState>;
}

pub struct Echo {
    pub message: String,
}

impl Module for Echo {
    fn apply(&self) -> Result<()> {
        println!("{}", self.message);
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Safe
    }

    fn state(&self) -> Result<TaskState> {
        Ok(TaskState::Stateless)
    }
}

pub struct File {
    pub path: String,
}

impl Module for File {
    fn apply(&self) -> Result<()> {
        fs::write(&self.path, b"Created by nucleos")?;
        println!("File created: {}", self.path);
        Ok(())
    }

    fn undo(&self) -> Result<()> {
        if Path::new(&self.path).exists() {
            fs::remove_file(&self.path)?;
            println!("File removed: {}", self.path);
        }
        Ok(())
    }

    fn undo_safety(&self) -> UndoSafety {
        UndoSafety::Safe
    }

    fn state(&self) -> Result<TaskState> {
        let exists = Path::new(&self.path).exists();
        if exists {
            Ok(TaskState::Applied)
        } else {
            Ok(TaskState::NotApplied)
        }
    }
}

impl UserData for Echo {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("apply", |_, this, ()| this.apply());
        methods.add_method("undo", |_, this, ()| this.undo());
        methods.add_method("undo_safety", |lua, this, ()| {
            let s = this.undo_safety();
            lua.to_value(&s)
        });
        methods.add_method("state", |lua, this, ()| {
            let s = this.state()?;
            lua.to_value(&s)
        });
    }
}

impl UserData for File {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("apply", |_, this, ()| this.apply());
        methods.add_method("undo", |_, this, ()| this.undo());
        methods.add_method("undo_safety", |lua, this, ()| {
            let s = this.undo_safety();
            lua.to_value(&s)
        });
        methods.add_method("state", |lua, this, ()| {
            let s = this.state()?;
            lua.to_value(&s)
        });
    }
}
