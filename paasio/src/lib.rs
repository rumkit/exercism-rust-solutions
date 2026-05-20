use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    reader: R,
    bytes_read: usize,
    reads_count: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            reader: wrapped,
            bytes_read: 0,
            reads_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.reader.read(buf);
        result.inspect(|n| {
            self.reads_count += 1;
            self.bytes_read += n;
        })
    }
}

pub struct WriteStats<W> {
    writer: W,
    bytes_written: usize,
    writes_count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            writer: wrapped,
            bytes_written: 0,
            writes_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.writes_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.writer.write(buf);
        result.inspect(|n| {
            self.writes_count += 1;
            self.bytes_written += n;
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
