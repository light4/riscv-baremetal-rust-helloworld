#![feature(llvm_asm)]
#![no_main]
#![no_std]

extern crate panic_halt;
extern crate riscv_rt;

pub mod uart;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut my_uart = uart::Uart::new(0x1000_0000);
    my_uart.init();

    println!("Hello World!");

    loop {}
}

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print
{
    ($($args:tt)+) => ({
            use core::fmt::Write;
            let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}
