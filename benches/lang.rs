#![feature(test)] // #[bench] is still experimental

extern crate test; // Even in '18 this is needed ... for reasons.
                   // Normally you don't need this in '18 code.

use test::{black_box, Bencher}; // `black_box` prevents `f` from being optimized away.

// swap by temp variable is no difference for primitive types
#[bench]
fn lang_int_swap_by_copy(b: &mut Bencher) {
    b.iter(|| {
        let mut arr = [1u32, 2u32];
        let temp = arr[0];
        black_box(arr[0] = arr[1]);
        arr[1] = temp
    });
}

#[bench]
fn lang_int_swap_inplace(b: &mut Bencher) {
    b.iter(|| {
        let mut arr = [1u32, 2u32];
        black_box(arr.swap(0, 1))
    });
}

// swap by temp is clone, which requires memory allocation, the cost can't be ignored
// the good thing about Rust is it makes the clone/copy explicit
#[bench]
fn lang_string_swap_by_copy(b: &mut Bencher) {
    b.iter(|| {
        let mut arr = [
            String::from("1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
            String::from("0987654321ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
        ];
        let temp = arr[0].clone();
        black_box(arr[0] = arr[1].clone());
        arr[1] = temp
    });
}

#[bench]
fn lang_string_swap_by_empty_temp(b: &mut Bencher) {
    b.iter(|| {
        let mut arr = [
            String::from("1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
            String::from("0987654321ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
        ];
        // let temp = arr[0]; // cannot move here!
        let mut temp = String::new();
        std::mem::swap(&mut temp, &mut arr[0]);
        black_box(arr.swap(0, 1));
        std::mem::swap(&mut arr[1], &mut temp)
    });
}

#[bench]
fn lang_string_swap_inplace(b: &mut Bencher) {
    b.iter(|| {
        let mut arr = [
            String::from("0987654321ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
            String::from("1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuwwxz"),
        ];
        black_box(arr.swap(0, 1))
    });
}
