use crate::global_struct::default_utils::DefaultUtility;

pub struct Nop {
    pub(crate) name: Vec<u8>,
    path: Vec<u8>,
}

impl DefaultUtility for Nop {
    fn execute(
        &self,
        _args: Vec<String>,
        _settings: &mut crate::global_struct::settings::Settings,
        _environment: &mut crate::global_struct::environment::Environment,
    ) -> anyhow::Result<Vec<u8>> {
        Ok(vec![])
    }

    fn name_into_path(&self, name: &mut Vec<u8>) {
        let mut path = self.path.clone();
        path.push(b'\0');
        std::mem::swap(name, &mut path);
    }
}

impl Default for Nop {
    fn default() -> Self {
        Self {
            name: "nop".as_bytes().to_vec(),
            path: "@nop".as_bytes().to_vec(),
        }
    }
}
