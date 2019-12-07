#![feature(test)] // #[bench] is still experimental

extern crate test; // Even in '18 this is needed ... for reasons.
				   // Normally you don't need this in '18 code.

use rand::{thread_rng, Rng};
use std::iter::repeat;
use test::{black_box, Bencher}; // `black_box` prevents `f` from being optimized away.

use algs4_rs::sorting::*;

#[bench]
fn sort_int7_base(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut _arr = arr.clone();
	});
}

#[bench]
fn sort_int7_insertion(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(insertion_sort(&mut arr))
	});
}

#[bench]
fn sort_int7_merge(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort(&mut arr))
	});
}

#[bench]
fn sort_int7_merge_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int7_quick(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort(&mut arr))
	});
}

#[bench]
fn sort_int7_quick3way(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick3way_sort(&mut arr))
	});
}

#[bench]
fn sort_int7_quick_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int7_heap(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort(&mut arr))
	});
}

#[bench]
fn sort_int7_heap_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(7).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int15_base(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut _arr = arr.clone();
	});
}

#[bench]
fn sort_int15_insertion(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(insertion_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_shell(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(shell_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_merge(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_merge_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int15_quick(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_quick3way(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick3way_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_quick_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int15_heap(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort(&mut arr))
	});
}

#[bench]
fn sort_int15_heap_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(15).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int100_base(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut _arr = arr.clone();
	});
}

#[bench]
fn sort_int100_insertion(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(insertion_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_shell(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(shell_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_merge(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_merge_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int100_quick(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_quick3way(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick3way_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_quick_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int100_heap(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort(&mut arr))
	});
}

#[bench]
fn sort_int100_heap_opt(b: &mut Bencher) {
	let arr = thread_rng().gen_iter::<u32>().take(100).collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int10k_base(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(1000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut _arr = arr.clone();
	});
}

#[bench]
fn sort_int10k_insertion(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(insertion_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_shell(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(shell_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_merge(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_merge_opt(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int10k_quick(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_quick3way(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick3way_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_quick_opt(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int10k_heap(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort(&mut arr))
	});
}

#[bench]
fn sort_int10k_heap_opt(b: &mut Bencher) {
	let arr = thread_rng()
		.gen_iter::<u32>()
		.take(10000)
		.collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int100_same_insertion(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(insertion_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_shell(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(shell_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_merge(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(merge_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_merge_opt(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(merge_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_almost_same_quick_opt(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	arr[99] = 100;
	b.iter(|| black_box(quick_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_same_quick(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_quick3way(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick3way_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_quick_opt(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_same_heap(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(heap_sort(&mut arr)));
}

#[bench]
fn sort_int100_same_heap_opt(b: &mut Bencher) {
	let mut arr = repeat(23u32).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(heap_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_sorted_insertion(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(insertion_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_shell(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(shell_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_merge(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(merge_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_merge_opt(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(merge_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_sorted_quick(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_quick3way(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick3way_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_quick_opt(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(quick_sort_opt(&mut arr)));
}

#[bench]
fn sort_int100_sorted_heap(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(heap_sort(&mut arr)));
}

#[bench]
fn sort_int100_sorted_heap_opt(b: &mut Bencher) {
	let mut arr = (0..100).take(100).collect::<Vec<_>>();
	b.iter(|| black_box(heap_sort_opt(&mut arr)));
}

#[bench]
fn sort_int1k_reversed_base(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut _arr = arr.clone();
	});
}

#[bench]
fn sort_int1k_reversed_insertion(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(insertion_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_shell(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(shell_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_merge(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_merge_opt(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(merge_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_quick(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_quick3way(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick3way_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_quick_opt(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(quick_sort_opt(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_heap(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort(&mut arr))
	});
}

#[bench]
fn sort_int1k_reversed_heap_opt(b: &mut Bencher) {
	let arr = (0..1000).rev().collect::<Vec<_>>();
	b.iter(|| {
		let mut arr = arr.clone();
		black_box(heap_sort_opt(&mut arr))
	});
}
