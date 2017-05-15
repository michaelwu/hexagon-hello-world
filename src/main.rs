// A no_std hello world example for Hexagon and HVX

#![feature(lang_items, core_intrinsics)]
#![feature(repr_simd, platform_intrinsics)]
#![feature(start)]
#![no_std]
use core::intrinsics;

#[link(name="standalone", kind="static")]
#[link(name="hexagon", kind="static")]
#[link(name="c", kind="static")]
extern {
    pub fn puts(s: *const u8) -> i32;
    pub fn write(fd: i32, buf: *const u8, count: i32) -> i32;
}

struct StdoutWriter;

use core::fmt::{Write, Error};
impl Write for StdoutWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        unsafe {
            write(0, s.as_ptr(), s.len() as i32);
        }
        Ok(())
    }
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n\r")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n\r"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        let mut writer = StdoutWriter;
        writer.write_fmt(format_args!($($arg)*)).unwrap()
    });
}

mod hvx;

extern {
    pub fn set_double_vector_mode();
    pub fn acquire_vector_unit(wait: i32) -> i32;
    pub fn release_vector_unit();

    pub fn power_vector_unit(clockbase: *const u32, resetbase: *const u32, powerbase: *const u32, delay: i32, state: i32);
}

struct HVXContext;

impl HVXContext {
    pub fn get_128b() -> HVXContext {
        unsafe {
            let subsys_base = 0xFE200000u32;
            power_vector_unit((subsys_base + 0xFC) as *const u32,
                              (subsys_base + 0xF8) as *const u32,
                              (subsys_base + 0xF0) as *const u32,
                              100, 1);
            if acquire_vector_unit(0) == 0 {
                panic!("couldn't get vector unit!");
            }
            set_double_vector_mode();
        }
        HVXContext
    }
}

impl Drop for HVXContext {
    fn drop(&mut self) {
        unsafe {
            release_vector_unit();
        }
    }
}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello world!");

    let _ctx = HVXContext::get_128b();
    let mut buf: [i32; 32] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    print!("buf:");
    for v in &buf {
        print!(" {}", v);
    }
    print!("\n");

    let tmp = hvx::i32x32::load(&buf, 0);
    let tmp = unsafe { hvx::Q6_Vw_vmpyi_VwRb128(tmp, 0x03030303) };
    tmp.store(&mut buf, 0);

    print!("buf:");
    for v in &buf {
        print!(" {}", v);
    }
    print!("\n");
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[allow(unused_must_use)]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    let mut writer = StdoutWriter;
    core::fmt::write(&mut writer, msg);
    writer.write_str("\n");
    unsafe { intrinsics::abort() }
}
