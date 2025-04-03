use mork_common::constants::ObjectType;
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use crate::riscv::syscall::call_with_mrs;

pub fn mork_alloc_object(cspace: usize, obj_type: ObjectType) -> Result<usize, ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::AllocObject, 0, 0, 1
    );
    let mut mr0 = obj_type as usize;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        cspace,
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