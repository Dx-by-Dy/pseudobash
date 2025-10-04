use {
    crate::{
        global_state::GlobalState,
        inner_utils::InnerUtils,
        listener::Listener,
        parser::{Parser, program_builder::program::Program},
        program_output::ProgramOutput,
    },
    std::io::Write,
};

#[derive(Default)]
pub struct CLI {
    listener: Listener,
    parser: Parser,
    inner_utils: InnerUtils,
    global_state: GlobalState,
}

impl CLI {
    pub fn start(&mut self) {
        loop {
            print!("{} ", self.global_state.settings.get_invitation_input());
            std::io::stdout().flush().unwrap();

            let _: Vec<()> = self
                .parse(self.listener.listen())
                .into_iter()
                .map(|program| {
                    Self::print_output(program.execute(&mut self.global_state, &self.inner_utils))
                })
                .collect();
        }
    }

    fn parse(&mut self, input: String) -> Vec<Program> {
        let mut result = Vec::new();
        if input.len() > 1 {
            for byte in input.as_bytes() {
                match self.parser.apply(*byte) {
                    Ok(Some(program)) => result.push(program),
                    Ok(None) => {}
                    Err(e) => eprintln!("Parser error: {}", e),
                }
            }
            match self.parser.finish() {
                Ok(Some(program)) => result.push(program),
                Ok(None) => {}
                Err(e) => eprintln!("Parser error: {}", e),
            }
        }
        result
    }

    fn print_output(output: anyhow::Result<ProgramOutput>) {
        match output {
            Ok(program_output) => match program_output.code {
                0 => print!("{}", String::from_utf8_lossy(&program_output.stdout)),
                _ => eprintln!(
                    "Program exited with code {}. Error: {}",
                    program_output.code,
                    String::from_utf8_lossy(&program_output.stderr)
                ),
            },
            Err(e) => eprintln!("Executing error: {}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{cli::CLI, program_output::ProgramOutput};

    #[test]
    fn check_var_setter() {
        let mut cli: CLI = CLI::default();

        let output: Vec<ProgramOutput> = cli
            .parse("  qwe=1278\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "1278".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("  qwe==10\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "=10".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("  qwe=qwe\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "qwe".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("  qwe=\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("  qwe='10$10'\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "qwe".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, "10$10".as_bytes().to_vec());
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("x=$PWD\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut x = "x".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut x);
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        assert_eq!(var, x);
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse("x=$PWD:9\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut x = "x".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut x);
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b':');
        var.push(b'9');
        assert_eq!(var, x);
        assert_eq!(output, vec![ProgramOutput::new(0, vec![], vec![])]);
    }

    #[test]
    fn check_var_getter() {
        let mut cli: CLI = CLI::default();

        let output: Vec<ProgramOutput> = cli
            .parse(" echo $PWD\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse(" echo $PWD $PWD\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.push(b' ');
        var.append(&mut var.clone());
        var.last_mut().map(|byte| *byte = b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse(" echo $PWD$PWD\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        let mut var = "PWD".as_bytes().to_vec();
        cli.global_state.environment.get_var(&mut var);
        var.append(&mut var.clone());
        var.push(b'\n');
        assert_eq!(output, vec![ProgramOutput::new(0, var, vec![])]);

        let output: Vec<ProgramOutput> = cli
            .parse(" echo $PWDPWD\n".to_string())
            .into_iter()
            .map(|program| {
                program
                    .execute(&mut cli.global_state, &cli.inner_utils)
                    .unwrap()
            })
            .collect();
        assert_eq!(
            output,
            vec![ProgramOutput::new(0, "\n".as_bytes().to_vec(), vec![])]
        );
    }

    #[test]
    fn check_error() {
        let mut cli: CLI = CLI::default();

        let output: Vec<anyhow::Result<ProgramOutput>> = cli
            .parse("  '1'\n".to_string())
            .into_iter()
            .map(|program| program.execute(&mut cli.global_state, &cli.inner_utils))
            .collect();
        assert!(output.iter().all(|res| res.is_err()));
    }
}
