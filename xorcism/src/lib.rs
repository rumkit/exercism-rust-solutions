use std::borrow::Borrow;
use std::io::{Read, Write};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key_pointer: usize,
    key: &'a [u8],
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: ?Sized + AsRef<[u8]>>(key: &'a Key) -> Xorcism<'a> {
        let key = key.as_ref();
        assert!(!key.is_empty(), "Key cannot be empty!");
        Self { key, key_pointer: 0 }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for item in data.iter_mut() {
            *item ^= self.next_key_byte()
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<Data,Item>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator<Item = Item>,
        Item: Borrow<u8>,
    {
        data.into_iter().map(move |d| { d.borrow() ^self.next_key_byte() })
    }

    fn next_key_byte(&mut self) -> u8 {
        let byte = self.key[self.key_pointer];
        self.key_pointer = (self.key_pointer + 1) % self.key.len();
        byte
    }

    // Stream adaptors bonus
    pub fn reader(self, inner: impl Read) -> impl Read {
        XorcismReader { xorcism: self, inner }
    }

    pub fn writer(self, inner: impl Write) -> impl Write {
        XorcismWriter { xorcism: self, inner }
    }
}


struct XorcismReader<'a, R: Read> {
    xorcism: Xorcism<'a>,
    inner: R
}
struct XorcismWriter<'a, W: Write> {
    xorcism: Xorcism<'a>,
    inner: W
}

impl<R: Read> Read for XorcismReader<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let cnt = self.inner.read(buf)?;
        self.xorcism.munge_in_place(&mut buf[..cnt]);
        Ok(cnt)
    }
}

const WRITER_BUF_SIZE: usize = 128;
impl<W: Write> Write for XorcismWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for chunk in buf.chunks(WRITER_BUF_SIZE) {
            // Allocated temporary buffer for the next chunk
            let mut inner_buf = [0u8; WRITER_BUF_SIZE];
            // Copy chunk to a sized slice of the temporary buffer
            let target_slice = &mut inner_buf[..chunk.len()];
            target_slice.copy_from_slice(chunk);
            // And munge it in place
            self.xorcism.munge_in_place(target_slice);
            // Finally, pass it down the pipeline
            self.inner.write_all(target_slice)?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
