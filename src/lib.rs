#![no_std]

pub mod riscv;
pub mod console;
mod logging;

pub use riscv::syscall::sys_put_char;

pub fn log_init() {
    logging::init();
}