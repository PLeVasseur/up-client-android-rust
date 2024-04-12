use crate::{UPClientAndroid, UPCLIENTANDROID_TAG};
use async_std::task;
use async_trait::async_trait;
use jni::objects::{JByteArray, JClass, JObject, JValue};
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use lazy_static::lazy_static;
use log::{error, trace};
use protobuf::Message;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use up_rust::{ComparableListener, UCode, UListener, UMessage, UStatus, UTransport, UUri};

const UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG: &str = "register_listener:";

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

        trace!(
            "{}:{} Entered register_listener",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // Get JNIEnv for the current thread
        let mut env = self
            .vm
            .attach_current_thread()
            .expect("Failed to attach current thread");

        trace!(
            "{}:{} Got JNIEnv",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // TODO: Add logic for making sure we only add listener once
        let hash = store_listener(topic.clone(), listener);

        let Ok(listener_class) =
            env.find_class("org/eclipse/uprotocol/streamer/service/UListenerNativeBridge")
        else {
            error!("Failed to find UListenerNativeBridge class");
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
            return Err(UStatus::fail_with_code(UCode::INTERNAL, "Class not found"));
        };
        trace!(
            "{}:{} Got UListenerNativeBridge class",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // Check if an exception occurred
        if env.exception_check().unwrap() {
            trace!(
                "{}:{} Hit exception constructing trying to find UListenerNativeBridge class",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            // Optionally log or describe the exception
            env.exception_describe().unwrap(); // This will print the exception details to the console
            env.exception_clear().unwrap(); // Clears the exception so that JNI calls can continue

            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Exception was thrown",
            )); // Replace UStatus::Error with appropriate error handling
        }

        trace!(
            "{}:{} Got UListenerNativeBridge class",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        let Ok(listener_obj) =
            env.new_object(listener_class, "(J)V", &[JValue::Long(hash as jlong)])
        else {
            error!("Failed to create a new instance of UListenerBridge class");
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
            return Err(UStatus::fail_with_code(UCode::INTERNAL, "Class not found"));
        };

        trace!(
            "{}:{} Constructed UListenerNativeBridge object",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // Check if an exception occurred
        if env.exception_check().unwrap() {
            trace!(
                "{}:{} Hit exception constructing UListenerNativeBridge object",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            // Optionally log or describe the exception
            env.exception_describe().unwrap(); // This will print the exception details to the console
            env.exception_clear().unwrap(); // Clears the exception so that JNI calls can continue

            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Exception was thrown",
            )); // Replace UStatus::Error with appropriate error handling
        }

        let up_client_ref = self.up_client.as_obj(); // Convert GlobalRef to JObject
        trace!(
            "{}:{} Converted GlobalRef to JObject",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        let Ok(uuri_bytes) = topic.write_to_bytes() else {
            error!("Failed to serialize UUri to bytes");
            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Failed to obtain UUri bytes",
            )); // Replace UStatus::Error with appropriate error handling
        };
        trace!(
            "{}:{} Turned UUri into byte vec",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );
        let byte_array = env
            .byte_array_from_slice(&uuri_bytes)
            .expect("Couldn't create jbyteArray from Rust Vec<u8>");
        trace!(
            "{}:{} Turned byte vec into JByteArray",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        let native_bridge_class = env
            .find_class("org/eclipse/uprotocol/streamer/service/NativeBridge")
            .expect("Couldn't find the Helper class");
        trace!(
            "{}:{} Found NativeBridge class",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // let Ok(foo) = JObject::from(byte_array) else {
        //     trace!(
        //         "{}:{} Failed when converting to JObject from JByteArray",
        //         UPCLIENTANDROID_TAG,
        //         UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        //     );
        //     env.exception_describe().unwrap();
        //     env.exception_clear().unwrap();
        //     return Err(UStatus::fail_with_code(
        //         UCode::INTERNAL,
        //         "Failed when calling deserializeToUUri",
        //     )); // Replace UStatus::Error with appropriate error handling
        // };

        let jvalue_byte_array = JValue::Object(&*byte_array);

        // let foo = JObject::from(byte_array);
        // let foo_val = JValue::try_from(foo);
        // let foo_val = foo_val.unwrap();
        let Ok(uuri_obj) = env.call_static_method(
            native_bridge_class,
            "deserializeToUUri",
            "([B)Lorg/eclipse/uprotocol/streamer/service/UUri;",
            &[jvalue_byte_array],
        ) else {
            trace!(
                "{}:{} Failed when calling deserializeToUUri",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Failed when calling deserializeToUUri",
            )); // Replace UStatus::Error with appropriate error handling
        };

        let Ok(uuri_obj) = uuri_obj.l() else {
            trace!(
                "{}:{} Failed when converting uuri_obj to a JObject",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            env.exception_describe().unwrap();
            env.exception_clear().unwrap();
            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Failed when converting uuri_obj to a JObject",
            )); // Replace UStatus::Error with appropriate error handling
        };

        trace!(
            "{}:{} Turned serialized JByteArray into UUri JObject",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // Check if an exception occurred
        if env.exception_check().unwrap() {
            trace!(
                "{}:{} Exception occured while turning JByteArray into UUri JObject",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            // Optionally log or describe the exception
            env.exception_describe().unwrap(); // This will print the exception details to the console
            env.exception_clear().unwrap(); // Clears the exception so that JNI calls can continue

            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Exception was thrown",
            )); // Replace UStatus::Error with appropriate error handling
        }

        let args = [JValue::Object(&uuri_obj), JValue::Object(&listener_obj)];
        trace!(
            "{}:{} Formed arguments to the Java UPClient's registerListener",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        env.call_method(
            up_client_ref,
            "registerListener",
            register_listener_signature(),
            &args,
        )
        .expect("Unable to call UPClient.registerListner()");
        trace!(
            "{}:{} Called registerListener on the Java UPClient",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        // Check if an exception occurred
        if env.exception_check().unwrap() {
            trace!(
                "{}:{} Exception occurred while calling registerListener on the Java UPClient",
                UPCLIENTANDROID_TAG,
                UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
            );
            // Optionally log or describe the exception
            env.exception_describe().unwrap(); // This will print the exception details to the console
            env.exception_clear().unwrap(); // Clears the exception so that JNI calls can continue

            return Err(UStatus::fail_with_code(
                UCode::INTERNAL,
                "Exception was thrown",
            )); // Replace UStatus::Error with appropriate error handling
        }

        trace!(
            "{}:{} Reached bottom of function",
            UPCLIENTANDROID_TAG,
            UPANDROIDCLIENT_FN_REGISTER_LISTENER_TAG
        );

        Ok(())
    }

    async fn unregister_listener(
        &self,
        topic: UUri,
        listener: Arc<dyn UListener>,
    ) -> Result<(), UStatus> {
        todo!()
    }
}
