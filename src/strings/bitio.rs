const MAX_BITS: usize = std::mem::size_of::<usize>() * 8;

pub trait BitReader {
	fn read(&mut self, len: usize) -> Option<usize>;
}
pub trait BitWriter {
	fn write(&mut self, val: usize, len: usize);
}

// LSB
pub struct MemBitIO {
	buf: Vec<u8>,
	r: usize,
	w: usize,
}
impl MemBitIO {
	pub fn new() -> Self {
		Self {
			buf: Vec::new(),
			r: 0,
			w: 0,
		}
	}

	fn read_on_safe(&mut self, len: usize) -> usize {
		// assert!(len <= MAX_BITS - 8);
		let mut radix = 1;
		let mut result: usize = 0;
		for i in (self.r / 8)..((self.r + len + 7) / 8) {
			result += (self.buf[i] as usize) * radix;
			radix *= 0x100;
		}
		result >>= self.r % 8;

		self.r += len;
		if self.r == self.w {
			self.r = 0;
			self.w = 0;
			self.buf.clear();
		}

		result & ((1 << len) - 1)
	}
}
impl BitReader for MemBitIO {
	fn read(&mut self, len: usize) -> Option<usize> {
		if len == 0 || len > MAX_BITS || self.r + len > self.w {
			return None;
		}

		let n = if len <= MAX_BITS / 2 {
			len
		} else {
			len - MAX_BITS / 2
		};
		let mut result = self.read_on_safe(n);
		if n < len {
			result += self.read_on_safe(len - n) * (1 << (MAX_BITS / 2));
		}

		Some(result)
	}
}
impl BitWriter for MemBitIO {
	fn write(&mut self, val: usize, len: usize) {
		if len == 0 || len > MAX_BITS {
			return;
		}
		if self.w % 8 == 0 {
			self.buf.push(0);
		}

		let val = if len < MAX_BITS {
			val & ((1 << len) - 1) // clear unused MSB
		} else {
			val
		};

		let first = (val << (self.w % 8)) & 0xff;
		self.buf[self.w / 8] |= first as u8;

		let n_first = 8 - self.w % 8;
		let nw = if len < n_first { len } else { n_first };
		let mut val = val >> nw;
		let mut remain = len - nw;
		while remain > 0 {
			self.buf.push((val & 0xff) as u8);
			if remain >= 8 {
				val >>= 8;
				remain -= 8;
			} else {
				break;
			};
		}

		self.w += len;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn within_one_byte() {
		let mut io = MemBitIO::new();
		assert_eq!(None, io.read(1));
		io.write(1, 1);
		assert_eq!(Some(1), io.read(1));
		assert_eq!(None, io.read(1));
	}

	#[test]
	fn more_than_one_byte() {
		let mut io = MemBitIO::new();
		io.write(0x7ff, 11);
		assert_eq!(None, io.read(12));
		assert_eq!(Some(0x3ff), io.read(10));
		assert_eq!(Some(1), io.read(1));
		assert_eq!(None, io.read(1));
	}

	#[test]
	fn value_exceeds_len() {
		let mut io = MemBitIO::new();
		io.write(0x7, 1);
		assert_eq!(None, io.read(3));
		assert_eq!(Some(1), io.read(1));
		assert_eq!(None, io.read(1));

		io.write(0x7ff, 8);
		assert_eq!(Some(0xff), io.read(8));
		assert_eq!(None, io.read(1));
	}
}
