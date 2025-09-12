use std::{collections::HashMap, env::vars, ffi::CString};

pub struct Environment {
    map: HashMap<String, String>,
    lin: Option<Vec<CString>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            map: vars().collect(),
            lin: None,
        }
    }
}

impl Environment {
    pub fn get_env(&mut self) -> anyhow::Result<Vec<CString>> {
        Ok(self
            .lin
            .get_or_insert(
                self.map
                    .iter()
                    .map(|(k, v)| {
                        CString::new(format!("{k}={v}")).map_err(|e| anyhow::Error::new(e))
                    })
                    .collect::<anyhow::Result<Vec<CString>>>()?,
            )
            .clone())
    }
}
