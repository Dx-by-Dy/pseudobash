use std::{collections::HashMap, ffi::CString, fs::File, io::BufReader};

pub struct Utils {
    utils: Vec<Utility>,
    index: HashMap<Vec<u8>, usize>,
}

impl Utils {
    pub fn new(path: &CString) -> anyhow::Result<Self> {
        let mut utils = Self::default_utils();
        utils.append(
            &mut serde_json::from_reader::<BufReader<File>, Vec<Utility>>(BufReader::new(
                File::open(path.to_str().expect("Bad utils path"))
                    .expect("Failed to open utils file"),
            ))
            .expect("Failed to read utilsfile"),
        );
        let mut index = HashMap::new();
        for (idx, utility) in utils.iter().enumerate() {
            match index.insert(utility.name.clone().as_bytes_with_nul().to_vec(), idx) {
                Some(old_idx) => {
                    if utils[old_idx].path != utils[idx].path {
                        anyhow::bail!(format!(
                            "Names repetition: ({}, {}) ({}, {})",
                            utils[old_idx].name.to_string_lossy(),
                            utils[old_idx].path.to_string_lossy(),
                            utils[idx].name.to_string_lossy(),
                            utils[idx].path.to_string_lossy()
                        ))
                    }
                }
                None => {}
            }
            index.insert(utility.path.clone().as_bytes_with_nul().to_vec(), idx);
        }

        Ok(Self { utils, index })
    }

    pub fn get_full_path<'a>(&self, name: &'a mut Vec<u8>) -> anyhow::Result<&'a mut Vec<u8>> {
        let utility = &self.utils[*self.index.get(name).ok_or(anyhow::Error::msg(format!(
            "Unknown command: {:?}",
            String::from_utf8_lossy(&name),
        )))?];
        let mut path = utility.path.as_bytes_with_nul().to_vec();

        name.clear();
        name.append(&mut path);
        Ok(name)
    }

    fn default_utils() -> Vec<Utility> {
        vec![Utility {
            name: CString::new("ive").unwrap(),
            path: CString::new("@ive").unwrap(),
        }]
    }
}

#[derive(serde::Deserialize)]
struct Utility {
    name: CString,
    path: CString,
}
