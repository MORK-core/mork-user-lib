use mork_common::constants::ObjectType;
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use crate::hal::call_with_mrs;

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

pub fn mork_cspace_copy(src_task: usize, src_cap: usize, dest_task: usize, dest_slot: usize) -> Result<usize, ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::CNodeCopy, 0, 0, 3
    );
    let mut mr0 = src_cap;
    let mut mr1 = dest_task;
    let mut mr2 = dest_slot;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        src_task,
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