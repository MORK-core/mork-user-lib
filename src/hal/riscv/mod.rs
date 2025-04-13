pub mod syscall;

use core::arch::global_asm;

global_asm!(include_str!("start.asm"));