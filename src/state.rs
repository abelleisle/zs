use std::{
    cell::{Ref, RefCell, RefMut},
    sync::OnceLock,
};

use anyhow::Result;

use crate::{
    config::Config,
    session::{Session, SessionMap},
};

pub struct State {
    sessions: RefCell<Option<SessionMap>>,
    config: OnceLock<Config>,
}

impl State {
    pub fn new() -> Self {
        State {
            sessions: RefCell::new(None),
            config: OnceLock::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(ref sessions) = *self.sessions.borrow() {
            Session::save_all(sessions)?
        }

        Ok(())
    }

    pub fn config(&self) -> &Config {
        self.config
            .get_or_init(|| Config::load().expect("Failed to load config"))
    }

    pub fn sessions(&self) -> Ref<'_, SessionMap> {
        if self.sessions.borrow().is_none() {
            *self.sessions.borrow_mut() =
                Some(Session::load_all(self.config()).expect("Failed to load sessions"));
        }
        Ref::map(self.sessions.borrow(), |opt| opt.as_ref().unwrap())
    }

    pub fn sessions_mut(&self) -> RefMut<'_, SessionMap> {
        if self.sessions.borrow().is_none() {
            *self.sessions.borrow_mut() =
                Some(Session::load_all(self.config()).expect("Failed to load sessions"));
        }
        RefMut::map(self.sessions.borrow_mut(), |opt| opt.as_mut().unwrap())
    }
}
