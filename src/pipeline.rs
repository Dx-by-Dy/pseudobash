use crate::{
    global_struct::{default_utils::DefaultUtils, environment::Environment},
    parser::Parser,
    program::Program,
};

#[derive(Clone, Copy)]
pub enum Delimeter {
    Pipe,
    Seq,
}

pub struct Pipeline {
    data: Vec<u8>,
    delimeter: Delimeter,
    parser: Parser,
    current_index: usize,
    old_index: usize,
}

impl Pipeline {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            data: input,
            delimeter: Delimeter::Seq,
            parser: Parser::default(),
            current_index: 0,
            old_index: 0,
        }
    }

    pub fn next(
        &mut self,
        last_output: &mut Vec<u8>,
        environment: &mut Environment,
        default_utils: &DefaultUtils,
    ) -> anyhow::Result<Option<Program>> {
        let old_delimeter = self.delimeter;

        while self.current_index < self.data.len() {
            let byte = self.data[self.current_index];
            self.current_index += 1;
            match byte {
                b'|' => {
                    self.delimeter = Delimeter::Pipe;
                    break;
                }
                b';' => {
                    self.delimeter = Delimeter::Seq;
                    break;
                }
                _ => {}
            }
        }
        if self.current_index == self.old_index {
            return Ok(None);
        }
        if self.current_index == self.data.len() {
            self.current_index += 1
        }

        let mut tokens = self.parser.parse(
            &self.data[self.old_index..self.current_index - 1],
            environment,
        )?;
        if tokens.len() == 0 {
            tokens.push("nop\0".as_bytes().to_vec());
        }

        environment.get_full_path(&mut tokens[0], default_utils)?;
        let mut program = Program::new(tokens.into_iter().flatten().collect());

        match old_delimeter {
            Delimeter::Pipe => {
                program.add_args(
                    self.parser
                        .parse(last_output, environment)?
                        .into_iter()
                        .flatten()
                        .collect(),
                );
            }
            Delimeter::Seq => {}
        }
        self.old_index = self.current_index;

        Ok(Some(program))
    }
}
