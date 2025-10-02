pub mod arg;

use crate::parser::{arg_builder::arg::Arg, context::Context, token::Token};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum ArgBuilderState {
    #[default]
    Default,

    WeakSep,
    StrongSep,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ArgBuilder {
    current_arg: Arg,
    current_token: Token,
}

impl ArgBuilder {
    pub fn apply(&mut self, byte: u8, context: &mut Context) -> anyhow::Result<Option<Arg>> {
        match byte {
            b'\'' => match context.arg_state {
                ArgBuilderState::Default => {
                    context.arg_state = ArgBuilderState::StrongSep;
                    return Ok(None);
                }
                ArgBuilderState::WeakSep => {}
                ArgBuilderState::StrongSep => {
                    context.arg_state = ArgBuilderState::Default;
                    self.current_token
                        .finish(context)
                        .map(|token| self.current_arg.push(token));
                    return Ok(self.return_if_not_empty());
                }
            },
            b'"' => match context.arg_state {
                ArgBuilderState::Default => {
                    context.arg_state = ArgBuilderState::WeakSep;
                    return Ok(None);
                }
                ArgBuilderState::WeakSep => {
                    context.arg_state = ArgBuilderState::Default;
                    self.current_token
                        .finish(context)
                        .map(|token| self.current_arg.push(token));
                    return Ok(self.return_if_not_empty());
                }
                ArgBuilderState::StrongSep => {}
            },
            _ => {}
        }

        match self.current_token.apply(byte, context)? {
            Some(token) => {
                self.current_arg.push(token);
                if !context.token_in_process {
                    Ok(self.return_if_not_empty())
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    pub fn finish(&mut self, context: &mut Context) -> anyhow::Result<Option<Arg>> {
        match context.arg_state {
            ArgBuilderState::Default => {
                self.current_token
                    .finish(context)
                    .map(|token| self.current_arg.push(token));
                Ok(self.return_if_not_empty())
            }
            ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => anyhow::bail!("Syntax error"),
        }
    }

    fn return_if_not_empty(&mut self) -> Option<Arg> {
        if self.current_arg.is_empty() {
            None
        } else {
            Some(std::mem::take(self).current_arg)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        arg_builder::{ArgBuilder, arg::Arg},
        context::Context,
        token::Token,
    };

    #[test]
    fn check_arg_builder_apply() {
        let mut arg_builder = ArgBuilder::default();
        let mut context = Context::default();

        let mut result: Vec<Arg> = "echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| arg_builder.apply(*byte, &mut context).unwrap())
            .collect();
        arg_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")])
            ]
        );
        assert_eq!(arg_builder, ArgBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Arg> = "x=100 echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| arg_builder.apply(*byte, &mut context).unwrap())
            .collect();
        arg_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![
                Arg::new(vec![Token::new_var_setter("x=100")]),
                Arg::new(vec![Token::new_default("echo")]),
                Arg::new(vec![Token::new_default("100")])
            ]
        );
        assert_eq!(arg_builder, ArgBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Arg> = "\"x=100 $x\""
            .as_bytes()
            .into_iter()
            .filter_map(|byte| arg_builder.apply(*byte, &mut context).unwrap())
            .collect();
        arg_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![Arg::new(vec![
                Token::new_default("x=100 "),
                Token::new_var_getter("x")
            ]),]
        );
        assert_eq!(arg_builder, ArgBuilder::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Arg> = "'x=100\"' $x"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| arg_builder.apply(*byte, &mut context).unwrap())
            .collect();
        arg_builder
            .finish(&mut context)
            .unwrap()
            .map(|arg| result.push(arg));

        assert_eq!(
            result,
            vec![
                Arg::new(vec![Token::new_default("x=100\"")]),
                Arg::new(vec![Token::new_var_getter("x")])
            ]
        );
        assert_eq!(arg_builder, ArgBuilder::default());
        assert_eq!(context, Context::default());

        let _result: Vec<Arg> = "echo 'x=100\" $x"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| arg_builder.apply(*byte, &mut context).unwrap())
            .collect();
        assert!(arg_builder.finish(&mut context).is_err());
    }
}
