use std::{collections::HashMap, env::vars, ffi::CString};

pub struct Environment {
    map: HashMap<CString, CString>,
    lin: Option<Vec<CString>>,
    current_dir: CString,
}

impl Environment {
    pub fn new() -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        let mut lin = Vec::new();
        let mut current_dir = None;
        for (k, v) in vars() {
            if k == "PWD" {
                current_dir.replace(CString::new(v.clone())?);
            }
            lin.push(CString::new(format!("{k}={v}"))?);
            map.insert(CString::new(k)?, CString::new(v)?);
        }

        Ok(Self {
            map,
            lin: Some(lin),
            current_dir: current_dir.ok_or(anyhow::Error::msg("PWD var not in environment"))?,
        })
    }

    pub fn get_env(&mut self) -> anyhow::Result<Vec<CString>> {
        Ok(self
            .lin
            .get_or_insert(
                self.map
                    .iter()
                    .map(|(k, v)| {
                        CString::new(format!("{}={}", k.to_string_lossy(), v.to_string_lossy()))
                            .map_err(|e| anyhow::Error::new(e))
                    })
                    .collect::<anyhow::Result<Vec<CString>>>()?,
            )
            .clone())
    }

    pub fn get_var(&self, name: &CString) -> Option<&CString> {
        self.map.get(name)
    }

    pub fn current_dir(&self) -> &CString {
        &self.current_dir
    }
}
