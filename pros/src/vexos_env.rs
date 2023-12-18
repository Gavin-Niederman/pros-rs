use alloc::format;

use crate::println;
use core::{
    alloc::{GlobalAlloc, Layout},
    panic::PanicInfo,
};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("Panicked! {_info}");
    let panic_message = alloc::ffi::CString::new(format!("Panicked! {}", _info)).unwrap();
    unsafe {
        pros_sys::puts(panic_message.as_ptr());
    }
    let panicking_task = crate::task::current();
    // Make sure we eat up every cycle to stop execution
    panicking_task.set_priority(crate::task::TaskPriority::High);
    loop {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        unsafe {
            core::arch::arm::__nop();
        }
    }
}

struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        pros_sys::memalign(layout.align() as _, layout.size() as _) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        pros_sys::free(ptr as *mut core::ffi::c_void)
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
