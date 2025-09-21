use crate::global_struct::default_utils::DefaultUtility;

pub struct Ive {
    pub(crate) name: Vec<u8>,
    path: Vec<u8>,
}

impl DefaultUtility for Ive {
    fn execute(
        &self,
        args: Vec<String>,
        settings: &mut crate::global_struct::settings::Settings,
        _environment: &mut crate::global_struct::environment::Environment,
    ) -> (i32, Vec<u8>, Vec<u8>) {
        if args.len() != 2 {
            return (
                -1,
                vec![],
                format!("Incorrect number of arguments: {}", args.join(" "))
                    .as_bytes()
                    .to_vec(),
            );
        }

        match args[1].as_str() {
            "on" => settings.set_interactive_mode(true),
            "off" => settings.set_interactive_mode(false),
            _ => {
                return (
                    -1,
                    vec![],
                    format!("Wrong argument: {}", args.join(" "))
                        .as_bytes()
                        .to_vec(),
                );
            }
        }

        (0, vec![], vec![])
    }

    fn name_into_path(&self, name: &mut Vec<u8>) {
        let mut path = self.path.clone();
        path.push(b'\0');
        std::mem::swap(name, &mut path);
    }
}

impl Default for Ive {
    fn default() -> Self {
        Self {
            name: "ive".as_bytes().to_vec(),
            path: "@ive".as_bytes().to_vec(),
        }
    }
}
