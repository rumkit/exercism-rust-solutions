use std::mem::take;

pub struct CircularBuffer<T> {
    data: Vec<T>,
    read_pointer: usize,
    write_pointer: usize,
    buffer_full: bool,
}

enum Pointer {
    Read,
    Write
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default + Clone> CircularBuffer<T> {

    pub fn new(capacity: usize) -> Self {
        Self { data: vec![T::default(); capacity], read_pointer: 0, write_pointer: 0, buffer_full: false }
    }

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    fn inc_pointer(&mut self, pointer: Pointer) {
        let capacity = self.capacity();
        let pointer = match pointer {
            Pointer::Read => &mut self.read_pointer,
            Pointer::Write => &mut self.write_pointer,
        };
        * pointer = (*pointer + 1) % capacity;
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer_full {
            return Err(Error::FullBuffer);
        }

        self.data[self.write_pointer] = element;
        self.inc_pointer(Pointer::Write);
        self.buffer_full = self.write_pointer == self.read_pointer;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.read_pointer == self.write_pointer && !self.buffer_full {
            return Err(Error::EmptyBuffer);
        }

        let read = take(&mut self.data[self.read_pointer]);
        self.inc_pointer(Pointer::Read);
        self.buffer_full = false;
        Ok(read)
    }

    pub fn clear(&mut self) {
        self.read_pointer = 0;
        self.write_pointer = 0;
        self.buffer_full = false;
        self.data = vec![T::default(); self.capacity()];
    }

    pub fn overwrite(&mut self, element: T) {
        self.data[self.write_pointer] = element;
        if self.write_pointer == self.read_pointer {
            self.inc_pointer(Pointer::Read);
        }
        self.inc_pointer(Pointer::Write);
        self.buffer_full = self.write_pointer == self.read_pointer;
    }
}
