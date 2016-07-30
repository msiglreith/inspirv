
use std::fmt;
use std::io::{Result, Write};

/// Wraps a writer and adds additional identation on every write call in front of the data
pub struct IdentWriter<W: Write> {
	inner: W,
	cur_ident: usize,
}

impl<W: Write> IdentWriter<W> {
	pub fn new(inner: W) -> IdentWriter<W> {
		IdentWriter {
			inner: inner,
			cur_ident: 0,
		}
	}

	pub fn ident(&mut self) {
		self.cur_ident = self.cur_ident.saturating_add(4);
	}

	pub fn unident(&mut self) {
		self.cur_ident = self.cur_ident.saturating_sub(4);
	}

	fn write_idents(&mut self) -> Result<()> {
		// ident using spaces
		for _ in 0..self.cur_ident {
			try!(self.inner.write(b" "));
		}

		Ok(())
	}
}

impl<W: Write> Write for IdentWriter<W> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		try!(self.write_idents());
		self.inner.write(buf)
	}

	fn flush(&mut self) -> Result<()> {
		self.inner.flush()
	}

	fn write_all(&mut self, buf: &[u8]) -> Result<()> {
		try!(self.write_idents());
        self.inner.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()> {
    	try!(self.write_idents());
		self.inner.write_fmt(fmt)
    }
}
