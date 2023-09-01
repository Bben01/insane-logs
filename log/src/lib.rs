use serde::Serialize;

// The LOGGER static holds a pointer to the global logger. It is protected by
// the STATE static which determines whether LOGGER has been initialized yet.
static mut LOGGER: &dyn Log = &NopLogger;

pub enum Product {
    Stat,
    System,
    Other,
}

pub trait Productible {
    fn prod(self) -> Vec<u8>;
}

pub trait Log: Send + Sync {
    fn publish(&self, r#type: Product, product: &dyn Productible);
}

struct NopLogger;

impl Log for NopLogger {
    fn publish(&self, _: Product, _: &dyn Productible) {}
}

macro_rules! stat {
    () => {};
}