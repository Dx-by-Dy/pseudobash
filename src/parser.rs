use crate::global_struct::environment::Environment;

enum ParserStateAns {
    Nop,
    Pop(bool),
    Push(ParserState),
    Ready,
}

#[derive(Default, Debug, PartialEq)]
enum ParserState {
    #[default]
    Default,

    VarSetter(Vec<u8>),
    VarGetter(usize),
    StrongSep,
    WeakSep,
}

impl ParserState {
    fn change_by(
        &self,
        buffer: &mut Vec<u8>,
        input: u8,
        environment: &mut Environment,
    ) -> anyhow::Result<ParserStateAns> {
        match self {
            ParserState::Default => Self::default_by(buffer, input),
            ParserState::VarSetter(name) => Self::var_setter_by(name, buffer, input, environment),
            ParserState::VarGetter(idx) => Self::var_getter_by(*idx, buffer, input, environment),
            ParserState::StrongSep => Self::strong_sep(buffer, input),
            ParserState::WeakSep => Self::weak_sep(buffer, input),
        }
    }

    fn default_by(buffer: &mut Vec<u8>, input: u8) -> anyhow::Result<ParserStateAns> {
        match input {
            b' ' | b'\n' | b'\0' => {
                if buffer.len() > 0 {
                    if *buffer.last().unwrap() != b'\0' {
                        buffer.push(b'\0');
                    }
                    Ok(ParserStateAns::Ready)
                } else {
                    Ok(ParserStateAns::Nop)
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'/' | b'_' | b':' | b'.' => {
                buffer.push(input);
                Ok(ParserStateAns::Nop)
            }
            b'=' => {
                let mut var_name = Vec::with_capacity(buffer.len());
                std::mem::swap(buffer, &mut var_name);
                Ok(ParserStateAns::Push(ParserState::VarSetter(var_name)))
            }
            b'$' => Ok(ParserStateAns::Push(ParserState::VarGetter(buffer.len()))),
            b'\'' => Ok(ParserStateAns::Push(ParserState::StrongSep)),
            b'"' => Ok(ParserStateAns::Push(ParserState::WeakSep)),
            _ => {
                anyhow::bail!(format!("Unknown symbol: {:?}", input as char))
            }
        }
    }

    fn var_setter_by(
        name: &Vec<u8>,
        buffer: &mut Vec<u8>,
        input: u8,
        environment: &mut Environment,
    ) -> anyhow::Result<ParserStateAns> {
        match input {
            b' ' | b'\n' | b'\0' => {
                let mut var_value = Vec::with_capacity(buffer.len());
                std::mem::swap(buffer, &mut var_value);
                environment.set_var(name.clone(), var_value);
                Ok(ParserStateAns::Pop(false))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'/' | b'_' | b':' | b'=' | b'.' => {
                buffer.push(input);
                Ok(ParserStateAns::Nop)
            }
            b'$' => Ok(ParserStateAns::Push(ParserState::VarGetter(buffer.len()))),
            b'\'' => Ok(ParserStateAns::Push(ParserState::StrongSep)),
            b'"' => Ok(ParserStateAns::Push(ParserState::WeakSep)),
            _ => {
                anyhow::bail!(format!("Unknown symbol: {:?}", input as char))
            }
        }
    }

    fn var_getter_by(
        index: usize,
        buffer: &mut Vec<u8>,
        input: u8,
        environment: &mut Environment,
    ) -> anyhow::Result<ParserStateAns> {
        match input {
            b' ' | b'\n' | b'\0' | b'$' | b':' | b'=' | b'"' | b'\'' => {
                match environment.get_var(str::from_utf8(&buffer[index..])?) {
                    Some(value) => {
                        let mut it_val = value.into_iter();
                        for i in 0..it_val.len().max(buffer.len() - index) {
                            let new_val = *it_val.next().unwrap_or(&b'\0');
                            if index + i < buffer.len() {
                                buffer[index + i] = new_val
                            } else {
                                buffer.push(new_val);
                            }
                        }
                    }
                    None => {
                        for i in index..buffer.len() {
                            buffer[i] = b'\0'
                        }
                    }
                }
                while buffer.last().is_some_and(|byte| *byte == b'\0') {
                    buffer.pop();
                }
                Ok(ParserStateAns::Pop(false))
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'/' | b'_' | b'.' => {
                buffer.push(input);
                Ok(ParserStateAns::Nop)
            }
            _ => {
                anyhow::bail!(format!("Unknown symbol: {:?}", input as char))
            }
        }
    }

    fn strong_sep(buffer: &mut Vec<u8>, input: u8) -> anyhow::Result<ParserStateAns> {
        match input {
            b'\'' => Ok(ParserStateAns::Pop(true)),
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'/'
            | b'_'
            | b':'
            | b' '
            | b'\n'
            | b'\0'
            | b'$'
            | b'='
            | b'"'
            | b'.' => {
                buffer.push(input);
                Ok(ParserStateAns::Nop)
            }
            _ => {
                anyhow::bail!(format!("Unknown symbol: {:?}", input as char))
            }
        }
    }

    fn weak_sep(buffer: &mut Vec<u8>, input: u8) -> anyhow::Result<ParserStateAns> {
        match input {
            b'"' => Ok(ParserStateAns::Pop(true)),
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'/'
            | b'_'
            | b':'
            | b' '
            | b'\n'
            | b'\0'
            | b'='
            | b'.' => {
                buffer.push(input);
                Ok(ParserStateAns::Nop)
            }
            b'$' => Ok(ParserStateAns::Push(ParserState::VarGetter(buffer.len()))),
            b'\'' => Ok(ParserStateAns::Push(ParserState::StrongSep)),
            _ => {
                anyhow::bail!(format!("Unknown symbol: {:?}", input as char))
            }
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    buffer: Vec<u8>,
    current_state: Option<ParserState>,
    state_stack: Vec<ParserState>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            buffer: Default::default(),
            current_state: Some(ParserState::default()),
            state_stack: Default::default(),
        }
    }
}

impl Parser {
    pub fn parse(
        &mut self,
        input: &[u8],
        environment: &mut Environment,
    ) -> anyhow::Result<Vec<Vec<u8>>> {
        let mut output = Vec::new();

        for byte in input {
            while self.apply_byte(*byte, &mut output, environment)? != true {}
        }
        while self.apply_byte(b'\0', &mut output, environment)? != true {}

        if self.state_stack.len() != 0 {
            self.buffer.clear();
            self.current_state = Some(ParserState::default());
            self.state_stack.clear();

            anyhow::bail!(format!(
                "Syntax error: {:?}",
                String::from_utf8_lossy(input)
            ))
        }
        Ok(output)
    }

    fn apply_byte(
        &mut self,
        byte: u8,
        output: &mut Vec<Vec<u8>>,
        environment: &mut Environment,
    ) -> anyhow::Result<bool> {
        match self
            .current_state
            .as_ref()
            .unwrap()
            .change_by(&mut self.buffer, byte, environment)?
        {
            ParserStateAns::Nop => {}
            ParserStateAns::Pop(value) => {
                self.current_state = self.state_stack.pop();
                return Ok(value);
            }
            ParserStateAns::Push(parser_state) => self
                .state_stack
                .push(self.current_state.replace(parser_state).unwrap()),
            ParserStateAns::Ready => {
                let mut token = Vec::with_capacity(self.buffer.capacity());
                std::mem::swap(&mut token, &mut self.buffer);
                output.push(token);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        global_struct::environment::Environment,
        parser::{Parser, ParserState},
    };

    #[test]
    fn check_parser() {
        let mut environment = Environment::default();
        let mut parser = Parser::default();

        let output = parser
            .parse("  echo 1278\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["echo\0".as_bytes(), "1278\0".as_bytes()]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  echo 1278 echo\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(
            output,
            vec![
                "echo\0".as_bytes(),
                "1278\0".as_bytes(),
                "echo\0".as_bytes()
            ]
        );
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  \necho\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["echo\0".as_bytes(),]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  \necho".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["echo\0".as_bytes(),]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  \necho\n\n\0".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["echo\0".as_bytes(),]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse("\n".as_bytes(), &mut environment).unwrap();
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse(" \n ".as_bytes(), &mut environment).unwrap();
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse("echo @\n".as_bytes(), &mut environment);
        assert!(output.is_err());
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);
    }

    #[test]
    fn check_var_setter() {
        let mut environment = Environment::default();
        let mut parser = Parser::default();

        let output = parser
            .parse("  qwe=1278\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(environment.get_var("qwe").unwrap(), "1278".as_bytes());
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  qwe==10\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(environment.get_var("qwe").unwrap(), "=10".as_bytes());
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("qwe=qwe \n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(environment.get_var("qwe").unwrap(), "qwe".as_bytes());
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse("qwe=\n".as_bytes(), &mut environment).unwrap();
        assert_eq!(environment.get_var("qwe").unwrap(), "".as_bytes());
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("qwe='10$10'\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(environment.get_var("qwe").unwrap(), "10$10".as_bytes());
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);
    }

    #[test]
    fn check_var_getter() {
        let mut environment = Environment::default();
        let mut parser = Parser::default();

        let output = parser
            .parse("  $PWD\n".as_bytes(), &mut environment)
            .unwrap();
        let mut target = environment.get_var("PWD").unwrap().clone();
        target.push(b'\0');
        assert_eq!(output, vec![target]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  $PWD $PWD\n".as_bytes(), &mut environment)
            .unwrap();
        let mut target = environment.get_var("PWD").unwrap().clone();
        target.push(b'\0');
        assert_eq!(output, vec![target.clone(), target]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  $PWD$PWD\n".as_bytes(), &mut environment)
            .unwrap();
        let mut target = environment.get_var("PWD").unwrap().clone();
        target.append(&mut environment.get_var("PWD").unwrap().clone());
        target.push(b'\0');
        assert_eq!(output, vec![target]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  $NOTPWD\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output.len(), 0);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);
    }

    #[test]
    fn check_strong_sep() {
        let mut environment = Environment::default();
        let mut parser = Parser::default();

        let output = parser.parse("'1'\n".as_bytes(), &mut environment).unwrap();
        assert_eq!(output, vec!["1\0".as_bytes()]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  '$PWD'\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["$PWD\0".as_bytes()]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("  '$PWD\"'\n".as_bytes(), &mut environment)
            .unwrap();
        assert_eq!(output, vec!["$PWD\"\0".as_bytes()]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse("  '$PWD\"\n".as_bytes(), &mut environment);
        assert!(output.is_err());
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);
    }

    #[test]
    fn check_weak_sep() {
        let mut environment = Environment::default();
        let mut parser = Parser::default();

        let output = parser
            .parse("\"$PWD\"\n".as_bytes(), &mut environment)
            .unwrap();
        let mut target = environment.get_var("PWD").unwrap().clone();
        target.push(b'\0');
        assert_eq!(output, vec![target]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser
            .parse("'$PWD'\"$PWD\"\n".as_bytes(), &mut environment)
            .unwrap();
        let mut target = environment.get_var("PWD").unwrap().clone();
        target.push(b'\0');
        let mut t = "$PWD".as_bytes().to_vec();
        t.append(&mut target);
        assert_eq!(output, vec![t]);
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);

        let output = parser.parse("\"$PWD\n".as_bytes(), &mut environment);
        assert!(output.is_err());
        assert_eq!(parser.buffer.len(), 0);
        assert_eq!(parser.current_state, Some(ParserState::default()));
        assert_eq!(parser.state_stack.len(), 0);
    }
}
