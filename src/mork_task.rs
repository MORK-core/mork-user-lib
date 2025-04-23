use mork_common::hal::{UserContext, UserContextTrait};
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use mork_common::types::ResultWithErr;
use crate::hal::call_with_mrs;
use crate::mork_ipc_buffer::{with_ipc_buffer, with_ipc_buffer_mut};
pub fn mork_task_suspend(task: usize) -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSuspend, 0, 0, 0
    );
    let mut mr0 = 0;
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
        Ok(())
    }
}

pub fn mork_task_resume(task: usize) -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBResume, 0, 0, 0
    );
    let mut mr0 = 0;
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
        Ok(())
    }
}

pub fn mork_task_set_space(task: usize, vspace: usize)
    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetSpace, 0, 0, 1
    );
    let mut mr0 = vspace;
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
        Ok(())
    }
}

pub fn mork_task_set_ipc_buffer(task: usize, vaddr: usize)
    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetIPCBuffer, 0, 0, 1
    );
    let mut mr0 = vaddr;
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
        Ok(())
    }
}

pub fn mork_task_set_tls_base(task: usize, tls_base: usize)
                                  -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBSetTLSBase, 0, 0, 1
    );
    let mut mr0 = tls_base;
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
        Ok(())
    }
}

pub fn mork_task_read_registers(task: usize, context: &mut UserContext)
                                    -> ResultWithErr<ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::TCBReadRegisters, 0, 0, 0
    );
    let mut mr0 = 0;
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
        with_ipc_buffer(|buffer| {
            *context = *UserContext::from_ipc_buffer(buffer);
        });
        Ok(())
    }
}

pub fn mork_task_write_registers(task: usize, context: &UserContext)
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
        Ok(())
    }
}