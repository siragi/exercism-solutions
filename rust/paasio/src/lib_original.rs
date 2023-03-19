use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes_through: usize,
    num_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(data: R) -> ReadStats<R> {
        Self {
            data,
            bytes_through: 0,
            num_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        // Collect statistics about this call reading
        let num_bytes = self.data.read(buf)?;
        self.bytes_through += num_bytes;
        self.num_reads += 1;
        Ok(num_bytes)
    }
}

pub struct WriteStats<W> {
    data: W,
    bytes_through: usize,
    num_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(data: W) -> WriteStats<W> {
        Self {
            data,
            bytes_through: 0,
            num_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
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
