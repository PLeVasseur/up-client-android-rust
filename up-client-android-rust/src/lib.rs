use aidl_rust_codegen::binder_impls::IUBus::IUBus;
use aidl_rust_codegen::binder_impls::IUListener::{BnUListener, IUListener};
use aidl_rust_codegen::parcelable_stubs::{ParcelableUMessage, ParcelableUUri};
use binder::binder_impl::Binder;
use binder::{BinderFeatures, Interface, SpIBinder, Strong};
use protobuf::EnumOrUnknown;
use std::sync::Arc;
use up_rust::uprotocol::{UCode, UEntity, UStatus, UUri};

pub mod transport;
pub mod transport_builder;

pub struct MessageListener;

impl Interface for MessageListener {}

impl IUListener for MessageListener {
    fn onReceive(&self, event: &ParcelableUMessage) -> binder::Result<()> {
        println!("received ParcelableUMessage: {:?}", event);
        Ok(())
    }
}

pub struct UpClientAndroid {
    ubus: Arc<Strong<dyn IUBus>>,
    token: SpIBinder,
    package: String,
    entity: UEntity,
}

impl UpClientAndroid {
    pub fn new(ubus: Arc<Strong<dyn IUBus>>, package: &str, entity: &UEntity) -> Self {
        Self {
            ubus: ubus.clone(),
            token: Binder::new(()).as_binder(),
            package: package.to_string(),
            entity: entity.clone(),
        }
    }

    pub async fn connect(&self) -> UStatus {
        let message_listener = MessageListener;
        let message_listener_binder =
            BnUListener::new_binder(message_listener, BinderFeatures::default());

        let res = self.ubus.registerClient(
            &self.package,
            &self.entity.clone().into(),
            &self.token,
            0,
            &message_listener_binder,
        );

        match res {
            Ok(ps) => ps.as_ref().clone(),
            Err(_s) => UStatus {
                code: EnumOrUnknown::from(UCode::UNKNOWN),
                message: Some("Stubbed, unknown error".to_string()),
                details: vec![],
                ..Default::default()
            },
        }
    }

    pub async fn enable_dispatching(&self, uuri: UUri) -> UStatus {
        let res = self
            .ubus
            .enableDispatching(&ParcelableUUri::from(uuri), 0, &self.token);

        match res {
            Ok(ps) => ps.as_ref().clone(),
            Err(_s) => UStatus {
                code: EnumOrUnknown::from(UCode::UNKNOWN),
                message: Some("Stubbed, unknown error".to_string()),
                details: vec![],
                ..Default::default()
            },
        }
    }

    pub async fn disable_dispatching(&self, uuri: UUri) -> UStatus {
        let res = self
            .ubus
            .disableDispatching(&ParcelableUUri::from(uuri), 0, &self.token);

        match res {
            Ok(ps) => ps.as_ref().clone(),
            Err(_s) => UStatus {
                code: EnumOrUnknown::from(UCode::UNKNOWN),
                message: Some("Stubbed, unknown error".to_string()),
                details: vec![],
                ..Default::default()
            },
        }
    }
}
