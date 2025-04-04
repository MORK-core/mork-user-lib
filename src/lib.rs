#![no_std]
#![feature(thread_local)]
extern crate alloc;

pub mod hal;
pub mod console;
mod logging;
pub mod mork_alloc;
pub mod mork_mm;
pub mod mork_task;
pub mod mork_tls;
pub mod mork_ipc_buffer;

pub fn log_init() {
    logging::init();
}