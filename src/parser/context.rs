use crate::parser::arg_builder::ArgBuilderState;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Context {
    pub arg_state: ArgBuilderState,
    pub token_in_process: bool,
}
