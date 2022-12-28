use shogi_kifu_converter::{
    converter::{ToCsa, ToKi2, ToKif},
    error::ParseError,
    jkf::JsonKifuFormat,
    parser::{parse_csa_str, parse_jkf_str, parse_ki2_str, parse_kif_str},
};
use std::{fmt::Error};

#[no_mangle]
pub extern "C" fn main() {}

// =====================================================================================

fn parse(
    parser: fn(&str) -> Result<JsonKifuFormat, ParseError>,
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    let src = unsafe { std::ffi::CStr::from_ptr(src) };
    let src = match src.to_str() {
        Err(_) => {
            return -2; //invalid utf-8 string
        }
        Ok(str) => str,
    };
    let result = match parser(src) {
        Err(_) => {
            return -3; //invalid format
        }
        Ok(result) => result,
    };
    let result = match serde_json::to_string(&result) {
        Err(_) => {
            return -1; //internal error
        }
        Ok(result) => result,
    };
    let result = match std::ffi::CString::new(result) {
        Err(_) => {
            return -1; //internal error
        }
        Ok(result) => result,
    };
    let result = result.as_bytes_with_nul();
    if (result.len() as i32) > dst_size {
        return result.len() as i32; //dst is too small
    }
    for i in 0..result.len() {
        unsafe {
            *dst.offset(i as isize) = result[i] as i8;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn parse_kif(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return parse(parse_kif_str, src, dst, dst_size);
}
#[no_mangle]
pub extern "C" fn parse_ki2(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return parse(parse_ki2_str, src, dst, dst_size);
}

#[no_mangle]
pub extern "C" fn parse_csa(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return parse(parse_csa_str, src, dst, dst_size);
}

#[no_mangle]
pub extern "C" fn parse_jkf(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return parse(parse_jkf_str, src, dst, dst_size);
}

// =====================================================================================
fn convert(
    converter: fn(&JsonKifuFormat, &mut String) -> Result<(), Error>,
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    let src = unsafe { std::ffi::CStr::from_ptr(src) };
    let src = match src.to_str() {
        Err(_) => {
            return -2; //invalid utf-8 string
        }
        Ok(str) => str,
    };
    let src: JsonKifuFormat = match serde_json::from_str(src) {
        Err(_) => {
            return -3; //invalid format
        }
        Ok(src) => src,
    };
    let mut result: String = "".to_string();
    match converter(&src, &mut result) {
        Err(_) => {
            return -1; // internal error
        }
        Ok(_) => {}
    };
    //ヌル文字が含まれていないかチェック
    for i in 0..result.len() {
        if result.as_bytes()[i] == 0 {
            return -1; //internal error
        }
    }
    //末尾にヌル文字を追加
    result.push('\0');

    let result = result.as_bytes();

    if (result.len() as i32) > dst_size {
        return result.len() as i32; //dst is too small
    }
    for i in 0..result.len() {
        unsafe {
            *dst.offset(i as isize) = result[i] as i8;
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn to_kif(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return convert(ToKif::to_kif, src, dst, dst_size);
}
#[no_mangle]
pub extern "C" fn to_ki2(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return convert(ToKi2::to_ki2, src, dst, dst_size);
}

#[no_mangle]
pub extern "C" fn to_csa(
    src: *const std::ffi::c_char,
    dst: *mut std::ffi::c_char,
    dst_size: i32,
) -> i32 {
    return convert(ToCsa::to_csa, src, dst, dst_size);
}
