#![no_std]
#![no_main]
use core::panic::PanicInfo;
use raisinit::println;
use syscalls::{syscall, Sysno};

#[no_mangle]
pub extern "C" fn _start() {
    println!("testing");
    loop{}
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    let s = "panic\0";
    unsafe { syscall!(Sysno::write, 1, s.as_ptr() as *const _, 6) };
    loop{}
}