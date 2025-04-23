use alloc::alloc::alloc_zeroed;
use core::alloc::Layout;
use mork_common::constants::CNodeSlot;
use mork_common::mork_user_log;
use mork_common::types::JustResult;
use crate::mork_task::mork_task_set_tls_base;

pub fn tls_init() -> JustResult {
    const TLS_SIZE: usize = 4096;
    let layout = Layout::from_size_align(TLS_SIZE, 4096).unwrap();
    let tls_addr = unsafe {
        alloc_zeroed(layout) as *const u8
    };
    if tls_addr.is_null() {
        mork_user_log!(error, "fail to allocate TLS address");
        return Err(());
    }
    if let Err(resp) =
        mork_task_set_tls_base(CNodeSlot::CapInitThread as usize, tls_addr as usize) {
        mork_user_log!(error, "fail to set TLS base address: {:?}", resp);
        return Err(());
    }
    Ok(())
}