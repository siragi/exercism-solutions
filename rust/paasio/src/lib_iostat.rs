use std::io::{Read, Result, Write};

pub type ReadStats<T> = IoStats<T>;
pub type WriteStats<T> = IoStats<T>;

pub struct IoStats<T> {
    data: T,
    bytes_through: usize,
    num_reads: usize,
    num_writes: usize,
}

impl<T> IoStats<T> {
    pub fn new(data: T) -> IoStats<T> {
        Self {
            data,
            bytes_through: 0,
            num_reads: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize
    where
        T: Read,
    {
        self.num_reads
    }

    pub fn writes(&self) -> usize
    where
        T: Write,
    {
        self.num_writes
    }
}

impl<T: Read> Read for IoStats<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Collect statistics about this call reading
        let num_bytes = self.data.read(buf)?;
        self.bytes_through += num_bytes;
        self.num_reads += 1;
        Ok(num_bytes)
    }
}

impl<T: Write> Write for IoStats<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Collect statistics about this call writing
        let num_bytes = self.data.write(buf)?;
        self.bytes_through += num_bytes;
        self.num_writes += 1;
        Ok(num_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
