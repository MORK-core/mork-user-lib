#![no_std]
#![feature(thread_local)]
extern crate alloc;

use mork_common::constants::CNodeSlot;
use mork_common::mork_user_log;
use crate::mork_ipc::mork_notification_signal;
use crate::mork_ipc_buffer::ipc_buffer_init;
use crate::mork_task::mork_task_suspend;
use crate::mork_tls::tls_init;

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
pub mod mork_thread;

unsafe extern "C" {
    fn main();
}

fn init() {
    logging::init();
    heap::init().unwrap();
}

#[unsafe(no_mangle)]
pub fn entry(_arg: usize) {
    init();
    if let Err(()) = tls_init() {
        mork_user_log!(error, "mork-root-task failed to initialize TLS context!");
        return;
    }
    if let Err(()) = ipc_buffer_init(CNodeSlot::CapInitThread as usize) {
        mork_user_log!(error, "mork-root-task ipc_buffer_init failed");
        return;
    }
    unsafe {
        main();
    }
    let _ = mork_notification_signal(CNodeSlot::CapParentCom as usize);
    mork_task_suspend(CNodeSlot::CapInitThread as usize).unwrap();
}

pub fn mork_shutdown() {
    hal::sys_shutdown();
}

pub fn dummy_function() {

}
