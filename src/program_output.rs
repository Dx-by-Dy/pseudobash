use std::process::Output;

#[derive(Debug)]
pub struct ProgramOutput {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl ProgramOutput {
    pub fn new(code: i32, stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        Self {
            code: code,
            stdout,
            stderr,
        }
    }
}

impl From<Output> for ProgramOutput {
    fn from(value: Output) -> Self {
        Self {
            code: value.status.code().unwrap(),
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}
