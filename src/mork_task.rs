use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use mork_common::types::ResultWithErr;
use crate::riscv::syscall::call_with_mrs;

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