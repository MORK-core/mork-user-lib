use mork_common::syscall::message_info::MessageInfo;
use crate::hal::riscv::syscall::riscv_sys_send_recv;

pub mod riscv;

pub fn sys_put_char(v8: u8) {
    riscv_sys_send_recv(mork_common::syscall::Syscall::SysDebugPutChar as isize, v8 as usize, &mut 0, 0,
                  &mut 0, &mut 0, &mut 0, &mut 0, &mut 0);
}

pub fn sys_shutdown() {
    riscv_sys_send_recv(mork_common::syscall::Syscall::SysDebugShutdown as isize, 0, &mut 0, 0,
                        &mut 0, &mut 0, &mut 0, &mut 0, &mut 0);
}

pub fn sys_nb_send(dest: usize) -> MessageInfo {
    let mut info = MessageInfo {words: [0; 1]};
    riscv_sys_send_recv(
        mork_common::syscall::Syscall::SysNBSend as isize,
        dest,
        &mut 0,
        info.words[0],
        &mut info.words[0],
        &mut 0,
        &mut 0,
        &mut 0,
        &mut 0
    );
    info
}

pub fn sys_receive(dest: usize, badge: &mut usize) -> MessageInfo {
    let mut info = MessageInfo {words: [0; 1]};
    riscv_sys_send_recv(
        mork_common::syscall::Syscall::SysRecv as isize,
        dest,
        badge,
        info.words[0],
        &mut info.words[0],
        &mut 0,
        &mut 0,
        &mut 0,
        &mut 0
    );
    info
}

pub fn call_with_mrs(dest: usize, msg_info: MessageInfo, mr0: &mut usize, mr1: &mut usize, mr2: &mut usize, mr3: &mut usize)
                     -> MessageInfo {
    let mut local_dest = dest;
    let mut info = MessageInfo {words: [0; 1]};
    let mut msg0 = if msg_info.get_length() > 0 { *mr0 } else { 0 };
    let mut msg1 = if msg_info.get_length() > 1 { *mr1 } else { 0 };
    let mut msg2 = if msg_info.get_length() > 2 { *mr2 } else { 0 };
    let mut msg3 = if msg_info.get_length() > 3 { *mr3 } else { 0 };

    riscv_sys_send_recv(
        mork_common::syscall::Syscall::Syscall as isize,
        dest,
        &mut local_dest,
        msg_info.words[0],
        &mut info.words[0],
        &mut msg0,
        &mut msg1,
        &mut msg2,
        &mut msg3
    );
    *mr0 = msg0;
    *mr1 = msg1;
    *mr2 = msg2;
    *mr3 = msg3;
    info
}