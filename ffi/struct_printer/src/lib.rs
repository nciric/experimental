// use std::os::raw::c_char;

#[repr(C)]
// Struct to be printed.
pub struct PrintableStruct {
//    pub text_ptr: *const c_char,
    pub logical: bool,
    pub short_number: i8,
    pub long_number: i32,
}

#[no_mangle]
// Prints all of the struct fields.
pub extern "C" fn struct_printer(printable_struct: PrintableStruct) {
    println!("Logical     : {}", printable_struct.logical);
    println!("Short number: {}", printable_struct.short_number);
    println!("Long number : {}", printable_struct.long_number);
}