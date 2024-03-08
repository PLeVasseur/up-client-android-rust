use std::sync::Arc;
use binder::Strong;
use aidl_rust_codegen::binder_impls::IUBus::IUBus;

mod transport;
mod transport_builder;

struct UpClientAndroid {
    ubus: Arc<Strong<dyn IUBus>>
}

impl UpClientAndroid {
    fn new(ubus: Arc<Strong<dyn IUBus>>) -> Self {
        Self { ubus: ubus.clone() }
    }
}