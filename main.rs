#![no_std]
#![no_main]

mod libc {
    use core::ffi::{c_char, c_int};

    enum Void {}

    #[repr(C)]
    pub struct File(Void);

    unsafe extern "C" {

        pub fn printf(fmt: *const c_char, ...);
        pub fn fgets(s: *mut c_char, n: c_int, stream: *mut File) -> *mut c_char;

        pub fn abort() -> !;

        pub static mut stdin: *mut File;
    }
}

mod eval;

use core::ffi::CStr;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { libc::abort() };
}

#[unsafe(no_mangle)]
unsafe extern "C" fn main() -> i32 {
    unsafe { libc::printf(c"Hello, from calculator!\n".as_ptr()) };

    loop {
        let mut buf = [0u8; 128];

        unsafe {
            if libc::fgets(buf.as_mut_ptr() as _, buf.len() as _, libc::stdin).is_null() {
                break;
            }
        }

        let expr = CStr::from_bytes_until_nul(&buf).unwrap_or_default();
        let result = unsafe { eval::eval(expr.as_ptr()) };

        unsafe { libc::printf(c"result: %d\n".as_ptr(), result) };
    }

    0
}
