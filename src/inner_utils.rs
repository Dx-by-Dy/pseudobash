use {
    crate::{global_state::GlobalState, program_output::ProgramOutput},
    std::collections::HashMap,
};

pub struct InnerUtils {
    utils: HashMap<String, fn(args: Vec<String>, gs: &mut GlobalState) -> ProgramOutput>,
}

impl Default for InnerUtils {
    fn default() -> Self {
        let mut utils = HashMap::new();
        // index.insert(
        //     "mode".to_string(),
        //     mode as fn(Vec<String>, &mut GlobalState) -> ProgramOutput,
        // );
        utils.insert(
            "exit".to_string(),
            exit as fn(Vec<String>, &mut GlobalState) -> ProgramOutput,
        );
        utils.insert(
            "nop".to_string(),
            nop as fn(Vec<String>, &mut GlobalState) -> ProgramOutput,
        );

        Self { utils }
    }
}

impl InnerUtils {
    pub fn is_inner(&self, name: &String) -> bool {
        self.utils.contains_key(name)
    }

    pub fn execute(&self, prep_program: Vec<String>, gs: &mut GlobalState) -> ProgramOutput {
        self.utils.get(&prep_program[0]).unwrap()(prep_program, gs)
    }
}

// fn mode(args: Vec<String>, gs: &mut GlobalState) -> ProgramOutput {
//     if args.len() != 2 {
//         return ProgramOutput::new(
//             -1,
//             vec![],
//             format!("Incorrect number of arguments: {:?}", args.join(" "))
//                 .as_bytes()
//                 .to_vec(),
//         );
//     }

//     for sym in args[1].chars() {
//         match sym {
//             '-' | '+' | 'i' | 'x' => {}
//             _ => {
//                 return ProgramOutput::new(
//                     -1,
//                     vec![],
//                     format!("Wrong argument: {:?}", sym).as_bytes().to_vec(),
//                 );
//             }
//         }
//     }

//     let mut mode = true;
//     for sym in args[1].chars() {
//         match sym {
//             '-' => mode = false,
//             '+' => mode = true,
//             'i' => gs.settings.set_interactive_mode(mode),
//             'x' => gs.settings.set_xargs_mode(mode),
//             _ => unreachable!(),
//         }
//     }

//     ProgramOutput::new(0, vec![], vec![])
// }

fn exit(_args: Vec<String>, _gs: &mut GlobalState) -> ProgramOutput {
    std::process::exit(0)
}

fn nop(_args: Vec<String>, _gs: &mut GlobalState) -> ProgramOutput {
    ProgramOutput::new(0, vec![], vec![])
}
