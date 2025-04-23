use alloc::alloc::alloc_zeroed;
use core::alloc::Layout;
use core::cell::RefCell;
use mork_common::constants::PAGE_SIZE_NORMAL;
use mork_common::mork_user_log;
use mork_common::syscall::ipc_buffer::IPCBuffer;
use mork_common::types::JustResult;
use crate::mork_task::mork_task_set_ipc_buffer;

#[thread_local]
static IPC_BUFFER: RefCell<Option<IPCBufferWrapper>> = RefCell::new(None);


struct IPCBufferWrapper {
    buffer: &'static mut IPCBuffer
}

pub fn ipc_buffer_init(thread: usize) -> JustResult {
    let layout = Layout::from_size_align(size_of::<IPCBuffer>(), PAGE_SIZE_NORMAL).unwrap();
    let vaddr = unsafe {
        alloc_zeroed(layout)
    };
    if vaddr.is_null() {
        panic!("alloc_zeroed failed");
    }
    ipc_buffer_init_with_vaddr(thread, vaddr as usize)
}

pub fn ipc_buffer_init_with_vaddr(thread: usize, vaddr: usize) -> JustResult {
    match mork_task_set_ipc_buffer(thread, vaddr) {
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