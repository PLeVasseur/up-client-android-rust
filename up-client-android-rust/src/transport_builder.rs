use crate::UpClientAndroid;
use aidl_rust_codegen::binder_impls::IUBus::IUBus;
use binder::Strong;
use std::sync::Arc;
use up_rust::transport::datamodel::UTransport;
use up_rust::uprotocol::UEntity;
use ustreamer::utransport_builder::UTransportBuilder;

pub struct AndroidTransportBuilder {
    pub ubus: Arc<Strong<dyn IUBus>>,
    pub package: String,
    pub entity: UEntity,
}

impl UTransportBuilder for AndroidTransportBuilder {
    fn build(&self) -> Box<dyn UTransport> {
        let ubus = Arc::clone(&self.ubus);

        let up_client: Box<dyn UTransport> =
            Box::new(UpClientAndroid::new(ubus, &self.package, &self.entity));
        up_client
    }
}
