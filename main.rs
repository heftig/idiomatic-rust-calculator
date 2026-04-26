#![no_std]
#![no_main]

use core::ffi::{c_char};
use core::panic::PanicInfo;

extern "C" {
    fn puts(msg: *const c_char);
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    puts("panic or sth\0".as_ptr() as *const c_char);
    loop {}
}

#[no_mangle]
unsafe extern "C" fn main() -> i32 {
    puts("Hello, from calculator!\0".as_ptr() as *const c_char);
    return 0;
}
