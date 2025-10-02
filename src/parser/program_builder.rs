pub mod program;

use crate::parser::{
    arg_builder::{ArgBuilder, ArgBuilderState},
    context::Context,
    program_builder::program::Program,
};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ProgramBuilder {
    current_program: Program,
    arg_builder: ArgBuilder,
}

impl ProgramBuilder {
    pub fn apply(&mut self, byte: u8, context: &mut Context) -> anyhow::Result<Option<Program>> {
        match byte {
            // b'|' => {
            //     context.current_arg_index = 0;
            //     self.data.delimeter = Delimeter::Pipe;
            //     return Ok(Some(std::mem::take(self).data));
            // }
            b';' => match context.arg_state {
                ArgBuilderState::Default => return self.finish(context),
                ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => {}
            },
            _ => {}
        }

        self.arg_builder.apply(byte, context)?.map(|arg| {
            self.current_program.push(arg);
        });
        Ok(None)
    }

    pub fn finish(&mut self, context: &mut Context) -> anyhow::Result<Option<Program>> {
        self.arg_builder.finish(context)?.map(|arg| {
            self.current_program.push(arg);
        });
        Ok(self.return_if_not_empty())
    }

    fn return_if_not_empty(&mut self) -> Option<Program> {
        if self.current_program.is_empty() {
            None
        } else {
            Some(std::mem::take(&mut self.current_program))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        arg_builder::arg::Arg,
        context::Context,
        program_builder::{ProgramBuilder, program::Program},
        token::Token,
    };

    #[test]
    fn check_program_builder_apply() {
        let mut program_builder = ProgramBuilder::default();
        let mut context = Context::default();

        let mut result: Vec<Program> = "echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "x=100 echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_var_setter("x=100")]),
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")]),
                Arg::new(vec![Token::new_default("200")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200;"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")]),
                Arg::new(vec![Token::new_default("200")])
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200; echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![
                Program::new(vec![
                    Arg::new(vec![Token::new_default("echo")]),
                    Arg::new(vec![Token::new_default("100")]),
                    Arg::new(vec![Token::new_default("200")])
                ]),
                Program::new(vec![
                    Arg::new(vec![Token::new_default("echo")]),
                    Arg::new(vec![Token::new_default("100")]),
                ])
            ]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo '100 200; echo 100'"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100 200; echo 100")]),
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo 100 200; echo 100; echo 300"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![
                Program::new(vec![
                    Arg::new(vec![Token::new_default("echo")]),
                    Arg::new(vec![Token::new_default("100")]),
                    Arg::new(vec![Token::new_default("200")]),
                ]),
                Program::new(vec![
                    Arg::new(vec![Token::new_default("echo")]),
                    Arg::new(vec![Token::new_default("100")]),
                ]),
                Program::new(vec![
                    Arg::new(vec![Token::new_default("echo")]),
                    Arg::new(vec![Token::new_default("300")]),
                ])
            ]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "echo $x"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_var_getter("x")]),
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Program> = "x=100 x=100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| program_builder.apply(*byte, &mut context).unwrap())
            .collect();
        program_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Program::new(vec![
                Arg::new(vec![Token::new_var_setter("x=100")]),
                Arg::new(vec![Token::new_var_setter("x=100")]),
            ]),]
        );
        assert_eq!(program_builder, ProgramBuilder::default());
        assert_eq!(context, Context::default());
    }
}
