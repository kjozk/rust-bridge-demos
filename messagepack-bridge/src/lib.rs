use rust_bridge_core::api::evaluate;
use rust_bridge_core::domain::shape::{Rectangle, AreaResult};
use std::slice;
use std::ptr;

/// MessagePack-bridge の役割:
/// - Rust の型 <-> MessagePack バイト列の相互変換を担う
/// - C 側との FFI 境界でメモリ割当（malloc/free）を明示的に扱う
///   （呼び出し側は free_buffer で解放すること）
/// input_ptr/input_len: MessagePack でシリアライズされた Rectangle
/// output_ptr/output_len: MessagePack でシリアライズされた AreaResult を返すポインタ
/// 戻り値: 0=成功、負値=エラー
#[unsafe(no_mangle)]
pub extern "C" fn calc_area_msgpack_ffi(
    input_ptr: *const u8,
    input_len: usize,
    output_ptr: *mut *mut u8,
    output_len: *mut usize,
) -> i32 {
    // 入力・出力ポインタの検証。null チェックで早期エラーを返す
    if input_ptr.is_null() || output_ptr.is_null() || output_len.is_null() {
        return -1;
    }

    let input = unsafe { slice::from_raw_parts(input_ptr, input_len) };

    // MessagePack のデシリアライズ失敗は -2 を返す
    let rect: Rectangle = match rmp_serde::from_slice(input) {
        Ok(v) => v,
        Err(_) => return -2,
    };

    let result: AreaResult = evaluate::calc_area(rect);

    // MessagePack のシリアライズ失敗は -3 を返す
    let output = match rmp_serde::to_vec_named(&result) {
        Ok(v) => v,
        Err(_) => return -3,
    };

    unsafe {
        // 出力は libc::malloc で確保して返す（呼び出し側で free_buffer を呼ぶこと）
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

/// FFI で返されたバッファを解放するヘルパー
#[unsafe(no_mangle)]
pub extern "C" fn free_buffer(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe { libc::free(ptr as *mut libc::c_void) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmp_serde::to_vec_named;

    // 正常系: Rectangle -> MessagePack -> FFI -> MessagePack -> AreaResult
    #[test]
    fn test_calc_area_msgpack_ffi_success() {
        let rect = Rectangle { width: 3.0, height: 4.0 };
        let input = to_vec_named(&rect).expect("serialize input");

        let mut out_ptr: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;

        let ret = calc_area_msgpack_ffi(input.as_ptr(), input.len(), &mut out_ptr, &mut out_len);
        assert_eq!(ret, 0);
        assert!(!out_ptr.is_null());
        assert!(out_len > 0);

        let out_slice = unsafe { slice::from_raw_parts(out_ptr, out_len) };
        let result: AreaResult = rmp_serde::from_slice(out_slice).expect("deserialize output");

        let expected = evaluate::calc_area(rect);
        assert_eq!(result.area, expected.area);

        // メモリ解放
        free_buffer(out_ptr);
    }

    // 異常系: 不正な MessagePack 入力 -> -2
    #[test]
    fn test_calc_area_msgpack_ffi_invalid_input() {
        let bad = vec![0xFFu8, 0xFFu8];
        let mut out_ptr: *mut u8 = ptr::null_mut();
        let mut out_len: usize = 0;

        let ret = calc_area_msgpack_ffi(bad.as_ptr(), bad.len(), &mut out_ptr, &mut out_len);
        assert_eq!(ret, -2);
        assert!(out_ptr.is_null());
        assert_eq!(out_len, 0);
    }

    // 異常系: null 引数 -> -1
    #[test]
    fn test_calc_area_msgpack_ffi_null_args() {
        let rect = Rectangle { width: 1.0, height: 2.0 };
        let input = to_vec_named(&rect).expect("serialize input");
        let mut out_len: usize = 0;
        // input_ptr が null
        let ret1 = calc_area_msgpack_ffi(ptr::null(), 0, &mut (ptr::null_mut()), &mut out_len);
        assert_eq!(ret1, -1);

        // output_ptr が null
        let ret2 = calc_area_msgpack_ffi(input.as_ptr(), input.len(), ptr::null_mut(), &mut out_len);
        assert_eq!(ret2, -1);

        // output_len が null
        let mut out_ptr: *mut u8 = ptr::null_mut();
        let ret3 = calc_area_msgpack_ffi(input.as_ptr(), input.len(), &mut out_ptr, ptr::null_mut());
        assert_eq!(ret3, -1);
    }
}
