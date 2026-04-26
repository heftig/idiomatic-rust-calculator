#![no_std]
#![no_main]

use core::ffi::{c_char, c_int, c_void};
use core::panic::PanicInfo;

extern "C" {
    fn puts(msg: *const c_char);
    fn fgets(s: *mut c_char, n: c_int, stream: *mut c_void) -> *mut c_char;

    static mut stdin: *mut c_void;
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    puts("panic or sth\0".as_ptr() as *const c_char);
    loop {}
}

#[no_mangle]
unsafe extern "C" fn _ZN4core9panicking30panic_null_pointer_dereference17h602c36c59c471956E() {}

#[no_mangle]
unsafe extern "C" fn main() -> i32 {
    puts("Hello, from calculator!\0".as_ptr() as *const c_char);
    loop {
        const BUF_SIZE: usize = 128;
        let mut buf = [0u8; BUF_SIZE];
        let ptr = buf.as_mut_ptr() as *mut c_char;
        
        fgets(ptr, BUF_SIZE as c_int, stdin);
        puts(ptr);
    }
}
