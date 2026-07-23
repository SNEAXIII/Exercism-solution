#[derive(Clone, Copy)]
struct Corner {
    x: usize,
    y: usize,
}
impl From<[usize; 2]> for Corner {
    fn from(coords: [usize; 2]) -> Self {
        Self::new(coords[0], coords[1])
    }
}
impl Corner {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct Rectangle {
    right_down: Corner,
    left_top: Corner,
}
impl From<[Corner; 2]> for Rectangle {
    fn from(corners: [Corner; 2]) -> Self {
        Self::new(corners)
    }
}
impl Rectangle {
    fn new(corners: [Corner; 2]) -> Self {
        Self {
            right_down: corners[0],
            left_top: corners[1],
        }
    }
}
struct AsciiTable<'a> {
    corners: Vec<Corner>,
    ascii_table: &'a [&'a str],
}
impl<'a> From<&'a [&'a str]> for AsciiTable<'a> {
    fn from(slice: &'a [&'a str]) -> Self {
        Self::new(slice)
    }
}
impl<'a> AsciiTable<'a> {
    fn new(ascii_table: &'a [&'a str]) -> Self {
        Self {
            corners: Vec::new(),
            ascii_table,
        }
    }
    fn char_at(&self, x: usize, y: usize) -> u8 {
        self.ascii_table[y].as_bytes()[x]
    }
    fn is_valid_rectangle(&self, rect: &Rectangle) -> bool {
        if self.char_at(rect.left_top.x, rect.right_down.y) != b'+'
            || self.char_at(rect.right_down.x, rect.left_top.y) != b'+'
        {
            return false;
        }
        for y in [rect.left_top.y, rect.right_down.y] {
            for x in (rect.left_top.x + 1)..rect.right_down.x {
                if !matches!(self.char_at(x, y), b'-' | b'+') {
                    return false;
                }
            }
        }

        for x in [rect.left_top.x, rect.right_down.x] {
            for y in (rect.left_top.y + 1)..rect.right_down.y {
                if !matches!(self.char_at(x, y), b'|' | b'+') {
                    return false;
                }
            }
        }

        true
    }
    fn solve(&mut self) -> usize {
        let mut result = 0;
        for (iy, row) in self.ascii_table.iter().enumerate() {
            for (ix, char) in row.chars().enumerate() {
                if char == '+' {
                    self.corners.push(Corner::from([ix, iy]));
                }
            }
        }
        for _ in 0..self.corners.len().saturating_sub(3) {
            let corner = self.corners.pop().expect("Should have a value");
            for corner_dest in &self.corners {
                if corner.x <= corner_dest.x || corner.y <= corner_dest.y {
                    continue;
                }

                let rect = Rectangle::from([corner, *corner_dest]);
                if self.is_valid_rectangle(&rect) {
                    result += 1
                };
            }
        }
        result
    }
}
pub fn count(ascii_table: &[&str]) -> usize {
    if ascii_table.is_empty() {
        0
    } else {
        AsciiTable::from(ascii_table).solve()
    }
}
