use super::bitio::*;
use super::tst_u8::*;

const R: usize = 256;
const CODE_BITS: usize = 12;
const MAX_CODES: usize = 1 << CODE_BITS;

pub fn compress(input: &[u8]) -> Vec<u8> {
	let mut tst = new_symbol_tst();
	let mut io = MemBitIO::new();
	let mut code = R;

	let mut p = input;
	while !p.is_empty() {
		let n = tst.longest_key_of(p);
		let symbol = tst.get(&p[..n]).unwrap();
		io.write(*symbol, CODE_BITS);

		if n + 1 <= p.len() && code < MAX_CODES {
			tst.put(&p[..n + 1], code);
			code += 1;
		}
		p = &p[n..];
	}

	io.dump()
}
pub fn decompress(input: &[u8]) -> Result<Vec<u8>, String> {
	let mut st = (0..R).map(|i| vec![i as u8; 1]).collect::<Vec<_>>();

	let mut input = MemBitIO::from_buffer(input);
	let v = input.read(12).ok_or("invalid input")?;
	let mut last = vec![v as u8; 1];
	let mut out = last.clone();

	while let Some(symbol) = input.read(12) {
		let word = if let Some(word) = st.get(symbol) {
			word.clone()
		} else if symbol == st.len() {
			let mut word = last.clone();
			word.push(last[0]);
			word
		} else {
			return Err("wrong state".into());
		};
		out.extend_from_slice(&word);

		if st.len() < MAX_CODES {
			last.push(word[0]);
			st.push(last);
		}

		last = word;
	}

	Ok(out)
}

fn new_symbol_tst() -> TST<usize> {
	let mut tst = TST::new();

	let mut w = vec![0u8; 1];
	for i in 0..R {
		w[0] = i as u8;
		tst.put(&w, i);
	}

	tst
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let r = compress("".as_bytes());
		assert_eq!(0, r.len());
		assert!(decompress(&r).is_err());
	}

	#[test]
	fn basic() {
		let v = "a".as_bytes().iter().cloned().collect::<Vec<_>>();
		let r = compress(&v[..]);
		assert_eq!(vec![97, 0], r);
		assert_eq!(v, decompress(&r).ok().unwrap());

		let s = "aaaaaaaaaa";
		let v = s.as_bytes().iter().cloned().collect::<Vec<_>>();
		let r = compress(s.as_bytes());
		assert!(r.len() < s.len());
		assert_eq!(v, decompress(&r).ok().unwrap());

		let s = "abcabcabcabc";
		let v = s.as_bytes().iter().cloned().collect::<Vec<_>>();
		let r = compress(s.as_bytes());
		assert!(r.len() < s.len());
		assert_eq!(v, decompress(&r).ok().unwrap());

		let s = "ABRACADABRABRABRA";
		let v = s.as_bytes().iter().cloned().collect::<Vec<_>>();
		let r = compress(s.as_bytes());
		// assert!(r.len() < s.len()); // 18 > 17
		assert_eq!(v, decompress(&r).ok().unwrap());

		let s = "abcdefghijklmnopqrstuvwxyz";
		let v = s.as_bytes().iter().cloned().collect::<Vec<_>>();
		let r = compress(s.as_bytes());
		// assert!(r.len() < s.len()); // 39 > 26
		assert_eq!(v, decompress(&r).ok().unwrap());
	}
}
