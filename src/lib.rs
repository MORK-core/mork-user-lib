#![no_std]
extern crate alloc;

pub mod riscv;
pub mod console;
mod logging;
pub mod mork_alloc;
pub mod mork_mm;
pub mod mork_task;

pub use riscv::syscall::sys_put_char;

pub fn log_init() {
    logging::init();
}