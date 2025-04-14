use mork_common::syscall::message_info::ResponseLabel;
use mork_common::types::ResultWithErr;
use crate::hal::{sys_nb_send, sys_receive};

pub fn mork_notification_signal(notification: usize) -> ResultWithErr<ResponseLabel> {
    let out_tag = sys_nb_send(notification);

    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(())
    }
}

pub fn mork_notification_wait(notification: usize) -> Result<usize, ResponseLabel> {
    let mut badge = 0;
    let out_tag = sys_receive(notification, &mut badge);
    if out_tag.get_label() != ResponseLabel::Success as usize {
        Err(ResponseLabel::from_usize(out_tag.get_label()))
    } else {
        Ok(badge)
    }
}