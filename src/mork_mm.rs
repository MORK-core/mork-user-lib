use alloc::vec::Vec;
use mork_common::constants::{ObjectType, PAGE_SIZE_NORMAL};
use mork_common::mork_user_log;
use mork_common::syscall::message_info::{InvocationLabel, MessageInfo, ResponseLabel};
use mork_common::types::{ResultWithErr, VMRights};
use mork_common::utils::alignas::is_aligned;
use crate::hal::call_with_mrs;
use crate::mork_cspace::mork_alloc_object;

pub fn mork_map_frame(vspace: usize, frame: usize, vaddr: usize, vm_rights: VMRights) -> Result<(), ResponseLabel> {
    if !is_aligned(vaddr, PAGE_SIZE_NORMAL) {
        return Err(ResponseLabel::InvalidParam);
    }
    let message_info = MessageInfo::new(
        InvocationLabel::PageMap, 0, 0, 4
    );
    let mut mr0 = frame;
    let mut mr1 = vaddr;
    let mut mr2 = vm_rights.bits() as usize;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        vspace,
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

pub fn mork_unmap_frame(vspace: usize, frame: usize) -> Result<(), ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::PageUnmap, 0, 0, 1
    );
    let mut mr0 = frame;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        vspace,
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

pub fn mork_map_page_table(vspace: usize, page_table: usize, vaddr: usize) -> Result<(), ResponseLabel> {
    if !is_aligned(vaddr, PAGE_SIZE_NORMAL) {
        return Err(ResponseLabel::InvalidParam);
    }
    let message_info = MessageInfo::new(
        InvocationLabel::PageTableMap, 0, 0, 2
    );
    let mut mr0 = page_table;
    let mut mr1 = vaddr;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        vspace,
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

pub fn mork_unmap_page_table(vspace: usize, page_table: usize) -> Result<(), ResponseLabel> {
    let message_info = MessageInfo::new(
        InvocationLabel::PageTableUnmap, 0, 0, 1
    );
    let mut mr0 = page_table;
    let mut mr1 = 0;
    let mut mr2 = 0;
    let mut mr3 = 0;
    let out_tag = call_with_mrs(
        vspace,
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

pub fn mork_map_frame_anyway(task: usize, vspace: usize, frame: usize, vaddr: usize, vm_rights: VMRights,
                            alloc_page_table: &mut Option<Vec<usize>>)
    -> ResultWithErr<ResponseLabel> {
    let mut push = |obj: usize| {
        if alloc_page_table.is_some() {
            alloc_page_table.as_mut().unwrap().push(obj);
        }
    };
    loop {
        match mork_map_frame(
            vspace,
            frame,
            vaddr,
            vm_rights,
        ) {
            Ok(_) => { return Ok(()); },
            Err(resp) => {
                if resp == ResponseLabel::PageTableMiss {
                    match mork_alloc_object(task, ObjectType::PageTable) {
                        Ok(page_table) => {
                            match mork_map_page_table(vspace, page_table, vaddr) {
                                Ok(_) => {
                                    push(page_table);
                                    continue;
                                }
                                Err(resp_inner) => {
                                    // todo: unmap page_table & dealloc
                                    return Err(resp_inner);
                                }
                            }
                        }
                        Err(alloc_resp) => {
                            // todo: unmap page_table & dealloc
                            return Err(alloc_resp);
                        }
                    }
                } else {
                    mork_user_log!(warn, "fail to mork_map_frame: ");
                    return Err(resp);
                }
            }
        }
    }
}