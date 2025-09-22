use crate::{global_struct::GS, parser::Parser, program::Program};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Delimeter {
    Start,
    Pipe,
}

pub struct Pipeline {
    lines: Vec<Vec<u8>>,
    parser: Parser,
    current_line_index: usize,
    current_index: usize,
    old_index: usize,
}

impl Pipeline {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            lines: Self::slice_to_lines(input),
            parser: Parser::default(),
            current_line_index: 0,
            current_index: 0,
            old_index: 0,
        }
    }

    pub fn next(
        &mut self,
        last_output: &Vec<u8>,
        last_status: i32,
        gs: &mut GS,
    ) -> anyhow::Result<Option<(Delimeter, Program)>> {
        if self.current_line_index == self.lines.len() {
            return Ok(None);
        }
        if last_status != 0 {
            if !self.to_next_line() {
                return Ok(None);
            }
        }

        if self.current_index == self.lines[self.current_line_index].len() {
            if !self.to_next_line() {
                return Ok(None);
            }
        }

        let delimeter = match self.current_index {
            0 => Delimeter::Start,
            _ => Delimeter::Pipe,
        };

        while self.current_index < self.lines[self.current_line_index].len() {
            let byte = self.lines[self.current_line_index][self.current_index];
            self.current_index += 1;
            if byte == b'|' {
                break;
            }
        }
        let slice = &self.lines[self.current_line_index][self.old_index..self.current_index - 1];
        self.old_index = self.current_index;

        let mut tokens = self.parser.parse(slice, &mut gs.environment)?;
        let mut stdin = Vec::new();
        if delimeter == Delimeter::Pipe {
            if gs.settings.mode.xargs {
                tokens.append(&mut self.parser.parse(last_output, &mut gs.environment)?);
            } else {
                stdin = last_output.clone();
            }
        }
        if tokens.len() == 0 {
            tokens.push("nop\0".as_bytes().to_vec());
        }
        gs.environment
            .get_full_path(&mut tokens[0], &gs.default_utils)?;
        let program = Program::new(stdin, tokens.into_iter().flatten().collect());

        Ok(Some((delimeter, program)))
    }

    fn slice_to_lines(input: Vec<u8>) -> Vec<Vec<u8>> {
        let mut output = Vec::new();
        let mut buffer = Vec::new();

        for byte in input {
            match byte {
                b';' | b'\n' => {
                    buffer.push(b'\n');
                    let mut line = Vec::with_capacity(buffer.capacity());
                    std::mem::swap(&mut line, &mut buffer);
                    output.push(line);
                }
                _ => {
                    buffer.push(byte);
                }
            }
        }

        output
    }

    fn to_next_line(&mut self) -> bool {
        self.current_line_index += 1;
        self.current_index = 0;
        self.old_index = 0;
        if self.current_line_index == self.lines.len() {
            return false;
        }
        true
    }
}
