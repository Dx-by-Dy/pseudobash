use crate::{
    global_state::GlobalState,
    parser::{arg_builder::ArgBuilderState, context::Context},
};

#[derive(Debug, Default, PartialEq, Eq)]
enum TokenType {
    #[default]
    Default,

    VarSetter,
    VarGetter,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Token {
    buffer: Vec<u8>,
    kind: TokenType,
}

impl Token {
    pub fn apply(&mut self, byte: u8, context: &mut Context) -> anyhow::Result<Option<Self>> {
        match byte {
            b' ' | b'\n' | b'\0' => {
                if self.buffer.len() > 0 {
                    match context.arg_state {
                        ArgBuilderState::Default => {
                            context.token_in_process = false;
                            Ok(Some(std::mem::take(self)))
                        }
                        ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => {
                            context.token_in_process = true;
                            self.buffer.push(byte);
                            Ok(None)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            b'a'..=b'z'
            | b'A'..=b'Z'
            | b'0'..=b'9'
            | b'-'
            | b'+'
            | b'/'
            | b'_'
            | b':'
            | b'.'
            | b'\''
            | b'"'
            | b';' => {
                context.token_in_process = true;
                self.buffer.push(byte);
                Ok(None)
            }
            b'=' => {
                context.token_in_process = true;
                match context.arg_state {
                    ArgBuilderState::Default => {
                        if self.kind == TokenType::VarGetter {
                            let output = self.return_if_not_empty();
                            self.buffer.push(byte);
                            return Ok(output);
                        }
                        self.kind = TokenType::VarSetter;
                        self.buffer.push(byte);
                        Ok(None)
                    }
                    ArgBuilderState::WeakSep | ArgBuilderState::StrongSep => {
                        self.buffer.push(byte);
                        Ok(None)
                    }
                }
            }
            b'$' => {
                context.token_in_process = true;
                match context.arg_state {
                    ArgBuilderState::Default | ArgBuilderState::WeakSep => {
                        if self.kind == TokenType::VarSetter {
                            self.buffer.push(byte);
                            return Ok(None);
                        }
                        let output = self.return_if_not_empty();
                        self.kind = TokenType::VarGetter;
                        Ok(output)
                    }
                    ArgBuilderState::StrongSep => {
                        self.buffer.push(byte);
                        Ok(None)
                    }
                }
            }
            _ => {
                anyhow::bail!(format!("Unexpected symbol: {:?}", byte as char))
            }
        }
    }

    pub fn finish(&mut self, context: &mut Context) -> Option<Self> {
        context.token_in_process = false;
        if self.buffer.len() > 0 {
            Some(std::mem::take(self))
        } else {
            std::mem::take(self);
            None
        }
    }

    pub fn downgrade(self) -> Vec<u8> {
        self.buffer
    }

    pub fn to_default(&mut self, gs: &mut GlobalState) {
        match self.kind {
            TokenType::Default => {}
            TokenType::VarSetter => gs.environment.set_var(std::mem::take(self).buffer),
            TokenType::VarGetter => {
                gs.environment.get_var(&mut self.buffer);
                self.kind = TokenType::Default
            }
        }
    }

    fn return_if_not_empty(&mut self) -> Option<Token> {
        if self.buffer.len() == 0 {
            None
        } else {
            Some(std::mem::take(self))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        context::Context,
        token::{Token, TokenType},
    };

    impl Token {
        pub fn new_default(buffer: &str) -> Self {
            Self {
                buffer: buffer.as_bytes().to_vec(),
                kind: TokenType::Default,
            }
        }

        pub fn new_var_setter(buffer: &str) -> Self {
            Self {
                buffer: buffer.as_bytes().to_vec(),
                kind: TokenType::VarSetter,
            }
        }

        pub fn new_var_getter(buffer: &str) -> Self {
            Self {
                buffer: buffer.as_bytes().to_vec(),
                kind: TokenType::VarGetter,
            }
        }
    }

    #[test]
    fn check_token_apply() {
        let mut token = Token::default();
        let mut context = Context::default();

        let mut result: Vec<Token> = "echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| token.apply(*byte, &mut context).unwrap())
            .collect();
        token.finish(&mut context).map(|token| result.push(token));

        assert_eq!(
            result,
            vec![Token::new_default("echo"), Token::new_default("100"),]
        );
        assert_eq!(token, Token::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Token> = "x=100 echo 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| token.apply(*byte, &mut context).unwrap())
            .collect();
        token.finish(&mut context).map(|token| result.push(token));

        assert_eq!(
            result,
            vec![
                Token::new_var_setter("x=100"),
                Token::new_default("echo"),
                Token::new_default("100")
            ]
        );
        assert_eq!(token, Token::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Token> = "  echo 100   200  "
            .as_bytes()
            .into_iter()
            .filter_map(|byte| token.apply(*byte, &mut context).unwrap())
            .collect();
        token.finish(&mut context).map(|token| result.push(token));

        assert_eq!(
            result,
            vec![
                Token::new_default("echo"),
                Token::new_default("100"),
                Token::new_default("200"),
            ]
        );
        assert_eq!(token, Token::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Token> = "x=100 $x"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| token.apply(*byte, &mut context).unwrap())
            .collect();
        token.finish(&mut context).map(|token| result.push(token));

        assert_eq!(
            result,
            vec![Token::new_var_setter("x=100"), Token::new_var_getter("x"),]
        );
        assert_eq!(token, Token::default());
        assert_eq!(context, Context::default());

        let mut result: Vec<Token> = "x=100 $x 100"
            .as_bytes()
            .into_iter()
            .filter_map(|byte| token.apply(*byte, &mut context).unwrap())
            .collect();
        token.finish(&mut context).map(|token| result.push(token));

        assert_eq!(
            result,
            vec![
                Token::new_var_setter("x=100"),
                Token::new_var_getter("x"),
                Token::new_default("100"),
            ]
        );
        assert_eq!(token, Token::default());
        assert_eq!(context, Context::default());
    }
}
