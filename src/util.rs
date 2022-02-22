use std::ffi::CString;

/// Allocates a buffer for a C string of len
/// A C string is a null-terminated array of bytes
/// # Example
/// ```
/// let c_str = rust_string_to_c_string(12);
/// ```
pub fn alloc_cstring_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1); // Erro buffer
    buffer.extend([b' '].iter().cycle().take(len as usize)); // fill buffer with spaces
    let str: CString = unsafe { CString::from_vec_unchecked(buffer) }; // creates CString to receive error
    str
}

pub fn sizeof<T>(vet: &Vec<T>) -> usize {
    vet.len() * std::mem::size_of::<T>()
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        CString::new($s).unwrap()
    };
}