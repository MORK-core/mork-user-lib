use core::arch::asm;
use mork_common::syscall::message_info::MessageInfo;

pub fn sys_send_recv(sys: isize, dest: usize, out_badge: &mut usize, info: usize, out_info: &mut usize,
                     in_out_mr0: &mut usize, in_out_mr1: &mut usize, in_out_mr2: &mut usize, in_out_mr3: &mut usize) {
    unsafe {
        asm!(
        "ecall",
        in("a0") dest,
        in("a1") info,
        in("a2") *in_out_mr0,
        in("a3") *in_out_mr1,
        in("a4") *in_out_mr2,
        in("a5") *in_out_mr3,
        in("a7") sys,
        );
    }
    unsafe {
        asm!("mv {}, a1", out(reg) *out_info);
        asm!("mv {}, a0", out(reg) *out_badge);
        asm!("mv {}, a2", out(reg) *in_out_mr0);
        asm!("mv {}, a3", out(reg) *in_out_mr1);
        asm!("mv {}, a4", out(reg) *in_out_mr2);
        asm!("mv {}, a5", out(reg) *in_out_mr3);
    }
}

pub fn sys_put_char(v8: u8) {
    sys_send_recv(mork_common::syscall::Syscall::SysDebugPutChar as isize, v8 as usize, &mut 0, 0,
                   &mut 0, &mut 0, &mut 0, &mut 0, &mut 0);
}

pub fn call_with_mrs(dest: usize, msg_info: MessageInfo, mr0: &mut usize, mr1: &mut usize, mr2: &mut usize, mr3: &mut usize)
                     -> MessageInfo {
    let mut local_dest = dest;
    let mut info = MessageInfo {words: [0; 1]};
    let mut msg0 = if msg_info.get_length() > 0 { *mr0 } else { 0 };
    let mut msg1 = if msg_info.get_length() > 1 { *mr1 } else { 0 };
    let mut msg2 = if msg_info.get_length() > 2 { *mr2 } else { 0 };
    let mut msg3 = if msg_info.get_length() > 3 { *mr3 } else { 0 };

    sys_send_recv(
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