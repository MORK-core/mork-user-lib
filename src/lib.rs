#![no_std]
#![feature(thread_local)]
extern crate alloc;

use mork_common::constants::CNodeSlot;
use crate::mork_task::mork_thread_suspend;

pub mod hal;
pub mod console;
mod logging;
pub mod mork_mm;
pub mod mork_task;
pub mod mork_tls;
pub mod mork_ipc_buffer;
mod lang_item;
mod heap;

unsafe extern "C" {
    fn main();
}

fn init() {
    logging::init();
    heap::init().unwrap();
}

#[unsafe(no_mangle)]
pub fn entry() {
    init();
    unsafe {
        main();
    }
    mork_thread_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}