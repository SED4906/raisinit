#![no_std]
#![no_main]
use core::{panic::PanicInfo};
use raisinlib::{println, io::{File,Read},exec::spawn};
use syscalls::{syscall, Sysno};


const MAX_FILE_SIZE: usize = 2048;

#[no_mangle]
pub extern "C" fn _start() {
    println!("raisINIT");
    let mut buf: [u8; MAX_FILE_SIZE+1] = [0; MAX_FILE_SIZE+1];
    let mut args: [*const u8; MAX_FILE_SIZE+1] = [core::ptr::null(); MAX_FILE_SIZE+1];
    let mut file = File::open("/etc/raisinrc\0").unwrap();
    let actual_size = file.read(&mut buf).unwrap();
    file.close();
    let mut buf_line: [u8; MAX_FILE_SIZE+1] = [0; MAX_FILE_SIZE+1];
    #[allow(unused_assignments)]
    let mut line_start = 0;
    let mut line_end = 0;
    for i in 0..actual_size {
        match buf[i] {
            0|b'\r'|b'\n' => {
                line_start = line_end;
                line_end = i;
                if line_end-line_start <= 1 {
                    line_end = i+1;
                    continue;
                }
                buf_line.fill(0);
                args.fill(core::ptr::null());
                buf_line[..(line_end - line_start)].copy_from_slice(&buf[line_start..line_end]);
                split_into_cmdline(&mut buf_line, &mut args);
                let _ = spawn(&buf_line, &args);
            },
            _ => {}
        }
    }
    loop{unsafe{let _ = syscall!(Sysno::pause);}}
}

#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    println!("init: panic!!!");
    loop{}
}

fn split_into_cmdline(buf: &mut [u8], args: &mut [*const u8]) {
    let mut i = 0;
    for arg in buf.split(|c| c==&b' ') {
        i += 1;
        if i > 0 {
            args[i-1] = arg.as_ptr();
        }
    }
    for c in buf {
        if c == &b' ' {
            *c = 0
        }
    }
}