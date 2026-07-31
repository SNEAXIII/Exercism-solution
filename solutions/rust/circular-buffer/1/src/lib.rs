#[derive(Debug)]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    write_cursor: usize,
    read_cursor: usize,
    capacity: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: (0..capacity).map(|_| None).collect(),
            write_cursor: 0,
            read_cursor: 0,
            capacity,
        }
    }
    pub fn move_write_cursor(&mut self) {
        self.write_cursor = (self.write_cursor + 1) % self.capacity;
    }
    pub fn move_read_cursor(&mut self) {
        self.read_cursor = (self.read_cursor + 1) % self.capacity;
    }
    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer[self.write_cursor].is_none() {
            self.buffer[self.write_cursor] = Some(element);
            self.move_write_cursor();
            Ok(())
        } else {
            Err(Error::FullBuffer)
        }
    }
    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer[self.read_cursor].is_none() {
            Err(Error::EmptyBuffer)
        } else {
            let return_value = self.buffer[self.read_cursor]
                .take()
                .expect("Should not be none");
            self.move_read_cursor();
            Ok(return_value)
        }
    }
    pub fn clear(&mut self) {
        for item in &mut self.buffer {
            *item = None
        }
        self.write_cursor = 0;
        self.read_cursor = 0;
    }
    pub fn overwrite(&mut self, element: T) {
        if self.buffer[self.write_cursor].is_some() {
            self.move_read_cursor();
        }
        self.buffer[self.write_cursor] = Some(element);
        self.move_write_cursor();
    }
}
