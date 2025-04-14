use core::cell::RefCell;
use mork_common::constants::{CNodeSlot, ObjectType};
use mork_common::mork_user_log;
use mork_common::syscall::ipc_buffer::IPCBuffer;
use mork_common::types::{JustResult, VMRights};
use crate::mork_mm::mork_map_frame_anyway;
use crate::mork_task::{mork_alloc_object, mork_thread_set_ipc_buffer};

#[thread_local]
static IPC_BUFFER: RefCell<Option<IPCBufferWrapper>> = RefCell::new(None);


struct IPCBufferWrapper {
    buffer: &'static mut IPCBuffer
}

pub fn ipc_buffer_init(thread: usize ,vaddr: usize) -> JustResult {
    match mork_alloc_object(CNodeSlot::CapInitThread as usize, ObjectType::Frame4K) {
        Ok(frame_handler) => {
            mork_user_log!(debug, "success to allocate memory: {:?}", frame_handler);
            let vm_rights = VMRights::R | VMRights::W;
            match mork_map_frame_anyway(
                CNodeSlot::CapInitThread as usize,
                CNodeSlot::CapInitVSpace as usize,
                frame_handler,
                vaddr,
                vm_rights,
                &mut None
            ) {
                Ok(_) => {
                    match mork_thread_set_ipc_buffer(thread, frame_handler) {
                        Ok(_) => {
                            mork_user_log!(debug, "success set ipc buffer: {:#x}", vaddr);
                            *IPC_BUFFER.borrow_mut() = Some(
                                IPCBufferWrapper {
                                    buffer: unsafe { &mut *(vaddr as *mut IPCBuffer) }
                                }
                            );
                            Ok(())
                        }
                        Err(resp) => {
                            mork_user_log!(error, "set ipc buffer failed: {:?}!", resp);
                            Err(())
                        }
                    }
                }
                Err(resp) => {
                    mork_user_log!(error, "fail to map frame: {:?}", resp);
                    Err(())
                }
            }
        }
        Err(resp) => {
            mork_user_log!(error, "fail to allocate memory: {:?}", resp);
            Err(())
        }
    }
}

pub fn with_ipc_buffer<F, T>(f: F) -> T
where F: FnOnce(&IPCBuffer) -> T {
    let borrow = IPC_BUFFER.borrow();
    let wrapper = borrow.as_ref().expect("IPC buffer not initialized");
    f(&wrapper.buffer)
}

pub fn with_ipc_buffer_mut<F, T>(f: F) -> T
where F: FnOnce(&mut IPCBuffer) -> T {
    let mut borrow = IPC_BUFFER.borrow_mut(); // 可变借用 RefCell
    let wrapper = borrow.as_mut().expect("IPC buffer not initialized");
    f(&mut wrapper.buffer)
}