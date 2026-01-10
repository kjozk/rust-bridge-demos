use rust_bridge_core::api::evaluate;
use rust_bridge_core::domain::shape::{Rectangle, AreaResult};
use std::slice;
use std::ptr;

use rmp_serde::{from_slice, to_vec};

pub fn calc_area_msgpack(input: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // 1. deserialize
    let rect: Rectangle = from_slice(input)?;

    // 2. call core
    let result: AreaResult = evaluate::calc_area(rect);

    // 3. serialize
    let output = to_vec(&result)?;

    Ok(output)
}

#[unsafe(no_mangle)]
pub extern "C" fn calc_area_msgpack_ffi(
    input_ptr: *const u8,
    input_len: usize,
    output_ptr: *mut *mut u8,
    output_len: *mut usize,
) -> i32 {
    if input_ptr.is_null() || output_ptr.is_null() || output_len.is_null() {
        return -1;
    }

    let input = unsafe { slice::from_raw_parts(input_ptr, input_len) };

    let rect: Rectangle = match from_slice(input) {
        Ok(v) => v,
        Err(_) => return -2,
    };

    let result: AreaResult = evaluate::calc_area(rect);

    let output = match to_vec(&result) {
        Ok(v) => v,
        Err(_) => return -3,
    };

    unsafe {
        let buf = libc::malloc(output.len()) as *mut u8;
        if buf.is_null() {
            return -4;
        }

        ptr::copy_nonoverlapping(output.as_ptr(), buf, output.len());
        *output_ptr = buf;
        *output_len = output.len();
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn free_buffer(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe { libc::free(ptr as *mut libc::c_void) }
    }
}

#[test]
fn test_msgpack_bridge() {
    let rect = Rectangle { width: 10.0, height: 10.0 };
    let input = rmp_serde::to_vec(&rect).unwrap();

    let output = calc_area_msgpack(&input).unwrap();
    let result: AreaResult = rmp_serde::from_slice(&output).unwrap();

    assert_eq!(result.area, 100.0);
}
