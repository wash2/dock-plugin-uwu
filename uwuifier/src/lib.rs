use std::ffi::CStr;
use std::os::raw::c_char;
use uwuifier::uwuify_str_sse;

#[no_mangle]
pub extern "C" fn uwu_uwu(text_ptr: *const c_char, buffer_ptr: *mut c_char, length: u32) -> bool {
    if !text_ptr.is_null() && !buffer_ptr.is_null() {
        let length: usize = length.try_into().unwrap();
        let text = unsafe { CStr::from_ptr(text_ptr).to_string_lossy() };
        let uwu = uwuify_str_sse(&text);
        let buffer =
            unsafe { std::slice::from_raw_parts_mut(buffer_ptr, length.try_into().unwrap()) };
        if uwu.len() > length {
            let i8uwu = unsafe { &*(&uwu.as_bytes()[..length] as *const _ as *const [i8]) };
            buffer.copy_from_slice(i8uwu);
        } else {
            let i8uwu = unsafe { &*(&uwu.as_bytes()[..] as *const _ as *const [i8]) };
            buffer[..uwu.len()].copy_from_slice(i8uwu);
        }
        return true;
    }
    return false;
}
