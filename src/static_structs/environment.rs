use {
    crate::DEFAULT_UTILS,
    std::{collections::HashMap, env::vars, ffi::CString, os::unix::fs::PermissionsExt},
};

pub struct Environment {
    map: HashMap<Vec<u8>, Vec<u8>>,
    lin: Option<Vec<CString>>,
}

impl Environment {
    pub fn get_env(&mut self) -> anyhow::Result<Vec<CString>> {
        Ok(self
            .lin
            .get_or_insert(
                self.map
                    .iter()
                    .map(|(k, v)| {
                        CString::new(format!(
                            "{}={}",
                            String::from_utf8_lossy(k),
                            String::from_utf8_lossy(v)
                        ))
                        .map_err(|e| anyhow::Error::new(e))
                    })
                    .collect::<anyhow::Result<Vec<CString>>>()?,
            )
            .clone())
    }

    pub fn get_full_path<'a>(&self, name: &'a mut Vec<u8>) -> anyhow::Result<&'a mut Vec<u8>> {
        match DEFAULT_UTILS.with_borrow(|utils| utils.name_into_path(name)) {
            true => Ok(name),
            false => {
                match Self::check_executable_file(name) {
                    Ok(_) => return Ok(name),
                    Err(_) => {}
                }

                let path = self
                    .map
                    .get(&"PSEUDOBASH_PATH".as_bytes().to_vec())
                    .unwrap();

                let mut buffer = Vec::new();
                for byte in path {
                    match *byte {
                        b':' => {
                            buffer.push(b'/');
                            buffer.append(&mut name.clone());
                            match Self::check_executable_file(&buffer) {
                                Ok(_) => {
                                    std::mem::swap(name, &mut buffer);
                                    return Ok(name);
                                }
                                Err(_) => {}
                            }
                            buffer.clear();
                        }
                        _ => {
                            buffer.push(*byte);
                        }
                    }
                }

                buffer.push(b'/');
                buffer.append(&mut name.clone());
                match Self::check_executable_file(&buffer) {
                    Ok(_) => std::mem::swap(name, &mut buffer),
                    Err(_) => anyhow::bail!(format!(
                        "No such file or directory: {:?}",
                        str::from_utf8(name)?
                    )),
                }

                Ok(name)
            }
        }
    }

    fn check_executable_file(path: &Vec<u8>) -> anyhow::Result<()> {
        let metadata = std::fs::metadata(str::from_utf8(&path[..path.len() - 1])?)?;

        if !metadata.is_file() {
            anyhow::bail!("Not a file");
        }

        match metadata.permissions().mode() & 0o111 != 0 {
            true => Ok(()),
            false => anyhow::bail!("Permissions denied"),
        }
    }

    // pub fn get_var(&self, name: &CString) -> Option<&CString> {
    //     self.map.get(name)
    // }

    // pub fn current_dir(&self) -> &CString {
    //     &self.current_dir
    // }
}

impl Default for Environment {
    fn default() -> Self {
        let mut map = HashMap::new();
        let mut lin = Vec::new();
        for (k, v) in vars() {
            if k == "PWD" {
                lin.push(
                    CString::new(format!(
                        "PSEUDOBASH_PATH={v}/utils:{v}/utils/echo/target/release"
                    ))
                    .unwrap(),
                );
                map.insert(
                    "PSEUDOBASH_PATH".as_bytes().to_vec(),
                    format!("{v}/utils:{v}/utils/echo/target/release")
                        .as_bytes()
                        .to_vec(),
                );
            }
            lin.push(CString::new(format!("{k}={v}")).unwrap());
            map.insert(k.as_bytes().to_vec(), v.as_bytes().to_vec());
        }

        Self {
            map,
            lin: Some(lin),
        }
    }
}
