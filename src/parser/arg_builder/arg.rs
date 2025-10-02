use crate::{global_state::GlobalState, parser::token::Token};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Arg(Vec<Token>);

impl Arg {
    pub fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_string_with_executing(self, gs: &mut GlobalState) -> String {
        String::from_utf8_lossy(
            &self
                .0
                .into_iter()
                .map(|mut token| {
                    token.to_default(gs);
                    token.downgrade()
                })
                .flatten()
                .collect::<Vec<u8>>(),
        )
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{arg_builder::arg::Arg, token::Token};

    impl Arg {
        pub fn new(tokens: Vec<Token>) -> Self {
            Self(tokens)
        }
    }
}
