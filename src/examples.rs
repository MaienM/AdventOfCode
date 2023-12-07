use std::ops::Deref;

pub struct Example {
    pub input: &'static str,
    pub part1: Option<&'static str>,
    pub part2: Option<&'static str>,
}
impl Deref for Example {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
