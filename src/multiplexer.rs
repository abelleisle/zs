use serde::{Deserialize, Serialize};

use crate::session::Session;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Multiplexer {
    Zellij,
}

impl Multiplexer {
    pub fn _open(&self, _session: &Session) {
        match self {
            Multiplexer::Zellij => {}
        }
    }
}
