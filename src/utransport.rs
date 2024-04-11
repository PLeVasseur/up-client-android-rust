use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use async_std::task;
use async_trait::async_trait;
use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::jstring;
use lazy_static::lazy_static;
use log::error;
use up_rust::{ComparableListener, UListener, UMessage, UStatus, UTransport, UUri};
use crate::UPClientAndroid;
use protobuf::Message;

lazy_static! {
    static ref LISTENERS: Mutex<HashSet<(UUri, ComparableListener)>> = Mutex::new(HashSet::new());
    static ref HASH_TO_LISTENER: Mutex<HashMap<u64, Arc<dyn UListener>>> = Mutex::new(HashMap::new());
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

    async fn register_listener(&self, topic: UUri, listener: Arc<dyn UListener>) -> Result<(), UStatus> {
        // here I want to call the held GlobalRef up_client's registerListener() method
        // and pass in a Java class implementing UListener
        //
        // that Java class implementing UListener would then be used to call the passed in
        // the Rust listener: Arc<dyn UListener>
        todo!()
    }

    async fn unregister_listener(&self, topic: UUri, listener: Arc<dyn UListener>) -> Result<(), UStatus> {
        todo!()
    }
}