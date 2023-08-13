#![no_std]
#![no_main]
use core::{panic::PanicInfo, ops::Add};
use raisin_std::{println, io::{File,Read, Utf8String},exec::execute};
use syscalls::{syscall, Sysno};

#[no_mangle]
pub extern "C" fn _start() {
    println!("raisINIT");
    let mut buf: [u8; 512] = [0; 512];
    let mut args: [*const u8; 512] = [0 as *const _; 512];
    let mut file = File::open("/etc/raisinrc\0").unwrap();
    file.read(&mut buf);
    file.close();
    if let Ok(pid) = unsafe{syscall!(Sysno::fork)} {
        if pid == 0 {
            let mut i = 0;
            for arg in buf.split(|c| c==&b' ') {
                i += 1;
                if i > 0 {
                    args[i-1] = arg.as_ptr();
                }
            }
            for c in &mut buf {
                if c == &b' ' {
                    *c = 0
                }
                
            }
            execute(&buf, &args);
        }
    }
    loop{}
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    println!("raisinit: panic!!!");
    loop{}
}