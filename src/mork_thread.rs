use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;
use mork_common::constants::{CNodeSlot, ObjectType};
use mork_common::hal::{UserContext, UserContextTrait};
use mork_common::mork_user_log;
use mork_common::syscall::ipc_buffer::IPCBuffer;
use mork_common::syscall::message_info::ResponseLabel;
use mork_common::types::{ResultWithErr, ResultWithValue};
use crate::mork_cspace::{mork_alloc_object, mork_cspace_copy, mork_delete_object};
use crate::mork_ipc::mork_notification_wait;
use crate::mork_ipc_buffer::ipc_buffer_init_with_vaddr;
use crate::mork_task::{mork_task_read_registers, mork_task_resume, mork_task_set_space, mork_task_write_registers};

#[derive(Default)]
pub struct ThreadControlBlock {
    parent_task: usize,
    task_slot: usize,
    notification_slot: usize,
    ipc_buffer_vaddr: usize,
    sp: usize,
    tls: usize,
}

const TLS_SIZE: usize = 4096 * 1;
const THREAD_STACK_SIZE: usize = 4096 * 32;
const PAGE_ALIGN: usize = 4096;

unsafe fn thread_entry(thread_ptr: usize, ipc_buffer: usize) {
    ipc_buffer_init_with_vaddr(CNodeSlot::CapInitThread as usize, ipc_buffer).unwrap();
    let thread_exec: fn() = unsafe { core::mem::transmute(thread_ptr) };
    thread_exec();
}

impl ThreadControlBlock {
    pub fn wait(&self) -> Result<usize, ResponseLabel> {
        mork_notification_wait(self.notification_slot)
    }

    pub fn resume(&self) -> ResultWithErr<ResponseLabel> {
        mork_task_resume(self.task_slot)
    }

    pub fn free(self) -> ResultWithErr<ResponseLabel> {
        if self.task_slot != 0 {
            let _ = mork_delete_object(self.parent_task, self.task_slot)?;
        }
        if self.notification_slot != 0 {
            let _ = mork_delete_object(self.parent_task, self.notification_slot)?;
        }
        if self.ipc_buffer_vaddr != 0 {
            let layout = Layout::from_size_align(size_of::<IPCBuffer>(), PAGE_ALIGN).unwrap();
            unsafe {
                dealloc(self.sp as *mut _, layout);
            }
        }
        if self.sp != 0 {
            let layout = Layout::from_size_align(THREAD_STACK_SIZE, PAGE_ALIGN).unwrap();
            unsafe {
                dealloc(self.sp as *mut _, layout);
            }
        }
        if self.tls != 0 {
            let layout = Layout::from_size_align(TLS_SIZE, PAGE_ALIGN).unwrap();
            unsafe {
                dealloc(self.tls as *mut _, layout);
            }
        }

        Ok(())
    }
}

pub fn create_thread(function: usize) -> ResultWithValue<ThreadControlBlock> {
    let mut success = true;
    let mut tcb = ThreadControlBlock::default();
    tcb.parent_task = CNodeSlot::CapInitThread as usize;
    let layout = Layout::from_size_align(THREAD_STACK_SIZE, PAGE_ALIGN).unwrap();
    let sp = unsafe { alloc_zeroed(layout) };
    if sp.is_null() {
        mork_user_log!(warn, "fail to alloc stack space");
        return Err(());
    }
    tcb.sp = sp as usize;

    let tls_layout = Layout::from_size_align(TLS_SIZE, PAGE_ALIGN).unwrap();
    let tp = unsafe { alloc_zeroed(tls_layout) };
    if tp.is_null() {
        mork_user_log!(warn, "fail to alloc tls space");
        tcb.free().unwrap();
        return Err(());
    }
    tcb.tls = tp as usize;

    let ipc_buffer_layout = Layout::from_size_align(size_of::<IPCBuffer>(), PAGE_ALIGN).unwrap();
    let buffer_vaddr = unsafe { alloc_zeroed(ipc_buffer_layout) };
    if buffer_vaddr.is_null() {
        mork_user_log!(warn, "fail to alloc ipc_buffer_vaddr");
        tcb.free().unwrap();
        return Err(());
    }
    tcb.ipc_buffer_vaddr = buffer_vaddr as usize;

    match mork_alloc_object(CNodeSlot::CapInitThread as usize, ObjectType::Thread) {
        Ok(new_task) => {
            tcb.task_slot = new_task;
            match mork_alloc_object(CNodeSlot::CapInitThread as usize, ObjectType::Notification) {
                Ok(notification) =>  {
                    tcb.notification_slot = notification;
                    if let Ok(com_slot) = mork_cspace_copy(
                        CNodeSlot::CapInitThread as usize,
                        notification,
                        new_task,
                        CNodeSlot::CapParentCom as usize
                    ) {
                        assert_eq!(com_slot, CNodeSlot::CapParentCom as usize);
                    } else {
                        success = false;
                    }
                },
                Err(_) => success = false,
            }
            let mut user_context = UserContext::new();

            if success {
                if let Err(resp) = mork_task_read_registers(CNodeSlot::CapInitThread as usize, &mut user_context) {
                    mork_user_log!(warn, "fail to read registers: {:?}", resp);
                    success = false;
                }
            }

            user_context.set_arg0(function);
            user_context.set_arg1(tcb.ipc_buffer_vaddr);

            user_context.set_next_ip(thread_entry as usize);
            user_context.set_sp(sp as usize + THREAD_STACK_SIZE);
            // mork_user_log!(info, "sp: {:#x}", sp as usize + THREAD_STACK_SIZE);

            user_context.set_tls(tp as usize);
            // mork_user_log!(info, "tp: {:#x}", tp as usize);
            if success {
                if let Err(resp) = mork_task_write_registers(new_task, &user_context) {
                    mork_user_log!(warn, "fail to write registers: {:?}", resp);
                    success = false;
                }
            }
            if success {
                if let Err(resp) = mork_task_set_space(
                    new_task,
                    CNodeSlot::CapInitVSpace as usize,
                ) {
                    mork_user_log!(warn, "fail to set space: {:?}", resp);
                    success = false;
                }
            }
            if !success {
                tcb.free().unwrap();
                return Err(());
            }
            Ok(tcb)
        }
        Err(resp) => {
            mork_user_log!(warn, "fail to alloc tcb object: {:?}", resp);
            Err(())
        }
    }
}