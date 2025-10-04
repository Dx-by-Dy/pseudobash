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
                Arg::new_default(vec![Token::new_default("echo")]),
                Arg::new_default(vec![Token::new_default("100")])
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
