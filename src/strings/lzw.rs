use super::bitio::*;
use super::tst_u8::*;

const R: usize = 256;
const SYMBOL_BITS: usize = 12;
const MAX_SYMBOLS: usize = 1 << SYMBOL_BITS;

pub fn compress(input: &[u8]) -> Vec<u8> {
	let mut tst = new_symbol_tst();
	let mut io = MemBitIO::new();
	let mut code = R;

	let mut p = input;
	while !p.is_empty() {
		let (n, symbol) = tst.longest_match(p).unwrap();
		io.write(*symbol, SYMBOL_BITS);

		if n < p.len() && code < MAX_SYMBOLS {
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
	let v = input.read(SYMBOL_BITS).ok_or("invalid input")?;
	let mut last = vec![v as u8; 1];
	let mut out = last.clone();

	while let Some(symbol) = input.read(SYMBOL_BITS) {
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

		if st.len() < MAX_SYMBOLS {
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
	// Optimize to balance the TST, otherwise it's height is 256
	// for i in 0..R {
	// 	w[0] = i as u8;
	// 	tst.put(&w, i);
	// }
	put_range(&mut tst, &mut w, 0, R);

	tst
}
fn put_range(tst: &mut TST<usize>, w: &mut Vec<u8>, lo: usize, hi: usize) {
	let i = lo + (hi - lo) / 2;
	w[0] = i as u8;
	tst.put(&w, i);

	if lo < i {
		put_range(tst, w, lo, i);
	}
	if i + 1 < hi {
		put_range(tst, w, i + 1, hi);
	}
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

	#[test]
	fn non_string() {
		let v: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
		let r = compress(&v[..]);
		assert_eq!(9, r.len());
		assert_eq!(v, decompress(&r).ok().unwrap());

		let v: Vec<u8> = vec![0; 512 * 1024];
		let r = compress(&v[..]);
		assert_eq!(1536, r.len()); // 0.29%
		assert_eq!(v, decompress(&r).ok().unwrap());
	}
}
