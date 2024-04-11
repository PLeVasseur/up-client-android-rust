use crate::UPClientAndroid;
use async_std::task;
use async_trait::async_trait;
use jni::objects::{JByteArray, JClass, JObject, JValue};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use lazy_static::lazy_static;
use log::error;
use protobuf::Message;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use up_rust::{ComparableListener, UListener, UMessage, UStatus, UTransport, UUri};

const CLASS_UURI: &str = "Lorg/eclipse/uprotocol/v1/UUri;";
const CLASS_ULISTENER: &str = "Lorg/eclipse/uprotocol/transport/UListener;";
const CLASS_USTATUS: &str = "Lorg/eclipse/uprotocol/v1/UStatus;";

fn register_listener_signature() -> String {
    format!("({}{}){}", CLASS_UURI, CLASS_ULISTENER, CLASS_USTATUS)
}

fn deserialize_uuri_signature() -> String {
    format!("([B){}", CLASS_UURI)
}

lazy_static! {
    static ref LISTENERS: Mutex<HashSet<(UUri, ComparableListener)>> = Mutex::new(HashSet::new());
    static ref HASH_TO_LISTENER: Mutex<HashMap<u64, Arc<dyn UListener>>> =
        Mutex::new(HashMap::new());
}

fn store_listener(uuri: UUri, listener: Arc<dyn UListener>) -> u64 {
    let mut listeners = LISTENERS.lock().unwrap();
    let mut hash_to_listener = HASH_TO_LISTENER.lock().unwrap();

    let hash_tuple = (uuri.clone(), ComparableListener::new(listener.clone()));
    let mut hasher = DefaultHasher::new();
    hash_tuple.hash(&mut hasher);
    let hash_value = hasher.finish();

    // Check if the hash is already in the map to handle collision or reinsertion scenarios.
    if !hash_to_listener.contains_key(&hash_value) {
        listeners.insert(hash_tuple);
        hash_to_listener.insert(hash_value, listener.clone());
    }

    hash_value
}

fn get_listener(hash: u64) -> Option<Arc<dyn UListener>> {
    let hash_to_listener = HASH_TO_LISTENER.lock().unwrap();
    hash_to_listener.get(&hash).cloned()
}

#[no_mangle]
pub extern "system" fn Java_org_eclipse_uprotocol_streamer_service_UListenerNativeBridge_onReceive<
    'local,
>(
    mut env: JNIEnv<'local>,
    // This is the class that owns our static method. It's not going to be used,
    // but still must be present to match the expected signature of a static
    // native method.
    class: JClass<'local>,
    listener_id: u64,
    message_bytes: JByteArray,
    // what tyep should the Java UMessage message be here in Rust?
) {
    let message_bytes_vec = env.convert_byte_array(message_bytes).unwrap();
    let message = UMessage::parse_from_bytes(&message_bytes_vec);

    let Ok(message) = message else {
        error!("Unable to convert to UMessage!");
        return;
    };

    let hash_to_listener = HASH_TO_LISTENER.lock().unwrap();
    let listener = hash_to_listener.get(&listener_id);
    if let Some(listener) = listener {
        // TODO: Consider if we want to block here or send over a channel to an existing task or
        //   something a little more performant than this
        task::block_on(listener.on_receive(message));
    }
}

#[async_trait]
impl UTransport for UPClientAndroid {
    async fn send(&self, message: UMessage) -> Result<(), UStatus> {
        todo!()
    }

    async fn receive(&self, topic: UUri) -> Result<UMessage, UStatus> {
        unimplemented!("UPClientAndroid listens, no need to call receive()")
    }

    async fn register_listener(
        &self,
        topic: UUri,
        listener: Arc<dyn UListener>,
    ) -> Result<(), UStatus> {
        // here I want to call the held GlobalRef up_client's registerListener() method
        // and pass in a Java class implementing UListener
        //
        // that Java class implementing UListener would then be used to call the passed in
        // the Rust listener: Arc<dyn UListener>

        // Get JNIEnv for the current thread
        let mut env = self
            .vm
            .attach_current_thread()
            .expect("Failed to attach current thread");

        // TODO: Add logic for making sure we only add listener once
        let hash = store_listener(topic.clone(), listener);

        // Create a new UListenerImpl object
        let listener_class = env
            .find_class("org/eclipse/uprotocol/streamer/service/UListenerNativeBridge")
            .expect("Failed to find UListenerImpl class");
        let listener_obj = env
            .new_object(listener_class, "()V", &[JValue::Long(hash as jlong)])
            .expect("Failed to create UListenerImpl object");

        let up_client_ref = self.up_client.as_obj(); // Convert GlobalRef to JObject

        let Ok(uuri_bytes) = topic.write_to_bytes() else {
            error!("Failed to serialize UUri to bytes");
            return Ok(());
        };
        let byte_array = env
            .byte_array_from_slice(&uuri_bytes)
            .expect("Couldn't create jbyteArray from Rust Vec<u8>");

        let native_bridge_class = env
            .find_class("org/eclipse/uprotocol/streamer/service/NativeBridge")
            .expect("Couldn't find the Helper class");
        let uuri_obj = env
            .call_static_method(
                native_bridge_class,
                "deserializeToUUri",
                "([B)Lorg/eclipse/uprotocol/streamer/service/UUri;",
                &[JValue::Object(&JObject::from(byte_array))],
            )
            .expect("Java method failed")
            .l()
            .expect("Expected a UUri object");
        let args = [JValue::Object(&uuri_obj), JValue::Object(&listener_obj)];

        env.call_method(
            up_client_ref,
            "registerListener",
            register_listener_signature(),
            &args,
        )
        .expect("Unable to call UPClient.registerListner()");

        todo!()
    }

    async fn unregister_listener(
        &self,
        topic: UUri,
        listener: Arc<dyn UListener>,
    ) -> Result<(), UStatus> {
        todo!()
    }
}
