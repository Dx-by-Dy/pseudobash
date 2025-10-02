use std::{
    collections::{HashMap, hash_map::Iter},
    env::vars,
};

#[derive(Clone)]
pub struct Environment {
    map: HashMap<String, String>,
}

impl Environment {
    // pub fn get_env(&mut self) -> anyhow::Result<Vec<CString>> {
    //     Ok(self
    //         .lin
    //         .get_or_insert(
    //             self.map
    //                 .iter()
    //                 .map(|(k, v)| {
    //                     CString::new(format!(
    //                         "{}={}",
    //                         String::from_utf8_lossy(k),
    //                         String::from_utf8_lossy(v)
    //                     ))
    //                     .map_err(|e| anyhow::Error::new(e))
    //                 })
    //                 .collect::<anyhow::Result<Vec<CString>>>()?,
    //         )
    //         .clone())
    // }

    // pub fn get_full_path<'a>(
    //     &self,
    //     name: &'a mut Vec<u8>,
    //     default_utils: &DefaultUtils,
    // ) -> anyhow::Result<()> {
    //     match default_utils.name_into_path(name) {
    //         true => Ok(()),
    //         false => {
    //             match Self::check_executable_file(name) {
    //                 Ok(_) => return Ok(()),
    //                 Err(_) => {}
    //             }

    //             let path = self
    //                 .map
    //                 .get(&"PSEUDOBASH_PATH".as_bytes().to_vec())
    //                 .ok_or(anyhow::Error::msg("$PSEUDOBASH_PATH not found"))
    //                 .unwrap();

    //             let mut buffer = Vec::new();
    //             for byte in path {
    //                 match *byte {
    //                     b':' => {
    //                         buffer.push(b'/');
    //                         buffer.append(&mut name.clone());
    //                         match Self::check_executable_file(&buffer) {
    //                             Ok(_) => {
    //                                 std::mem::swap(name, &mut buffer);
    //                                 return Ok(());
    //                             }
    //                             Err(_) => {}
    //                         }
    //                         buffer.clear();
    //                     }
    //                     _ => {
    //                         buffer.push(*byte);
    //                     }
    //                 }
    //             }

    //             buffer.push(b'/');
    //             buffer.append(&mut name.clone());
    //             match Self::check_executable_file(&buffer) {
    //                 Ok(_) => std::mem::swap(name, &mut buffer),
    //                 Err(_) => anyhow::bail!(format!(
    //                     "No such file or directory: {:?}",
    //                     str::from_utf8(name)?
    //                 )),
    //             }

    //             Ok(())
    //         }
    //     }
    // }

    // fn check_executable_file(path: &Vec<u8>) -> anyhow::Result<()> {
    //     let metadata = std::fs::metadata(str::from_utf8(&path[..path.len() - 1])?)?;

    //     if !metadata.is_file() {
    //         anyhow::bail!("Not a file");
    //     }

    //     match metadata.permissions().mode() & 0o111 != 0 {
    //         true => Ok(()),
    //         false => anyhow::bail!("Permissions denied"),
    //     }
    // }

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
            if byte == b'=' {
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
        // if !cfg!(test) {
        //     let mut file = std::fs::File::open(
        //         std::env::current_exe()
        //             .expect("Failed to parse current exe path")
        //             .parent()
        //             .expect("Failed to parse parenr directory of current exe path")
        //             .join("../.env")
        //             .canonicalize()
        //             .expect("Failed to canonicalize current exe path"),
        //     )
        //     .unwrap();
        //     let mut data = String::new();
        //     file.read_to_string(&mut data).unwrap();
        //     for line in data.split(|sym| sym == '\n') {
        //         let mut is_key = true;
        //         let mut k = Vec::new();
        //         let mut v = Vec::new();
        //         for sym in line.as_bytes() {
        //             if *sym == b'=' {
        //                 is_key = false;
        //                 continue;
        //             }
        //             if is_key {
        //                 k.push(*sym);
        //             } else {
        //                 v.push(*sym);
        //             }
        //         }
        //         lin.push(
        //             CString::new(format!(
        //                 "{}={}",
        //                 String::from_utf8_lossy(&k),
        //                 String::from_utf8_lossy(&v)
        //             ))
        //             .unwrap(),
        //         );
        //         map.insert(k, v);
        //     }
        // }

        Self {
            map: vars().collect(),
        }
    }
}
