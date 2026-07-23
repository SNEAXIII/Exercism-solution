use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl fmt::Display for Corner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    corners: [Corner; 2],
}
impl From<[Corner; 2]> for Rectangle {
    fn from(corners: [Corner; 2]) -> Self {
        Self::new(corners)
    }
}
impl Rectangle {
    fn new(corners: [Corner; 2]) -> Self {
        Self { corners }
    }
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#?})", self.corners)
    }
}
struct AsciiTable<'a> {
    corners: Vec<Corner>,
    ascii_table: &'a [&'a str],
    iter: u32,
}
impl<'a> From<&'a [&'a str]> for AsciiTable<'a> {
    fn from(slice: &'a [&'a str]) -> Self {
        Self::new(slice)
    }
}
impl<'a> AsciiTable<'a> {
    fn new(ascii_table: &'a [&'a str]) -> Self {
        Self {
            iter: 0,
            corners: Vec::new(),
            ascii_table,
        }
    }

    fn is_valid_rectangle(&self, rect: &Rectangle) -> bool {
        if self.ascii_table[rect.corners[0].y].as_bytes()[rect.corners[1].x] as char != '+'
            || self.ascii_table[rect.corners[1].y].as_bytes()[rect.corners[0].x] as char != '+'
        {
            return false;
        }
        for y in [rect.corners[1].y, rect.corners[0].y] {
            for x in (rect.corners[1].x + 1)..rect.corners[0].x {
                if !"-+".contains(self.ascii_table[y].as_bytes()[x] as char) {
                    return false;
                }
            }
        }

        for x in [rect.corners[1].x, rect.corners[0].x] {
            for y in (rect.corners[1].y + 1)..rect.corners[0].y {
                if !"|+".contains(self.ascii_table[y].as_bytes()[x] as char) {
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
                self.iter += 1;
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
