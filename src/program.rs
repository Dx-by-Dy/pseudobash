use {crate::config::CONFIG, std::ffi::CString};

#[derive(Debug, Default)]
pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub fn add_args(mut self, args: &Vec<u8>) -> anyhow::Result<Self> {
        Self::normalize(args, &mut self.data, &mut false)?;
        Ok(self)
    }

    pub fn is_default(&self) -> bool {
        self.data.get(0).is_some_and(|byte| *byte == b'@')
    }

    pub fn get_default_name_and_args(&self) -> anyhow::Result<Vec<String>> {
        if !self.is_default() {
            anyhow::bail!("Not defaults program")
        }

        Ok(String::from_utf8_lossy(&self.data[1..self.data.len() - 1])
            .split("\0")
            .map(|word| word.to_string())
            .collect())
    }

    fn normalize(
        input: &[u8],
        buffer: &mut Vec<u8>,
        with_command: &mut bool,
    ) -> anyhow::Result<()> {
        let input_len = input.len();
        let mut idx = 0;
        let mut last_byte = b' ';

        while idx < input_len {
            let byte = input[idx];
            match byte {
                b' ' | b'\n' | b'\0' => {
                    if last_byte != b' ' && last_byte != b'\n' && last_byte != b'\0' {
                        buffer.push(b'\0');
                        if *with_command {
                            CONFIG.get_full_path(buffer)?;
                            *with_command = false;
                        }
                    }
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                    buffer.push(byte);
                }
                _ => {
                    anyhow::bail!(format!("Unknown symbol: {}", byte as char))
                }
            }
            last_byte = byte;
            idx += 1
        }
        if buffer.last().is_some_and(|byte| *byte != b'\0') {
            buffer.push(b'\0');
        }

        Ok(())
    }
}

impl TryFrom<Vec<u8>> for Program {
    type Error = anyhow::Error;

    fn try_from(mut bytes: Vec<u8>) -> anyhow::Result<Self> {
        bytes.push(b'\0');

        let mut data = Vec::with_capacity(bytes.len());
        let mut with_command = true;
        Self::normalize(&bytes, &mut data, &mut with_command)?;

        if with_command {
            anyhow::bail!(format!(
                "Unknown command: {:?}",
                String::from_utf8_lossy(&data),
            ))
        }

        Ok(Self { data })
    }
}

impl TryFrom<CString> for Program {
    type Error = anyhow::Error;

    fn try_from(value: CString) -> anyhow::Result<Self> {
        Self::try_from(value.as_bytes().to_vec())
    }
}

impl<'a> IntoIterator for &'a Program {
    type Item = *const i8;

    type IntoIter = IterProgram<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IterProgram::new(&self.data)
    }
}

pub struct IterProgram<'a> {
    data: &'a Vec<u8>,
    last_byte: u8,
    index: usize,
}

impl<'a> IterProgram<'a> {
    fn new(data: &'a Vec<u8>) -> Self {
        Self {
            data,
            last_byte: 0,
            index: 0,
        }
    }
}

impl<'a> Iterator for IterProgram<'a> {
    type Item = *const i8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.last_byte != 0 {
            self.last_byte = match self.data.get(self.index) {
                Some(byte) => *byte,
                None => return None,
            };
            self.index += 1
        }
        match self.data.get(self.index) {
            Some(byte) => {
                let value = &self.data[self.index] as *const u8 as *const i8;
                self.last_byte = *byte;
                self.index += 1;
                Some(value)
            }
            None => return None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{ffi::CString, str::FromStr};

    use crate::program::Program;

    #[test]
    fn check_program_iterator() {
        let p = Program {
            data: vec![2, 3, 0, 9, 0],
        };
        let mut it = p.into_iter();

        assert_eq!(unsafe { *it.next().unwrap() } as u8, it.data[0]);
        assert_eq!(unsafe { *it.next().unwrap() } as u8, it.data[3]);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);

        let p = Program {
            data: vec![2, 0, 0, 9, 0],
        };
        let mut it = p.into_iter();

        assert_eq!(unsafe { *it.next().unwrap() } as u8, it.data[0]);
        assert_eq!(unsafe { *it.next().unwrap() } as u8, it.data[2]);
        assert_eq!(unsafe { *it.next().unwrap() } as u8, it.data[3]);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn check_program_tryfrom() {
        let p = Program::try_from(CString::new("  echo 1278 ").unwrap()).unwrap();
        assert_eq!(
            p.data,
            String::from_str("/home/none/pseudobash/utils/echo/target/release/echo\01278\0")
                .unwrap()
                .as_bytes()
                .to_vec()
        );

        let p = Program::try_from(CString::new("  echo 1278 echo").unwrap()).unwrap();
        assert_eq!(
            p.data,
            String::from_str("/home/none/pseudobash/utils/echo/target/release/echo\01278\0echo\0")
                .unwrap()
                .as_bytes()
                .to_vec()
        );

        let p = Program::try_from(CString::new("echo").unwrap()).unwrap();
        assert_eq!(
            p.data,
            String::from_str("/home/none/pseudobash/utils/echo/target/release/echo\0")
                .unwrap()
                .as_bytes()
                .to_vec()
        );

        let p = Program::try_from(CString::new("echo @").unwrap());
        assert!(p.is_err());

        let p = Program::try_from(CString::new("").unwrap());
        assert!(p.is_err());

        let p = Program::try_from(CString::new("e").unwrap());
        assert!(p.is_err());
    }
}
