use crate::global_struct::default_utils::DefaultUtility;

pub struct Mode {
    pub(crate) name: Vec<u8>,
    path: Vec<u8>,
}

impl DefaultUtility for Mode {
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
                format!("Incorrect number of arguments: {:?}", args.join(" "))
                    .as_bytes()
                    .to_vec(),
            );
        }

        for sym in args[1].chars() {
            match sym {
                '-' | '+' | 'i' | 'x' => {}
                _ => {
                    return (
                        -1,
                        vec![],
                        format!("Wrong argument: {:?}", sym).as_bytes().to_vec(),
                    );
                }
            }
        }

        let mut mode = true;
        for sym in args[1].chars() {
            match sym {
                '-' => mode = false,
                '+' => mode = true,
                'i' => settings.set_interactive_mode(mode),
                'x' => settings.set_xargs_mode(mode),
                _ => unreachable!(),
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

impl Default for Mode {
    fn default() -> Self {
        Self {
            name: "mode".as_bytes().to_vec(),
            path: "@mode".as_bytes().to_vec(),
        }
    }
}
