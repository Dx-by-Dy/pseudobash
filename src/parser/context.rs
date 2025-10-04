use crate::parser::arg_builder::ArgBuilderState;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Context {
    pub arg_builder_state: ArgBuilderState,
    pub token_in_process: bool,
    pub current_arg_is_setter: bool,
}
