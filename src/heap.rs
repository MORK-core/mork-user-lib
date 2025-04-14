use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use buddy_system_allocator::Heap;
use spin::mutex::Mutex;
use mork_common::constants::{CNodeSlot, ObjectType};
use mork_common::mork_user_log;
use mork_common::types::{JustResult, VMRights};
use crate::mork_mm::mork_map_frame_anyway;
use crate::mork_cspace::mork_alloc_object;

const HEAP_START: usize = 0x0000_4440_0000;
const HEAP_SIZE: usize = 1 << 21;

const ORDER: usize = 32;

static HEAP: Mutex<Heap<ORDER>> = Mutex::new(Heap::empty());

pub fn init() -> JustResult {
    match mork_alloc_object(CNodeSlot::CapInitThread as usize, ObjectType::Frame2M) {
        Ok(frame) => {
            if let Err(resp) = mork_map_frame_anyway(
                CNodeSlot::CapInitThread as usize,
                CNodeSlot::CapInitVSpace as usize,
                frame,
                HEAP_START,
                VMRights::R | VMRights::W,
                &mut None,
            ) {
                mork_user_log!(error, "fail to map heap memory, {:?}", resp);
                return Err(());
            }
        }
        Err(resp) => {
            mork_user_log!(error, "fail to alloc heap memory, {:?}", resp);
            return Err(());
        }
    }
    unsafe {
        HEAP.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

struct Global;

#[global_allocator]
static GLOBAL: Global = Global;

unsafe impl GlobalAlloc for Global {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.lock().alloc(layout).ok()
            .map_or(0 as *mut u8, |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.lock().dealloc(unsafe { NonNull::new_unchecked(ptr) }, layout);
        return;
    }
}