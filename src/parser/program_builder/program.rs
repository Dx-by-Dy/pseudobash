use crate::{
    global_state::GlobalState, inner_utils::InnerUtils, parser::arg_builder::arg::Arg,
    program_output::ProgramOutput,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Program {
    args: Vec<Arg>,
}

impl Program {
    pub fn execute(self, gs: &mut GlobalState, iu: &InnerUtils) -> anyhow::Result<ProgramOutput> {
        let prep_program = self.prepare(gs);
        if prep_program.len() == 0 {
            return Ok(ProgramOutput::new(0, vec![], vec![]));
        }

        if prep_program.first().is_some_and(|name| iu.is_inner(name)) {
            Ok(iu.execute(prep_program, gs))
        } else {
            let mut command = std::process::Command::new(&prep_program[0]);
            for (idx, arg) in prep_program.iter().enumerate() {
                if idx > 0 {
                    command.arg(arg);
                }
            }

            command.env_clear();
            for (k, v) in gs.environment.vars() {
                command.env(k, v);
            }

            Ok(command
                .output()
                .map_err(|e| {
                    anyhow::Error::msg(format!("{}: '{}'", e.to_string(), prep_program.join(" ")))
                })?
                .into())
        }
    }

    pub fn push(&mut self, arg: Arg) {
        self.args.push(arg);
    }

    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }

    fn prepare(self, gs: &mut GlobalState) -> Vec<String> {
        self.args
            .into_iter()
            .map(|arg| arg.into_string_with_executing(gs))
            .filter(|arg| arg.len() > 0)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{arg_builder::arg::Arg, program_builder::program::Program};

    impl Program {
        pub fn new(args: Vec<Arg>) -> Self {
            Self { args }
        }
    }
}
