use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;
use mork_common::constants::{CNodeSlot, ObjectType};
use mork_common::hal::{UserContext, UserContextTrait};
use mork_common::mork_user_log;
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use mork_common::types::{ResultWithErr, ResultWithValue};
use crate::hal::call_with_mrs;
use crate::mork_cspace::{mork_alloc_object, mork_cspace_copy, mork_delete_object};
use crate::mork_ipc::mork_notification_wait;
use crate::mork_ipc_buffer::{with_ipc_buffer, with_ipc_buffer_mut};
pub fn mork_thread_suspend(thread: usize) -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSuspend, 0, 0, 0
    );
    let mut mr0 = 0;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_thread_resume(thread: usize) -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBResume, 0, 0, 0
    );
    let mut mr0 = 0;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_thread_set_space(thread: usize, vspace: usize)
    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetSpace, 0, 0, 1
    );
    let mut mr0 = vspace;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_thread_set_ipc_buffer(thread: usize, frame: usize)
    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetIPCBuffer, 0, 0, 1
    );
    let mut mr0 = frame;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_thread_set_tls_base(thread: usize, tls_base: usize)
                                  -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetTLSBase, 0, 0, 1
    );
    let mut mr0 = tls_base;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_thread_read_registers(thread: usize, context: &mut UserContext)
                                    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBReadRegisters, 0, 0, 0
    );
    let mut mr0 = 0;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );
    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        with_ipc_buffer(|buffer| {
            *context = *UserContext::from_ipc_buffer(buffer);
        });
        Ok(())
    }
}

pub fn mork_thread_write_registers(thread: usize, context: &UserContext)
                                    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBWriteRegisters, 0, 0, 0
    );

    with_ipc_buffer_mut(|buffer| {
        buffer.copy_from_user_context(context);
    });

    let mut mr0 = 0;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        thread,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );
    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

#[derive(Default)]
pub struct ThreadControlBlock {
    parent_task: usize,
    task_slot: usize, 
    notification_slot: usize,
    sp: usize,
}

const THREAD_STACK_SIZE: usize = 4096 * 16;
const STACK_ALIGN: usize = 4096;

impl ThreadControlBlock {
    pub fn wait(&self) -> Result<usize, ResponseLabel> {
        mork_notification_wait(self.notification_slot)
    }

    pub fn resume(&self) -> ResultWithErr<ResponseLabel> {
        mork_thread_resume(self.task_slot)
    }

    pub fn free(self) -> ResultWithErr<ResponseLabel> {
        if self.task_slot != 0 {
            let _ = mork_delete_object(self.parent_task, self.task_slot)?;
        }
        if self.notification_slot != 0 {
            let _ = mork_delete_object(self.parent_task, self.notification_slot)?;
        }
        if self.sp != 0 {
            let layout = Layout::from_size_align(THREAD_STACK_SIZE, STACK_ALIGN).unwrap();
            unsafe {
                dealloc(self.sp as *mut _, layout);
            }
        }

        Ok(())
    }
}

pub fn create_thread(function: usize) -> ResultWithValue<ThreadControlBlock> {
    let mut success = true;
    let mut tcb = ThreadControlBlock::default();
    tcb.parent_task = CNodeSlot::CapInitThread as usize;
    let layout = Layout::from_size_align(THREAD_STACK_SIZE, STACK_ALIGN).unwrap();
    let sp = unsafe { alloc_zeroed(layout) };
    if sp.is_null() {
        mork_user_log!(warn, "fail to alloc stack space");
        return Err(());
    }
    tcb.sp = sp as usize;

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
                if let Err(resp) = mork_thread_read_registers(CNodeSlot::CapInitThread as usize, &mut user_context) {
                    mork_user_log!(warn, "fail to read registers: {:?}", resp);
                    success = false;
                }
            }
            
            user_context.set_next_ip(function);
            user_context.set_sp(sp as usize + THREAD_STACK_SIZE);
            if success {
                if let Err(resp) = mork_thread_write_registers(new_task, &user_context) {
                    mork_user_log!(warn, "fail to write registers: {:?}", resp);
                    success = false;
                }
            }
            if success {
                if let Err(resp) = mork_thread_set_space(
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