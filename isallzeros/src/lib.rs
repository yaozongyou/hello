#![feature(portable_simd)]

use std::simd::{Simd, SimdUint};

pub fn is_all_zeros_stupid(data: &[u8]) -> bool {
    for i in 0..data.len() {
        if data[i] != 0 {
            return false;
        }
    }

    true
}

pub fn is_all_zeros_iter(data: &[u8]) -> bool {
    data.iter().all(|&i| i == 0)
}

pub fn is_all_zeros_fast(data: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { data.align_to::<u128>() };
    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

pub fn is_all_zeros_simd(data: &[u8]) -> bool {
    let (prefix, aligned, suffix) = unsafe { data.align_to::<Simd<usize, 64>>() };
    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x.reduce_or() == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = vec![0; 1024];

        assert!(is_all_zeros_stupid(&data));
        assert!(is_all_zeros_iter(&data));
        assert!(is_all_zeros_fast(&data));
        assert!(is_all_zeros_simd(&data));

        data.push(1);

        assert!(!is_all_zeros_stupid(&data));
        assert!(!is_all_zeros_iter(&data));
        assert!(!is_all_zeros_fast(&data));
        assert!(!is_all_zeros_simd(&data));
    }
}
