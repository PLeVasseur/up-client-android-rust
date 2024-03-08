use async_trait::async_trait;
use up_rust::transport::datamodel::UTransport;
use up_rust::uprotocol::{UMessage, UStatus, UUri};
use crate::UpClientAndroid;

#[async_trait]
impl UTransport for UpClientAndroid {
    #[allow(dead_code)]
    async fn send(&self, _message: UMessage) -> Result<(), UStatus> {
        todo!()
    }

    #[allow(dead_code)]
    async fn receive(&self, _topic: UUri) -> Result<UMessage, UStatus> {
        todo!()
    }

    #[allow(dead_code)]
    async fn register_listener(
        &self,
        _topic: UUri,
        _listener: Box<dyn Fn(Result<UMessage, UStatus>) + Send + Sync + 'static>,
    ) -> Result<String, UStatus> {
        todo!()
    }

    #[allow(dead_code)]
    async fn unregister_listener(&self, _topic: UUri, _listener: &str) -> Result<(), UStatus> {
        todo!()
    }
}