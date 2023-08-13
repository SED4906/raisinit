#![no_std]
#![no_main]

use core::panic::PanicInfo;
use raisinlib::{println,exec::spawn};
use syscalls::{syscall, Sysno};

#[no_mangle]
pub extern "C" fn _start(argc: i32, argv: *const *const u8) -> i32 {
    if argc > 1 {
        loop {
            if let Ok(pid) = unsafe{syscall!(Sysno::fork)} {
                if pid == 0 {
                    unsafe{syscall!(Sysno::execve,*argv.add(1),argv.add(2),0)};
                } else {
                    unsafe{syscall!(Sysno::waitid,1,pid,0,0)};
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