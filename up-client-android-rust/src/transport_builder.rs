use std::sync::Arc;
use binder::Strong;
use up_rust::transport::datamodel::UTransport;
use ustreamer::utransport_builder::UTransportBuilder;
use aidl_rust_codegen::binder_impls::IUBus::IUBus;
use crate::UpClientAndroid;

pub struct AndroidTransportBuilder {
    pub ubus: Arc<Strong<dyn IUBus>>,
}

impl UTransportBuilder for AndroidTransportBuilder {
    fn build(&self) -> Box<dyn UTransport> {
        let ubus = Arc::clone(&self.ubus);

        let up_client: Box<dyn UTransport> = Box::new(UpClientAndroid::new(ubus));
        up_client
    }
}
