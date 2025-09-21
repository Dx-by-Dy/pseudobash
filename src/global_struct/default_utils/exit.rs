use crate::global_struct::default_utils::DefaultUtility;

pub struct Exit {
    pub(crate) name: Vec<u8>,
    path: Vec<u8>,
}

impl DefaultUtility for Exit {
    fn execute(
        &self,
        _args: Vec<String>,
        _settings: &mut crate::global_struct::settings::Settings,
        _environment: &mut crate::global_struct::environment::Environment,
    ) -> (i32, Vec<u8>, Vec<u8>) {
        std::process::exit(0)
    }

    fn name_into_path(&self, name: &mut Vec<u8>) {
        let mut path = self.path.clone();
        path.push(b'\0');
        std::mem::swap(name, &mut path);
    }
}

impl Default for Exit {
    fn default() -> Self {
        Self {
            name: "exit".as_bytes().to_vec(),
            path: "@exit".as_bytes().to_vec(),
        }
    }
}
