#![feature(start)]
#![no_std]

use core::panic::PanicInfo;

mod io;

#[start]
fn _start(_c: isize, _v: *const *const u8) -> isize {
    println!("* raisINIT *");
    loop{}
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    println!("raisINIT: panic!!!");
    loop{}
}