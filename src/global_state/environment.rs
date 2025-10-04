use std::{
    collections::{HashMap, hash_map::Iter},
    env::{current_exe, var, vars},
};

#[derive(Clone)]
pub struct Environment {
    map: HashMap<String, String>,
}

impl Environment {
    pub fn get_var(&self, name: &mut Vec<u8>) {
        std::mem::swap(
            name,
            &mut self
                .map
                .get(&String::from_utf8_lossy(name).to_string())
                .unwrap_or(&String::new())
                .as_bytes()
                .to_vec(),
        );
    }

    pub fn set_var(&mut self, var: Vec<u8>) {
        let mut is_name = true;
        let mut name = Vec::new();
        let mut value = Vec::new();
        for byte in var {
            if byte == b'=' && is_name {
                is_name = false;
                continue;
            }
            if is_name {
                name.push(byte);
            } else {
                value.push(byte);
            }
        }
        self.map.insert(
            String::from_utf8_lossy(&name).to_string(),
            String::from_utf8_lossy(&value).to_string(),
        );
    }

    pub fn vars(&self) -> Iter<String, String> {
        self.map.iter()
    }
}

impl Default for Environment {
    fn default() -> Self {
        let mut result = Self {
            map: vars().collect(),
        };

        match current_exe()
            .map_err(|e| anyhow::Error::new(e))
            .and_then(|path| {
                path.parent()
                    .ok_or(anyhow::Error::msg(""))
                    .map(|path| path.to_owned())
            })
            .and_then(|path| {
                path.join("../utils/release")
                    .canonicalize()
                    .map_err(|e| anyhow::Error::new(e))
            })
            .map(|path| path.to_string_lossy().to_string())
            .map(|mut string| {
                string.push(':');
                string.push_str(&var("PATH").unwrap());
                string
            }) {
            Ok(path_value) => {
                result.map.insert("PATH".to_string(), path_value);
            }
            Err(e) => eprintln!("WARNING! Failed to modify $PATH: {}", e),
        };

        result
    }
}
