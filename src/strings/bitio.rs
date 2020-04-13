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
}
impl BitReader for MemBitIO {
	fn read(&mut self, len: usize) -> Option<usize> {
		println!("read:r={},len={},w={}", self.r, len, self.w);
		if len > MAX_BITS || self.r + len > self.w {
			return None;
		}

		let n = (self.r + len) / 8 - (self.r / 8);
		let hi = (self.r + len) % 8;
		// within one byte
		println!("read:n={}", n);
		if n == 0 {
			let mut b = (self.buf[self.r / 8] as usize) & ((1 << hi) - 1);
			println!("read:{},b={}", self.r, b);
			b >>= self.r % 8;
			self.r += len;
			return Some(b);
		}

		let mut result: usize = 0;
		for i in 0..n {
			result = result * 0x100 + (self.buf[self.r / 8 + i] as usize);
		}
		result >>= self.r % 8;
		self.r += n * 8;

		let last = (self.buf[self.r / 8] as usize) & ((1 << hi) - 1);
		result += last << (len - hi);
		self.r += hi;

		Some(result)
	}
}
impl BitWriter for MemBitIO {
	fn write(&mut self, val: usize, len: usize) {
		println!("write:len={},size={}", len, MAX_BITS);
		if len == 0 || len > MAX_BITS {
			return;
		}
		if self.w % 8 == 0 {
			self.buf.push(0);
		}

		let n = (self.w + len) / 8 - (self.w / 8);
		println!("write:w={},n={}", self.w, n);
		if n == 0 {
			let val = val << (self.w % 8);
			println!("write:{}<={}", self.w, val);
			self.buf[self.w / 8] += val as u8;
			self.w += len;
			return;
		}

		let head = 8 - self.w % 8;
		let first = val & ((1 << head) - 1);
		println!("write:head={},first={}", head, first);
		self.buf[self.w / 8] += first as u8;
		self.w += head;
		let mut val = val >> head;
		let mut remain = len - head;
		println!("write:remain={}", remain);
		while remain > 0 {
			self.buf.push((val & 0xff) as u8);
			if remain >= 8 {
				val >>= 8;
				remain -= 8;
			} else {
				break;
			};
		}
		self.w += len - head;
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
}
