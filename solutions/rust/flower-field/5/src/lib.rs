struct Garden<'a> {
    row_count: usize,
    column_count: usize,
    counter: Vec<i8>,
    garden: &'a [&'a str],
}
impl<'a> From<&'a [&'a str]> for Garden<'a> {
    fn from(slice: &'a [&'a str]) -> Self {
        Self::new(slice)
    }
}
impl<'a> Garden<'a> {
    fn new(garden: &'a [&'a str]) -> Self {
        let row_count = garden.len();
        let column_count = match garden.first() {
            Some(row) => row.len(),
            None => 0,
        };
        let vec_size = column_count * row_count;
        let counter: Vec<i8> = vec![0; vec_size];
        Self {
            row_count,
            column_count,
            counter,
            garden,
        }
    }
    #[must_use]
    const fn get_flat_index(&self, x: usize, y: usize) -> usize {
        x + y * self.column_count
    }
    fn increment_proximity_grass(&mut self, x: usize, y: usize) {
        let mut flattened_index = self.get_flat_index(x, y);
        self.counter[flattened_index] = -1;
        for ry in y.saturating_sub(1)..self.row_count.min(y + 2) {
            for rx in x.saturating_sub(1)..self.column_count.min(x + 2) {
                flattened_index = self.get_flat_index(rx, ry);
                if self.counter[flattened_index] != -1 {
                    self.counter[flattened_index] += 1;
                }
            }
        }
    }
    fn solve(&mut self) -> Vec<String> {
        for (iy, row) in self.garden.iter().enumerate() {
            for (ix, char) in row.chars().enumerate() {
                if char == '*' {
                    self.increment_proximity_grass(ix, iy);
                }
            }
        }
        let mut result = vec![String::with_capacity(self.column_count); self.row_count];
        for (iy, row) in self.garden.iter().enumerate() {
            for ix in 0..row.len() {
                let count = self.counter[self.get_flat_index(ix, iy)];
                let symbol = match count {
                    0 => ' ',
                    -1 => '*',
                    _ => (count as u8 + b'0') as char,
                };
                result[iy].push(symbol)
            }
        }
        result
    }
}
pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        Vec::new()
    } else {
        Garden::from(garden).solve()
    }
}
