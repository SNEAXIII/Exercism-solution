struct Garden<'a> {
    row_count: usize,
    column_count: usize,
    counter: Vec<i8>,
    garden: &'a [&'a str],
}
impl<'a> Garden<'a> {
    fn new(garden: &'a [&'a str]) -> Self {
        let row_count: usize = garden.len();
        let column_count: usize = garden[0].len();
        let vec_size = column_count * row_count;
        let mut counter: Vec<i8> = Vec::with_capacity(vec_size);
        counter.resize(vec_size, 0);
        Self {
            row_count,
            column_count,
            counter,
            garden,
        }
    }

    fn get_flat_index(&self, x: usize, y: usize) -> usize {
        x + y * self.column_count
    }

    fn increment_proximity_grass(&mut self, x: usize, y: usize) {
        let mut flattened_index = self.get_flat_index(x, y);
        self.counter[flattened_index] = -1;
        for ry in y.saturating_sub(1)..=(self.row_count - 1).min(y + 1) {
            for rx in x.saturating_sub(1)..=(self.column_count - 1).min(x + 1) {
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
        let mut result: Vec<String> = (0..self.row_count)
            .map(|_| String::with_capacity(self.column_count))
            .collect();
        for (iy, row) in self.garden.iter().enumerate() {
            for ix in 0..row.len() {
                let flattened_index = self.get_flat_index(ix, iy);
                match self.counter[flattened_index] {
                    0 => result[iy].push(' '),
                    -1 => result[iy].push('*'),
                    _ => result[iy].push((self.counter[flattened_index] as u8 + b'0') as char),
                }
            }
        }
        result
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::<String>::new();
    }
    let mut gard: Garden<'_> = Garden::new(garden);
    gard.solve()
}