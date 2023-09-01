use log::{Log, Producible, Product};

struct DevNullLogger {}

impl Log for DevNullLogger {
    fn publish(&self, _: Product, product: &dyn Producible) {
        // serialise the product, then drop it.
        drop(product.prod());
    }
}