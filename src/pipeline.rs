use crate::program::Program;

pub struct Pipeline {
    programs: Vec<Program>,
}

impl Pipeline {
    pub fn new(programs: Vec<Program>) -> Self {
        Self { programs }
    }
}

impl IntoIterator for Pipeline {
    type Item = Program;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.programs.into_iter()
    }
}
