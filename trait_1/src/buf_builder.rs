use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Write;

pub(crate) struct BufBuilder {
    pub(crate) buf: Vec<u8>,
}
impl BufBuilder {
    pub fn new() -> Self {
        BufBuilder { buf: Vec::new() }
    }
}
impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(&self.buf))
    }
}
