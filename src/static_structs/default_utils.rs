use {
    crate::{SETTINGS, program::Program},
    std::{collections::HashMap},
};

struct DefaultUtility {
    func: fn(Vec<String>) -> anyhow::Result<Vec<u8>>,
    path: Vec<u8>,
}

impl DefaultUtility {
    fn new(func: fn(Vec<String>) -> anyhow::Result<Vec<u8>>, path: Vec<u8>) -> Self {
        Self { func, path }
    }

    fn set_path(&self, buffer: &mut Vec<u8>) {
        let mut path = self.path.clone();
        path.push(b'\0');
        std::mem::swap(buffer, &mut path);
    }
}

pub struct DefaultUtils {
    index: HashMap<Vec<u8>, DefaultUtility>,
}

impl Default for DefaultUtils {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            "ive".as_bytes().to_vec(),
            DefaultUtility::new(ive, "@ive".as_bytes().to_vec()),
        );

        Self { index: map }
    }
}

impl DefaultUtils {
    pub fn name_into_path(&self, name: &mut Vec<u8>) -> bool {
        if let Some(utility) = self.index.get(&name[..name.len() - 1]) {
            utility.set_path(name);
            return true;
        }
        false
    }

    pub fn execute(&self, program: Program) -> anyhow::Result<Vec<u8>> {
        let input = parse_input(program.get_data());
        (self.index.get(input[0].as_bytes()).unwrap().func)(input)
    }
}

fn ive(args: Vec<String>) -> anyhow::Result<Vec<u8>> {
    if args.len() != 2 {
        anyhow::bail!(format!("Incorrect number of arguments: {}", args.join(" ")))
    }

    match args[1].as_str() {
        "on" => SETTINGS
            .lock()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .set_interactive_mode(true),
        "off" => SETTINGS
            .lock()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?
            .set_interactive_mode(false),
        _ => anyhow::bail!(format!("Wrong argument: {}", args.join(" "))),
    }

    Ok(vec![])
}

fn parse_input(args: Vec<u8>) -> Vec<String> {
    let mut result = Vec::new();
    let mut buffer = Vec::new();
    for byte in args {
        match byte {
            b'\0' => {
                result.push(String::from_utf8_lossy(&buffer).to_string());
                buffer.clear();
            }
            b'@' => {}
            _ => buffer.push(byte),
        }
    }
    result
}
