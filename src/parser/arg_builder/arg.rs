use crate::{global_state::GlobalState, parser::token::Token};

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum ArgType {
    #[default]
    Default,

    VarSetter,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Arg {
    data: Vec<Token>,
    kind: ArgType,
}

impl Arg {
    pub fn push(&mut self, token: Token) {
        self.data.push(token);
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set_setter(&mut self) {
        self.kind = ArgType::VarSetter
    }

    pub fn into_string_with_executing(self, gs: &mut GlobalState) -> String {
        let arg = &self
            .data
            .into_iter()
            .map(|mut token| {
                token.to_default(gs);
                token.downgrade()
            })
            .flatten()
            .collect::<Vec<u8>>();
        if self.kind == ArgType::VarSetter {
            gs.environment.set_var(arg.to_vec());
            String::new()
        } else {
            String::from_utf8_lossy(arg).to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        arg_builder::arg::{Arg, ArgType},
        token::Token,
    };

    impl Arg {
        pub fn new_default(tokens: Vec<Token>) -> Self {
            Self {
                data: tokens,
                kind: ArgType::Default,
            }
        }

        pub fn new_var_setter(tokens: Vec<Token>) -> Self {
            Self {
                data: tokens,
                kind: ArgType::VarSetter,
            }
        }
    }
}
