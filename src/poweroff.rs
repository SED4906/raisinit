#![no_std]
#![no_main]
use core::panic::PanicInfo;
use raisinlib::println;
use syscalls::{syscall, Sysno};

#[no_mangle]
pub unsafe extern "C" fn _start() {
    let _ = syscall!(Sysno::sync);
    let _ = syscall!(Sysno::reboot, 0xfee1dead, 0x28121969, 0x4321FEDC, 0);
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    println!("init: panic!!!");
    loop {}
}
