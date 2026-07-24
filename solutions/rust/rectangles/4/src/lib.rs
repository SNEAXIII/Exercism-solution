#[derive(Clone, Copy)]
struct Corner {
    x: usize,
    y: usize,
}
impl From<[usize; 2]> for Corner {
    fn from(coords: [usize; 2]) -> Self {
        Self {
            x: coords[0],
            y: coords[1],
        }
    }
}
struct Rectangle {
    right_bottom: Corner,
    left_top: Corner,
}
impl From<[Corner; 2]> for Rectangle {
    fn from(corners: [Corner; 2]) -> Self {
        Self {
            right_bottom: corners[0],
            left_top: corners[1],
        }
    }
}
struct AsciiTable<'a> {
    corners: Vec<Corner>,
    ascii_table: &'a [&'a str],
}
impl<'a> From<&'a [&'a str]> for AsciiTable<'a> {
    fn from(ascii_table: &'a [&'a str]) -> Self {
        Self {
            corners: Vec::new(),
            ascii_table,
        }
    }
}
impl<'a> AsciiTable<'a> {
    fn char_at(&self, x: usize, y: usize) -> u8 {
        self.ascii_table[y].as_bytes()[x]
    }
    fn is_valid_rectangle(&self, rect: &Rectangle) -> bool {
        let Rectangle {
            left_top: Corner { x: left, y: top },
            right_bottom:
                Corner {
                    x: right,
                    y: bottom,
                },
        } = *rect;
        if self.char_at(left, bottom) != b'+' || self.char_at(right, top) != b'+' {
            return false;
        }

        for x in (left + 1)..right {
            if !matches!(self.char_at(x, top), b'-' | b'+')
                || !matches!(self.char_at(x, bottom), b'-' | b'+')
            {
                return false;
            }
        }

        for y in (top + 1)..bottom {
            if !matches!(self.char_at(left, y), b'|' | b'+')
                || !matches!(self.char_at(right, y), b'|' | b'+')
            {
                return false;
            }
        }
        true
    }
    fn solve(&mut self) -> usize {
        let mut result = 0;
        for (iy, row) in self.ascii_table.iter().enumerate() {
            for (ix, char) in row.chars().enumerate() {
                if char != '+' {
                    continue;
                }
                let corner = Corner::from([ix, iy]);
                self.corners.push(corner);
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
