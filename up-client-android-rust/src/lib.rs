use std::sync::Arc;
use binder::Strong;
use aidl_rust_codegen::binder_impls::IUBus::IUBus;

pub mod transport;
pub mod transport_builder;

pub struct UpClientAndroid {
    ubus: Arc<Strong<dyn IUBus>>
}

impl UpClientAndroid {
    pub fn new(ubus: Arc<Strong<dyn IUBus>>) -> Self {
        Self { ubus: ubus.clone() }
    }
}