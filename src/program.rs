use std::ffi::CString;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub command: CString,
    pub args: Vec<CString>,
    pub interactive: bool,
}

impl Program {
    pub fn new(command: CString, args: Vec<CString>, interactive: bool) -> Self {
        Self {
            command,
            args,
            interactive,
        }
    }
}
