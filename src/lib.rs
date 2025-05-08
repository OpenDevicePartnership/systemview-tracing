#![no_std]
#![allow(dead_code)]

#[cfg(feature = "tracing-enabled")]
mod tracing_impl {
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

    pub fn isr_enter() {
        rtos_trace::trace::isr_enter();
    }

    pub fn isr_exit() {
        rtos_trace::trace::isr_exit();
    }

    #[macro_export]
    macro_rules! trace_interrupt {
        ($($body:tt)*) => {
            {
                $crate::isr_enter();

                // Execute the actual interrupt handler code
                $($body)*

                $crate::isr_exit();
            }
        };
    }

    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            // Do nothing when systemview-tracing is enabled
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            // Do nothing when systemview-tracing is enabled
        };
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            // Do nothing when systemview-tracing is enabled
        };
    }

    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            // Do nothing when systemview-tracing is enabled
        };
    }

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            // Do nothing when systemview-tracing is enabled
        };
    }

    #[macro_export]
    macro_rules! panic {
        ($($arg:tt)*) => {
            // Do nothing for the log part, but still panic
            core::panic!($($arg)*);
        };
    }

    // Note: defmt-rtt cannot be used at the same time as SystemView RTT
    // Stub implementations for defmt
    #[no_mangle]
    pub extern "C" fn _defmt_write(_bytes: *const u8, _len: usize) {}

    #[no_mangle]
    pub extern "C" fn _defmt_acquire() -> isize {
        0
    }

    #[no_mangle]
    pub extern "C" fn _defmt_release(_token: isize) {}

    #[no_mangle]
    pub extern "C" fn _defmt_timestamp() -> u64 {
        0
    }
}

#[cfg(not(feature = "tracing-enabled"))]
mod tracing_impl {
    use defmt_rtt as _;
    pub fn init_tracing(_sysclock: u32) {}
    pub fn mark_trace(_marker: u32) {}

    #[macro_export]
    macro_rules! trace {
        ($($arg:tt)*) => {
            defmt::trace!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($($arg:tt)*) => {
            defmt::debug!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            defmt::info!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            defmt::warn!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => {
            defmt::error!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! panic {
        ($($arg:tt)*) => {
            defmt::panic!($($arg)*);
        };
    }
}

// Re-export the implementation functions at the crate root
pub use tracing_impl::*;
