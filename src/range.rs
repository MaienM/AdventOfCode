pub struct ReversibleRange {
    current: usize,
    end: usize,
    step: isize,
}
impl ReversibleRange {
    fn contains(&self, value: usize) -> bool {
        return if self.step > 0 {
            self.current <= value && value <= self.end
        } else {
            self.end <= value && value <= self.current
        };
    }
}
impl Iterator for ReversibleRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            return None;
        }

        self.current = (self.current as isize + self.step) as usize;
        return Some(self.current);
    }
}

pub fn range(start: usize, end: usize) -> ReversibleRange {
    let step: isize = if start < end { 1 } else { -1 };
    return ReversibleRange {
        end,
        current: (start as isize - step) as usize,
        step,
    };
}
