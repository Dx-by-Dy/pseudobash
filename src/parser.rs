use crate::parser::{
    context::Context,
    program_builder::{ProgramBuilder, program::Program},
};

pub mod arg_builder;
pub mod program_builder;

mod context;
mod token;

#[derive(Debug, Default, PartialEq)]
pub struct Parser {
    program_builder: ProgramBuilder,
    context: Context,
}

impl Parser {
    pub fn apply(&mut self, byte: u8) -> anyhow::Result<Option<Program>> {
        self.program_builder
            .apply(byte, &mut self.context)
            .map_err(|e| {
                std::mem::take(self);
                e
            })
    }

    pub fn finish(&mut self) -> anyhow::Result<Option<Program>> {
        self.program_builder.finish(&mut self.context).map_err(|e| {
            std::mem::take(self);
            e
        })
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        Parser, arg_builder::arg::Arg, program_builder::program::Program, token::Token,
    };

    #[test]
    fn check_program_builder_apply() {
        let mut parser = Parser::default();

        let mut result: Vec<Program> = "echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| parser.apply(*byte).unwrap())
            .collect();
        parser.finish().unwrap().map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")])
            ]),]
        );
        assert_eq!(parser, Parser::default());

        let _result: Vec<Program> = "echo '100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| parser.apply(*byte).unwrap())
            .collect();
        assert!(parser.finish().is_err());
        assert_eq!(parser, Parser::default());
    }
}

// #[cfg(test)]
// mod test {
//     use crate::parser::{Parser, program_builder::program::Program};

//     #[test]
//     fn check_parser() {
//         let mut parser = Parser::default();

//         let result: Vec<Program> = "echo 100\n"
//             .as_bytes()
//             .into_iter()
//             .filter_map(|byte| parser.apply(*byte).unwrap())
//             .collect();
//         assert_eq!(result, vec![Program {}]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }
// }

//         let output = parser
//             .parse("  echo 1278 echo\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(
//             output,
//             vec![
//                 "echo\0".as_bytes(),
//                 "1278\0".as_bytes(),
//                 "echo\0".as_bytes()
//             ]
//         );
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  \necho\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output, vec!["echo\0".as_bytes(),]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  \necho".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output, vec!["echo\0".as_bytes(),]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  \necho\n\n\0".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output, vec!["echo\0".as_bytes(),]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse("\n".as_bytes(), &mut environment).unwrap();
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse(" \n ".as_bytes(), &mut environment).unwrap();
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse("echo @\n".as_bytes(), &mut environment);
//         assert!(output.is_err());
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }

//     #[test]
//     fn check_var_setter() {
//         let mut environment = Environment::default();
//         let mut parser = Parser::default();

//         let output = parser
//             .parse("  qwe=1278\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(environment.get_var("qwe").unwrap(), "1278".as_bytes());
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  qwe==10\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(environment.get_var("qwe").unwrap(), "=10".as_bytes());
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("qwe=qwe \n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(environment.get_var("qwe").unwrap(), "qwe".as_bytes());
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse("qwe=\n".as_bytes(), &mut environment).unwrap();
//         assert_eq!(environment.get_var("qwe").unwrap(), "".as_bytes());
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("qwe='10$10'\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(environment.get_var("qwe").unwrap(), "10$10".as_bytes());
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }

//     #[test]
//     fn check_var_getter() {
//         let mut environment = Environment::default();
//         let mut parser = Parser::default();

//         let output = parser
//             .parse("  $PWD\n".as_bytes(), &mut environment)
//             .unwrap();
//         let mut target = environment.get_var("PWD").unwrap().clone();
//         target.push(b'\0');
//         assert_eq!(output, vec![target]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  $PWD $PWD\n".as_bytes(), &mut environment)
//             .unwrap();
//         let mut target = environment.get_var("PWD").unwrap().clone();
//         target.push(b'\0');
//         assert_eq!(output, vec![target.clone(), target]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  $PWD$PWD\n".as_bytes(), &mut environment)
//             .unwrap();
//         let mut target = environment.get_var("PWD").unwrap().clone();
//         target.append(&mut environment.get_var("PWD").unwrap().clone());
//         target.push(b'\0');
//         assert_eq!(output, vec![target]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  $NOTPWD\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output.len(), 0);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }

//     #[test]
//     fn check_strong_sep() {
//         let mut environment = Environment::default();
//         let mut parser = Parser::default();

//         let output = parser.parse("'1'\n".as_bytes(), &mut environment).unwrap();
//         assert_eq!(output, vec!["1\0".as_bytes()]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  '$PWD'\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output, vec!["$PWD\0".as_bytes()]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("  '$PWD\"'\n".as_bytes(), &mut environment)
//             .unwrap();
//         assert_eq!(output, vec!["$PWD\"\0".as_bytes()]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse("  '$PWD\"\n".as_bytes(), &mut environment);
//         assert!(output.is_err());
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }

//     #[test]
//     fn check_weak_sep() {
//         let mut environment = Environment::default();
//         let mut parser = Parser::default();

//         let output = parser
//             .parse("\"$PWD\"\n".as_bytes(), &mut environment)
//             .unwrap();
//         let mut target = environment.get_var("PWD").unwrap().clone();
//         target.push(b'\0');
//         assert_eq!(output, vec![target]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser
//             .parse("'$PWD'\"$PWD\"\n".as_bytes(), &mut environment)
//             .unwrap();
//         let mut target = environment.get_var("PWD").unwrap().clone();
//         target.push(b'\0');
//         let mut t = "$PWD".as_bytes().to_vec();
//         t.append(&mut target);
//         assert_eq!(output, vec![t]);
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);

//         let output = parser.parse("\"$PWD\n".as_bytes(), &mut environment);
//         assert!(output.is_err());
//         assert_eq!(parser.buffer.len(), 0);
//         assert_eq!(parser.current_state, Some(ParserState::default()));
//         assert_eq!(parser.state_stack.len(), 0);
//     }
// }
