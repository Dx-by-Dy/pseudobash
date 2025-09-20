use {
    crate::global_struct::default_utils::DefaultUtils,
    std::{collections::HashMap, env::vars, ffi::CString, io::Read, os::unix::fs::PermissionsExt},
};

#[derive(Clone)]
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

    pub fn get_full_path<'a>(
        &self,
        name: &'a mut Vec<u8>,
        default_utils: &DefaultUtils,
    ) -> anyhow::Result<()> {
        match default_utils.name_into_path(name) {
            true => Ok(()),
            false => {
                match Self::check_executable_file(name) {
                    Ok(_) => return Ok(()),
                    Err(_) => {}
                }

                let path = self
                    .map
                    .get(&"PSEUDOBASH_PATH".as_bytes().to_vec())
                    .ok_or(anyhow::Error::msg("$PSEUDOBASH_PATH not found"))
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
                                    return Ok(());
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

                Ok(())
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

    pub fn get_var(&self, name: &str) -> Option<&Vec<u8>> {
        self.map.get(&name.as_bytes().to_vec())
    }

    pub fn set_var(&mut self, name: Vec<u8>, value: Vec<u8>) {
        self.map.insert(name, value);
        self.lin = None;
    }
}

impl Default for Environment {
    fn default() -> Self {
        let mut map = HashMap::new();
        let mut lin = Vec::new();
        for (k, v) in vars() {
            // if k == "PWD" {
            //     lin.push(
            //         CString::new(format!(
            //             "PSEUDOBASH_PATH={v}/utils:{v}/utils/echo/target/release"
            //         ))
            //         .unwrap(),
            //     );
            //     map.insert(
            //         "PSEUDOBASH_PATH".as_bytes().to_vec(),
            //         format!("{v}/utils:{v}/utils/echo/target/release")
            //             .as_bytes()
            //             .to_vec(),
            //     );
            // }
            lin.push(CString::new(format!("{k}={v}")).unwrap());
            map.insert(k.as_bytes().to_vec(), v.as_bytes().to_vec());
        }

        let mut file = std::fs::File::open(
            std::env::current_exe()
                .expect("Failed to parse current exe path")
                .parent()
                .expect("Failed to parse parenr directory of current exe path")
                .join("../../.env")
                .canonicalize()
                .expect("Failed to canonicalize current exe path"),
        )
        .unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        for line in data.split(|sym| sym == '\n') {
            let mut is_key = true;
            let mut k = Vec::new();
            let mut v = Vec::new();
            for sym in line.as_bytes() {
                if *sym == b'=' {
                    is_key = false;
                }
                if is_key {
                    k.push(*sym);
                } else {
                    v.push(*sym);
                }
            }
            lin.push(
                CString::new(format!(
                    "{}={}",
                    String::from_utf8_lossy(&k),
                    String::from_utf8_lossy(&v)
                ))
                .unwrap(),
            );
            map.insert(k, v);
        }

        Self {
            map,
            lin: Some(lin),
        }
    }
}
