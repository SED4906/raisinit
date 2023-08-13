#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};
use raisinlib::{println,exec::spawn};
use syscalls::{syscall, Sysno};

#[no_mangle]
pub extern "C" fn _start() {
    unsafe{asm!(
        "pop rbp;", // C compiler will push rbp
        "pop rdi;",        // argc
        "mov rsi,rsp;", // argv
        "and rsp, -16;",
        "call main;",
    )};
}

#[no_mangle]
pub extern "C" fn main(argc: i32, argv: *const *const u8) -> i32 {
    if argc > 1 {
        loop {
            if let Ok(pid) = unsafe{syscall!(Sysno::fork)} {
                if pid == 0 {
                    unsafe{syscall!(Sysno::execve,*argv.add(1),argv.add(2),0)};
                } else {
                    unsafe{syscall!(Sysno::wait4,pid,0,0,0)};
                }
            }
        }
    }
    -1
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    println!("respawn: panic!!!");
    loop{}
}