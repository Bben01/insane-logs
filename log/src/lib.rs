use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

// The LOGGER static holds a pointer to the global logger. It is protected by
// the STATE static which determines whether LOGGER has been initialized yet.
static mut LOGGER: &dyn Log = &NopLogger;

static STATE: AtomicUsize = AtomicUsize::new(0);

// There are three different states that we care about: the logger's
// uninitialized, the logger's initializing (set_logger's been called but
// LOGGER hasn't actually been set yet), or the logger's active.
// const UNINITIALIZED: usize = 0;
// const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;

pub enum Product {
    Stat,
    System,
    Other,
}

pub trait Producible {
    fn prod(&self) -> Vec<u8>;
}

impl<T: Serialize> Producible for T {
    fn prod(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}

pub trait Log: Send + Sync {
    fn publish(&self, r#type: Product, product: &dyn Producible);
}

/// Returns a reference to the logger.
///
/// If a logger has not been set, a no-op implementation is returned.
pub fn logger() -> &'static dyn Log {
    if STATE.load(Ordering::Acquire) != INITIALIZED {
        static NOP: NopLogger = NopLogger;
        &NOP
    } else {
        unsafe { LOGGER }
    }
}

struct NopLogger;

impl Log for NopLogger {
    fn publish(&self, _: Product, _: &dyn Producible) {}
}

#[macro_export]
macro_rules! stat {
    ($arg:tt) => {
        $crate::logger().publish($crate::Product::Stat, &$arg)
    };
}

#[macro_export]
macro_rules! system {
    ($arg:tt) => {
        $crate::logger().publish($crate::Product::System, &$arg)
    };
}

#[macro_export]
macro_rules! other {
    ($arg:tt) => {
        $crate::logger().publish($crate::Product::Other, &$arg)
    };
}
