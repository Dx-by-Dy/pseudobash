use {
    lazy_static::lazy_static,
    std::{
        cell::Cell,
        collections::HashMap,
        env::vars,
        ffi::CString,
        sync::{Arc, Mutex},
    },
};

struct Environment {
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
    fn get_env(&mut self) -> Vec<CString> {
        self.lin
            .get_or_insert(
                self.map
                    .iter()
                    .map(|(k, v)| {
                        CString::new(format!("{k}={v}"))
                            .expect(&format!("Failed to create CString from `{k}={v}`"))
                    })
                    .collect(),
            )
            .clone()
    }
}

pub struct GEnvironment {
    env: Arc<Mutex<Cell<Option<Environment>>>>,
}

impl Default for GEnvironment {
    fn default() -> Self {
        Self {
            env: Arc::new(Mutex::new(Cell::new(Some(Environment::default())))),
        }
    }
}

impl GEnvironment {
    pub fn get_env(&self) -> anyhow::Result<Vec<CString>> {
        let mg = self
            .env
            .lock()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;
        let mut environment = mg.take().unwrap();
        let result = environment.get_env();
        mg.replace(Some(environment));
        Ok(result)
    }
}

lazy_static! {
    pub static ref ENVIRONMENT: GEnvironment = GEnvironment::default();
}
