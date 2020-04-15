use super::super::sorting::MinPQ;
use super::bitio::{BitReader, BitWriter};

use std::cmp::Ordering;
use std::collections::BTreeMap;

const LENGTH_BITS: usize = std::mem::size_of::<usize>() * 8;

pub fn compress(input: &str, w: &mut impl BitWriter) {
	if input.is_empty() {
		return;
	}

	let mut freq: [usize; 256] = [0; 256];
	for c in input.chars() {
		freq[c as usize] += 1;
	}

	let root = build_tree(&freq);

	let mut ct = BTreeMap::new();
	let mut prefix: Vec<u8> = Vec::new();
	build_code_table(&root, &mut prefix, &mut ct);

	write_tree(&root, w);
	w.write(input.len(), LENGTH_BITS);
	for c in input.chars() {
		let code = ct.get(&c).unwrap();
		for &b in code.iter() {
			w.write(b as usize, 1);
		}
	}
}
pub fn decompress(r: &mut impl BitReader) -> Result<String, String> {
	let root = read_tree(r)?;
	let len = r.read(LENGTH_BITS).ok_or("incomplete length")?;

	let mut s = String::new();
	for _ in 0..len {
		let mut p = &root;
		loop {
			match &p.kind {
				Kind::Leaf(c) => {
					s.push(*c as char);
					break;
				}
				Kind::Internal((left, right)) => {
					let b = r.read(1).ok_or("incomplete data")?;
					if b == 0 {
						p = &left;
					} else {
						p = &right;
					}
				}
			}
		}
	}

	Ok(s)
}

fn build_tree(freq: &[usize; 256]) -> NodePtr {
	let mut pq = MinPQ::new();
	for (c, &f) in freq.iter().enumerate() {
		if f > 0 {
			pq.push(Node::new_leaf(f, (c as u8) as char));
		}
	}

	while pq.len() > 1 {
		let left = pq.pop().unwrap();
		let right = pq.pop().unwrap();
		pq.push(Node::new(
			left.freq + right.freq,
			Box::new(left),
			Box::new(right),
		));
	}

	Box::new(pq.pop().unwrap())
}

fn build_code_table(p: &NodePtr, prefix: &mut Vec<u8>, ct: &mut BTreeMap<char, Vec<u8>>) {
	match &p.kind {
		Kind::Leaf(c) => {
			//TODO: bits compressing?
			ct.insert(*c, prefix.clone());
		}
		Kind::Internal((left, right)) => {
			prefix.push(0);
			build_code_table(left, prefix, ct);
			prefix.pop();
			prefix.push(1);
			build_code_table(right, prefix, ct);
			prefix.pop();
		}
	}
}

fn write_tree(p: &NodePtr, w: &mut impl BitWriter) {
	match &p.kind {
		Kind::Leaf(c) => {
			w.write(1, 1);
			w.write(*c as usize, 8);
		}
		Kind::Internal((left, right)) => {
			w.write(0, 1);
			write_tree(&left, w);
			write_tree(&right, w);
		}
	}
}
fn read_tree(r: &mut impl BitReader) -> Result<NodePtr, String> {
	let b = r.read(1).ok_or("incomplete tree")?;
	let node = if b == 1 {
		let c = r.read(8).ok_or("incomplete leaf node")?;
		Node::new_leaf(0, (c as u8) as char)
	} else {
		let left = read_tree(r)?;
		let right = read_tree(r)?;
		Node::new(0, left, right)
	};

	Ok(Box::new(node))
}

#[derive(PartialEq, Eq)]
enum Kind {
	Leaf(char),
	Internal((NodePtr, NodePtr)),
}
#[derive(PartialEq, Eq)]
struct Node {
	freq: usize,
	kind: Kind,
}
type NodePtr = Box<Node>;
impl Node {
	fn new_leaf(freq: usize, c: char) -> Self {
		Self {
			freq,
			kind: Kind::Leaf(c),
		}
	}
	fn new(freq: usize, left: Box<Node>, right: Box<Node>) -> Self {
		Self {
			freq,
			kind: Kind::Internal((left, right)),
		}
	}
}
impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		self.freq.cmp(&other.freq)
	}
}
impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.freq.partial_cmp(&other.freq)
	}
}

#[cfg(test)]
mod tests {
	use super::super::bitio::MemBitIO;
	use super::*;

	#[test]
	fn empty() {
		let mut io = MemBitIO::new();
		compress("", &mut io);
		assert_eq!(0, io.len());
		assert!(decompress(&mut io).is_err());
	}

	#[test]
	fn basic() {
		let mut io = MemBitIO::new();
		compress("a", &mut io);
		assert_eq!(10, io.len());
		assert_eq!(Ok("a".into()), decompress(&mut io));

		compress("aaa", &mut io);
		assert_eq!(10, io.len());
		assert_eq!(Ok("aaa".into()), decompress(&mut io));

		compress("aaabbc", &mut io);
		assert_eq!(13, io.len());
		assert_eq!(Ok("aaabbc".into()), decompress(&mut io));

		let all_unique = "abcdefghijklmnopqrstuvwxyz";
		compress(all_unique, &mut io);
		assert_eq!(56, io.len());
		assert_eq!(Ok(all_unique.into()), decompress(&mut io));

		let s = "this is an example for huffman encoding";
		compress(s, &mut io);
		assert_eq!(52, io.len());
		assert_eq!(Ok(s.into()), decompress(&mut io));
	}
}
