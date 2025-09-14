use {
    crate::{delimeter::Delimeter, program::Program},
    std::{ffi::CString, vec::IntoIter},
};

pub struct Pipeline {
    programs: Vec<Program>,
    delimeters: Vec<Delimeter>,
}

impl TryFrom<&mut Vec<u8>> for Pipeline {
    type Error = anyhow::Error;

    fn try_from(bytes: &mut Vec<u8>) -> anyhow::Result<Self> {
        bytes.push(b'\n');

        let command_len = bytes.len();
        let mut idx = 0;
        let mut programs = Vec::new();
        let mut delimeters = vec![Delimeter::Seq];
        let mut buffer = Vec::new();

        while idx < command_len {
            let byte = bytes[idx];
            match byte {
                b'|' => {
                    let mut line = Vec::with_capacity(buffer.capacity());
                    std::mem::swap(&mut buffer, &mut line);
                    programs.push(Program::try_from(line)?);
                    delimeters.push(Delimeter::Pipe);
                }
                b';' => {
                    let mut line = Vec::with_capacity(buffer.capacity());
                    std::mem::swap(&mut buffer, &mut line);
                    programs.push(Program::try_from(line)?);
                    delimeters.push(Delimeter::Seq);
                }
                _ => {
                    buffer.push(byte);
                }
            }
            idx += 1
        }
        if buffer.len() > 0 {
            programs.push(Program::try_from(buffer)?);
        }

        Ok(Self {
            programs,
            delimeters,
        })
    }
}

impl TryFrom<CString> for Pipeline {
    type Error = anyhow::Error;

    fn try_from(value: CString) -> anyhow::Result<Self> {
        Self::try_from(&mut value.as_bytes().to_vec())
    }
}

impl Pipeline {
    pub fn into_iter(self) -> IterPipeline {
        IterPipeline {
            it_prog: self.programs.into_iter(),
            it_delim: self.delimeters.into_iter(),
        }
    }
}

pub struct IterPipeline {
    it_prog: IntoIter<Program>,
    it_delim: IntoIter<Delimeter>,
}

impl IterPipeline {
    pub fn next(&mut self, output: &Vec<u8>) -> Option<anyhow::Result<Program>> {
        let Some(program) = self.it_prog.next() else {
            return None;
        };
        match self.it_delim.next().unwrap() {
            Delimeter::Pipe => Some(program.add_args(output)),
            Delimeter::Seq => Some(Ok(program)),
        }
    }
}
