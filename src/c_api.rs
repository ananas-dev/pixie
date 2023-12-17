use std::ffi::{c_char, CStr};

use crate::solver::solve_dc;

#[repr(C)]
pub struct OpResult {
    data: *mut f64,
    len: usize,
}

#[no_mangle]
pub extern "C" fn solve_netlist(input: *const c_char) -> OpResult {
    let input = unsafe { CStr::from_ptr(input) };

    let net = input.to_str().unwrap().parse().expect("Invalid netlist");

    let mut x = solve_dc(&net).expect("Unsolvable circuit").to_vec();
    let ptr = x.as_mut_ptr();
    let ptr_len = x.len();

    std::mem::forget(x);

    OpResult { data: ptr, len: ptr_len }
}