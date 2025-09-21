mod exit;
mod ive;
mod nop;

use {
    crate::{
        global_struct::{
            default_utils::{exit::Exit, ive::Ive, nop::Nop},
            environment::Environment,
            settings::Settings,
        },
        program::Program,
    },
    std::collections::HashMap,
};

trait DefaultUtility {
    fn execute(
        &self,
        args: Vec<String>,
        settings: &mut Settings,
        environment: &mut Environment,
    ) -> (i32, Vec<u8>, Vec<u8>);

    fn name_into_path(&self, name: &mut Vec<u8>);
}

pub struct DefaultUtils {
    index: HashMap<Vec<u8>, Box<dyn DefaultUtility>>,
}

impl Default for DefaultUtils {
    fn default() -> Self {
        let mut index = HashMap::new();

        let ive = Ive::default();
        index.insert(ive.name.clone(), Box::new(ive) as Box<dyn DefaultUtility>);
        let nop = Nop::default();
        index.insert(nop.name.clone(), Box::new(nop) as Box<dyn DefaultUtility>);
        let exit = Exit::default();
        index.insert(exit.name.clone(), Box::new(exit) as Box<dyn DefaultUtility>);

        Self { index }
    }
}

impl DefaultUtils {
    pub fn name_into_path(&self, name: &mut Vec<u8>) -> bool {
        if let Some(utility) = self.index.get(&name[..name.len() - 1]) {
            utility.name_into_path(name);
            return true;
        }
        false
    }

    pub fn execute(
        &self,
        program: Program,
        settings: &mut Settings,
        environment: &mut Environment,
    ) -> (i32, Vec<u8>, Vec<u8>) {
        let input = parse_input(program.get_data());
        self.index
            .get(input[0].as_bytes())
            .expect("Not default method")
            .execute(input, settings, environment)
    }
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
