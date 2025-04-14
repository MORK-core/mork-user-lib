#![no_std]
#![feature(thread_local)]
extern crate alloc;

use mork_common::constants::CNodeSlot;
use crate::mork_ipc::mork_notification_signal;
use crate::mork_task::mork_thread_suspend;

pub mod hal;
pub mod console;
mod logging;
pub mod mork_mm;
pub mod mork_task;
pub mod mork_cspace;
pub mod mork_ipc;
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
    let _ = mork_notification_signal(CNodeSlot::CapParentCom as usize);
    mork_thread_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}

pub fn dummy_function() {

}
