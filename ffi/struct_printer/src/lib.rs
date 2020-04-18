use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
// Struct to be printed.
pub struct PrintableStruct {
    pub text_ptr: *const c_char,
    pub logical: bool,
    pub short_number: i8,
    pub long_number: i32,
}

#[no_mangle]
// Prints all of the struct fields.
pub extern "C" fn struct_printer(printable_struct: PrintableStruct) -> u32 {
    // We have to be careful with C strings
    let c_str = unsafe {
        assert!(!printable_struct.text_ptr.is_null());
        CStr::from_ptr(printable_struct.text_ptr)
    };
    let r_str = c_str.to_str().unwrap();

    println!("From Rust, content of struct is:");
    println!("Text        : {}", r_str);
    println!("Logical     : {}", printable_struct.logical);
    println!("Short number: {}", printable_struct.short_number);
    println!("Long number : {}", printable_struct.long_number);

    // Number of fields in the struct.
    return 4;
}