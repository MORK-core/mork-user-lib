use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;
use mork_common::constants::{CNodeSlot, ObjectType};
use mork_common::hal::{UserContext, UserContextTrait};
use mork_common::mork_user_log;
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use mork_common::types::{ResultWithErr, ResultWithValue};
use crate::hal::call_with_mrs;
use crate::mork_ipc_buffer::{with_ipc_buffer, with_ipc_buffer_mut};

pub fn mork_alloc_object(task: usize, obj_type: ObjectType) -> Result<usize, ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::CNodeAlloc, 0, 0, 1
    );
    let mut mr0 = obj_type as usize;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        task,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(mr0)
    }
}

pub fn mork_delete_object(task: usize, object: usize) -> Result<usize, ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::CNodeDelete, 0, 0, 1
    );
    let mut mr0 = object;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        task,
        message_info,
        &mut mr0,
        &mut mr1,
        &mut mr2,
        &mut mr3,
    );

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(mr0)
    }
}

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

pub fn create_thread(function: usize) -> ResultWithValue<usize> {
    let mut success = true;
    const THREAD_STACK_SIZE: usize = 4096 * 16;
    const STACK_ALIGN: usize = 4096;
    let layout = Layout::from_size_align(THREAD_STACK_SIZE, STACK_ALIGN).unwrap();
    let sp = unsafe { alloc_zeroed(layout) };
    if sp.is_null() {
        mork_user_log!(warn, "fail to alloc stack space");
        return Err(());
    }
    match mork_alloc_object(CNodeSlot::CapInitThread as usize, ObjectType::Thread) {
        Ok(new_tcb) => {
            let mut user_context = UserContext::new();
            if let Err(resp) = mork_thread_read_registers(CNodeSlot::CapInitThread as usize, &mut user_context) {
                mork_user_log!(warn, "fail to read registers: {:?}", resp);
                return Err(());
            }
            user_context.set_next_ip(function);
            user_context.set_sp(sp as usize);
            if let Err(resp) = mork_thread_write_registers(new_tcb, &user_context) {
                mork_user_log!(warn, "fail to write registers: {:?}", resp);
                success = false;
            }
            if success {
                if let Err(resp) = mork_thread_set_space(
                    new_tcb,
                    CNodeSlot::CapInitVSpace as usize,
                ) {
                    mork_user_log!(warn, "fail to set space: {:?}", resp);
                    success = false;
                }
            }
            if !success {
                unsafe {
                    dealloc(sp, layout);
                }
                todo!("free new_tcb");
            }

            Ok(new_tcb)
        }
        Err(resp) => {
            mork_user_log!(warn, "fail to alloc tcb object: {:?}", resp);
            Err(())
        }
    }
}