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
