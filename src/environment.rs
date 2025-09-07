use std::{collections::HashMap, ffi::CString};

pub struct Environment {
    map: HashMap<String, String>,
    lin: Option<Vec<CString>>,
}

impl Default for Environment {
    fn default() -> Self {
        let map: HashMap<String, String> = std::env::vars().collect();
        Self {
            lin: Some(
                map.iter()
                    .map(|(k, v)| {
                        CString::new(format!("{k}={v}"))
                            .expect(&format!("Failed to create CString from `{k}={v}`"))
                    })
                    .collect(),
            ),
            map,
        }
    }
}
