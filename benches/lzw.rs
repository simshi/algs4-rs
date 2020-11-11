#![feature(test)] // #[bench] is still experimental
extern crate test;

use rand::{thread_rng, Rng};
use std::iter::repeat;
use test::{black_box, Bencher};

use algs4_rs::strings::lzw::*;

#[bench]
fn lzw_compress_100_trie_st(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u8>().take(100).collect::<Vec<_>>();
	b.iter(|| black_box(compress(&arr)));
}

#[bench]
fn lzw_compress_100k_trie_st(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u8>()
		.take(100 * 1024)
		.collect::<Vec<_>>();
	b.iter(|| black_box(compress(&arr)));
}

#[bench]
fn lzw_compress_100k_zero_trie_st(b: &mut Bencher) {
	let arr = repeat(0).take(100 * 1024).collect::<Vec<_>>();
	b.iter(|| black_box(compress(&arr)));
}
