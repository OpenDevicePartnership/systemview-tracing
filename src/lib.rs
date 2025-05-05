#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};
use systemview_target::SystemView;

// SystemView instance
pub static SYSTEMVIEW: systemview_target::SystemView = systemview_target::SystemView::new();

// Set up the global trace
rtos_trace::global_trace! {SystemView}

// Define the trace info struct
pub struct TraceInfo();

static SYSCLOCK: AtomicU32 = AtomicU32::new(250_000_000);

impl rtos_trace::RtosTraceApplicationCallbacks for TraceInfo {
    fn system_description() {}
    fn sysclock() -> u32 {
        SYSCLOCK.load(Ordering::Relaxed)
    }
}

// Set up global application callbacks
rtos_trace::global_application_callbacks! {TraceInfo}

pub fn init_tracing(sysclock: u32) {
    SYSCLOCK.store(sysclock, Ordering::Relaxed);
    SYSTEMVIEW.init();
}

pub fn mark_trace(marker: u32) {
    rtos_trace::trace::marker(marker);
}

// Stub implementations for defmt
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _defmt_write(_bytes: *const u8, _len: usize) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _defmt_acquire() -> isize {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _defmt_release(_token: isize) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _defmt_timestamp() -> u64 {
    0
}
