#[derive(Debug, Default)]
pub struct Program {
    data: Vec<u8>,
}

impl Program {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn add_args(&mut self, mut args: Vec<u8>) {
        self.data.append(&mut args);
    }

    pub fn is_default(&self) -> bool {
        self.data.get(0).is_some_and(|byte| *byte == b'@')
    }

    pub fn get_data(self) -> Vec<u8> {
        self.data
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
}
