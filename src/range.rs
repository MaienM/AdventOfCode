pub struct ReversibleRange {
    current: usize,
    end: usize,
    step: isize,
}
#[allow(dead_code)]
impl ReversibleRange {
    fn contains(&self, value: usize) -> bool {
        if self.step > 0 {
            self.current <= value && value <= self.end
        } else {
            self.end <= value && value <= self.current
        }
    }
}
impl Iterator for ReversibleRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        self.current = self.current.checked_add_signed(self.step).unwrap();
        Some(self.current)
    }
}

#[must_use]
pub fn range(start: usize, end: usize) -> ReversibleRange {
    let step: isize = if start < end { 1 } else { -1 };
    ReversibleRange {
        end,
        current: start.checked_add_signed(-step).unwrap(),
        step,
    }
}
